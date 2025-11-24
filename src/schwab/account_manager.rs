use crate::{
    Error,
    config::TradingConfig,
    oauth::token::OauthManager,
    schwab::{
        client::SchwabClient,
        schemas::accounts_and_trading::accounts::{
            SecuritiesAccount, TransactionInstrument, TransactionType,
        },
    },
};
use chrono::{DateTime, Local, Utc};
use std::time::Duration;
use tokio::{sync::watch, task::JoinSet};

// In this Manager, we will want to represent a state we want to achieve/maintain.

enum Amount {
    PercentageValue(f64),
    AmountValue(u64),
}

struct AccountInvestments {
    priority_queue_investments: Vec<Investment>,
    enable_tax_loss_harvesting: bool,
    // debt limit in dollars
    margin_debt_limit: f64,
}

struct Investment {
    group_name: String,
    equities: Vec<String>,
    amount: Amount,
}

pub struct AccountManager {
    account_number: String,
    investments: AccountInvestments,
    om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    account_data: watch::Sender<AccountData>,
    account_hash: std::sync::Arc<tokio::sync::RwLock<String>>,
    js: JoinSet<Result<(), Error>>,
}

#[derive(Default)]
pub struct AccountData {
    pub account_value: f64,
    //add an avaialble cash field
    // add a total amount of available margin
    // a map of security groupings with financial information that can be broken down by group.
    // Each group is a collection of securities, each security needs to be broken down by lot.
}

impl AccountData {
    pub fn update(&mut self, securities_account: &SecuritiesAccount) {
        self.account_value = securities_account.initial_balances.account_value;
    }
}

impl AccountManager {
    pub fn new(
        trading_config: TradingConfig,
        om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    ) -> Self {
        Self {
            account_number: trading_config.account_number.clone(),
            investments: Self::account_config_from_trading_config(&trading_config),
            om,
            account_data: {
                let (s, _) = watch::channel(AccountData::default());
                s
            },
            account_hash: std::sync::Arc::new(tokio::sync::RwLock::new(String::default())),
            js: JoinSet::new(),
        }
    }

    fn account_config_from_trading_config(trading_config: &TradingConfig) -> AccountInvestments {
        AccountInvestments {
            priority_queue_investments: {
                let (groups, allocation_percent, allocation_amount) = trading_config.to_maps();

                allocation_percent.iter().fold(
                    allocation_amount
                        .iter()
                        .fold(Vec::new(), |mut v, (id, amount)| {
                            v.push(Investment {
                                group_name: id.clone(),
                                equities: groups[id].clone(),
                                amount: Amount::AmountValue(*amount),
                            });
                            v
                        }),
                    |mut v, (id, percent)| {
                        v.push(Investment {
                            group_name: id.clone(),
                            equities: groups[id].clone(),
                            amount: Amount::PercentageValue(*percent),
                        });
                        v
                    },
                )
            },
            enable_tax_loss_harvesting: false,
            margin_debt_limit: trading_config.margin_debt_limit,
        }
    }

    pub fn account_data_watcher(&mut self) -> watch::Receiver<AccountData> {
        self.account_data.subscribe()
    }

    pub async fn update_stock_basis(
        om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
        account_hash: String,
        oldest_transaction_date: DateTime<Utc>,
    ) -> Result<(), Error> {
        let sc = match om.lock().await.get_unexpired_token() {
            Some(Ok(token)) => Ok(SchwabClient::new(token)),
            Some(Err(e)) => Err(e),
            None => Err("no token".into()),
        }?;

        let transactions = sc
            .get_transactions(
                &account_hash,
                oldest_transaction_date,
                Local::now().to_utc(),
                TransactionType::Trade,
            )
            .await?;
        println!("ITOT");

        for t in transactions.iter() {
            for ti in t.transfer_items.iter() {
                if let TransactionInstrument::CollectiveInvestment { symbol, .. } = &ti.instrument {
                    if symbol == "VTI" {
                        println!("VTI: {:?}", t); //ti.instrument);
                    }
                }

                if let TransactionInstrument::TransactionEquity { .. } = &ti.instrument {
                    println!("equity instrument: {:?}", ti.instrument);
                }

                if let TransactionInstrument::TransactionOption { .. } = &ti.instrument {
                    println!("option instrument: {:?}", ti.instrument);
                }
            }
        }
        Ok(())
    }

    async fn initialize_account_hash(
        om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
        account_hash: std::sync::Arc<tokio::sync::RwLock<String>>,
        account_number: String,
    ) -> Result<(), Error> {
        *account_hash.write().await = 'outer: loop {
            if let Some(Ok(token)) = om.lock().await.get_unexpired_token() {
                for an in SchwabClient::new(token).get_account_numbers().await?.iter() {
                    if an.account_number == account_number {
                        log::info!("Retrieved the account hash.");
                        break 'outer an.hash_value.clone();
                    }
                }
                return Err("Account hash not found".into());
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        };

        Ok(())
    }

    async fn update_account_data(
        om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
        account_data: watch::Sender<AccountData>,
        account_hash: String,
    ) -> Result<(), Error> {
        if let Some(Ok(token)) = om.lock().await.get_unexpired_token() {
            let account = SchwabClient::new(token).get_account(&account_hash).await?;

            if let Some(securities_account) = account.securities_account {
                account_data.send_modify(|ad| {
                    ad.update(&securities_account);
                });
            }
        }
        Ok(())
    }

    async fn print_orders(
        om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
        account_hash: String,
    ) -> Result<(), Error> {
        if let Some(Ok(token)) = om.lock().await.get_unexpired_token() {
            log::info!(
                "ORDERS: {}",
                SchwabClient::new(token)
                    .get_orders(
                        &account_hash,
                        Utc::now() - std::time::Duration::from_secs(60 * 60 * 24 * 7 * 52),
                        Utc::now()
                    )
                    .await?
            );
        }
        Ok(())
    }

    pub async fn init(&mut self, timeout: tokio::time::Duration) -> Result<(), Error> {
        self.js.spawn({
            let om = self.om.clone();
            let account_data = self.account_data.clone();
            let account_hash = self.account_hash.clone();
            let account_number = self.account_number.clone();
            async move {
                Self::initialize_account_hash(om.clone(), account_hash.clone(), account_number)
                    .await?;

                loop {
                    let account_hash = (*account_hash.read().await).clone();
                    if let Err(e) = Self::update_account_data(
                        om.clone(),
                        account_data.clone(),
                        account_hash.clone(),
                    )
                    .await
                    {
                        log::error!("Error when updating account data: '{}'", e);
                    }

                    if let Err(e) = Self::print_orders(om.clone(), account_hash.clone()).await {
                        log::error!("Error printing orders: '{}'", e);
                    }

                    tokio::time::sleep(timeout).await;
                }
            }
        });

        Ok(())
    }
}
