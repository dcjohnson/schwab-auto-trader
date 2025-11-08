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

pub struct AccountManager {
    trading_config: TradingConfig,
    om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    account_data: watch::Sender<AccountData>,
    account_hash: String,
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
            trading_config,
            om,
            account_data: {
                let (s, _) = watch::channel(AccountData::default());
                s
            },
            account_hash: String::default(),
            js: JoinSet::new(),
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

    async fn initialize_account_hash(&mut self) -> Result<(), Error> {
        self.account_hash = 'outer: loop {
            if let Some(Ok(token)) = self.om.lock().await.get_unexpired_token() {
                for an in SchwabClient::new(token).get_account_numbers().await?.iter() {
                    if an.account_number == self.trading_config.account_number {
                        log::info!("Retrieved the account hash.");
                        break 'outer an.hash_value.clone();
                    }
                }
                return Err("Account hash not found".into());
            }
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

    pub async fn init(&mut self, timeout: tokio::time::Duration) -> Result<(), Error> {
        // initialize account hash
        self.initialize_account_hash().await?;

        // initialize stock basis
        Self::update_stock_basis(
            self.om.clone(),
            self.account_hash.clone(),
            self.trading_config.oldest_transaction_date,
        )
        .await?;

        self.js.spawn({
            let om = self.om.clone();
            let account_data = self.account_data.clone();
            let account_hash = self.account_hash.clone();
            async move {
                loop {
                    if let Err(e) = Self::update_account_data(
                        om.clone(),
                        account_data.clone(),
                        account_hash.clone(),
                    )
                    .await
                    {
                        log::error!("Error when updating account data: '{}'", e);
                    }
                    tokio::time::sleep(timeout).await;
                }
            }
        });

        Ok(())
    }
}
