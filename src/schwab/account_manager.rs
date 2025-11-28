use crate::{
    Error,
    config::TradingConfig,
    oauth::token::OauthManager,
    schwab::{client::SchwabClient, schemas::accounts_and_trading::accounts::SecuritiesAccount},
    server::web_resources::files::html::InvestmentCollectionPercent,
};
use std::collections::HashMap;
use tokio::{sync::watch, task::JoinSet};

// In this Manager, we will want to represent a state we want to achieve/maintain.

#[derive(Clone)]
enum Amount {
    PercentageValue(f64),
    AmountValue(u64),
}

#[derive(Clone)]
struct AccountInvestments {
    priority_queue_investments: Vec<Investment>,
    enable_tax_loss_harvesting: bool,
    // debt limit in dollars
    margin_debt_limit: f64,
}

#[derive(Clone)]
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

pub struct Security {
    pub amount: f64,
    pub total_value: f64,
}

#[derive(Default)]
pub struct AccountData {
    pub total_account_value: f64,
    pub total_cash: f64,
    pub total_market_value: f64,
    pub total_day_change: f64,
    pub total_profit_loss: f64,

    securities: HashMap<String, Security>,

    pub investment_account_state_percent: Vec<InvestmentCollectionPercent>,
}

impl AccountData {
    fn two_decimals(f: f64) -> f64 {
        (f * 100.0).round() / 100.0
    }

    pub fn update(
        &mut self,
        securities_account: &SecuritiesAccount,
        investment_targets: &AccountInvestments,
    ) {
        self.total_account_value = securities_account.initial_balances.account_value;
        self.total_cash = match securities_account.initial_balances.total_cash > 0.0 {
            true => securities_account.initial_balances.total_cash,
            false => securities_account.initial_balances.margin_balance,
        };

        (
            self.total_market_value,
            self.total_day_change,
            self.total_profit_loss,
            self.securities,
        ) = securities_account.positions.iter().fold(
            (0.0, 0.0, 0.0, HashMap::new()),
            |(tmv, tdc, tpl, mut s), p| {
                (
                    Self::two_decimals(tmv + p.market_value),
                    Self::two_decimals(tdc + p.current_day_profit_loss),
                    Self::two_decimals(tpl + p.long_open_profit_loss),
                    {
                        s.insert(
                            p.instrument.symbol(),
                            Security {
                                amount: p.long_quantity,
                                total_value: p.market_value,
                            },
                        );
                        s
                    },
                )
            },
        );

        self.investment_account_state_percent =
            investment_targets.priority_queue_investments.iter().fold(
                Vec::new(),
                |mut v,
                 Investment {
                     group_name,
                     equities,
                     amount,
                 }| {
                    match amount {
                        Amount::PercentageValue(p) => {
                            v.push(InvestmentCollectionPercent {
                                name: group_name.clone(),
                                target_investment: *p,
                                actual_investment: Self::two_decimals(
                                    (equities.iter().fold(0.0, |t, e| {
                                        t + self
                                            .securities
                                            .get(e)
                                            .map(|s| s.total_value)
                                            .unwrap_or(0.0)
                                    }) / self.total_market_value)
                                        * 100.0,
                                ),
                            });
                            v
                        }
                        Amount::AmountValue(_) => v,
                    }
                },
            );

        // next step is to calculate the percent allocations and add that to the map.
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

    async fn initialize_account_hash(
        om: &std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
        account_hash: &std::sync::Arc<tokio::sync::RwLock<String>>,
        account_number: &String,
    ) -> Result<(), Error> {
        *account_hash.write().await = 'outer: loop {
            if let Some(Ok(token)) = om.lock().await.get_unexpired_token() {
                for an in SchwabClient::new(token).get_account_numbers().await?.iter() {
                    if an.account_number == *account_number {
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
        om: &std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
        account_data: &watch::Sender<AccountData>,
        account_hash: &String,
        target_investments: &AccountInvestments,
    ) -> Result<(), Error> {
        if let Some(Ok(token)) = om.lock().await.get_unexpired_token() {
            let account = SchwabClient::new(token).get_account(&account_hash).await?;

            if let Some(securities_account) = account.securities_account {
                account_data.send_modify(|ad| {
                    ad.update(&securities_account, target_investments);
                });
            }
        }
        Ok(())
    }

    pub async fn init(&mut self, timeout: tokio::time::Duration) -> Result<(), Error> {
        self.js.spawn({
            let om = self.om.clone();
            let account_data = self.account_data.clone();
            let mut account_hash = self.account_hash.clone();
            let account_number = self.account_number.clone();
            let investments = self.investments.clone();
            async move {
                Self::initialize_account_hash(&om, &mut account_hash, &account_number).await?;

                loop {
                    let account_hash = (*account_hash.read().await).clone();
                    if let Err(e) =
                        Self::update_account_data(&om, &account_data, &account_hash, &investments)
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
