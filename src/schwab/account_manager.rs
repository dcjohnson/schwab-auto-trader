use crate::{
    Error,
    config::TradingConfig,
    oauth::token::OauthManager,
    schwab::{
        client::SchwabClient, math::two_decimals,
        schemas::accounts_and_trading::accounts::SecuritiesAccount,
    },
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
    _enable_tax_loss_harvesting: bool,
    // debt limit in dollars
    _margin_debt_limit: f64,
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
    internal_account_data: std::sync::Arc<tokio::sync::RwLock<InternalAccountData>>,
    js: JoinSet<Result<(), Error>>,
}

#[derive(Clone)]
pub struct Security {
    pub amount: f64,
    pub total_value: f64,
}

#[derive(Default, Clone)]
pub struct AccountData {
    pub total_account_value: f64,
    pub total_cash: f64,
    pub total_market_value: f64,
    pub total_day_change: f64,
    pub total_profit_loss: f64,
    pub investment_account_state_percent: Vec<InvestmentCollectionPercent>,
}

#[derive(Default, Clone)]
struct InternalAccountData {
    account_data: AccountData,
    account_hash: String,
    securities: HashMap<String, Security>,
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
            internal_account_data: std::sync::Arc::new(tokio::sync::RwLock::new(
                InternalAccountData::default(),
            )),
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
            _enable_tax_loss_harvesting: false,
            _margin_debt_limit: trading_config.margin_debt_limit,
        }
    }

    pub fn account_data_watcher(&mut self) -> watch::Receiver<AccountData> {
        self.account_data.subscribe()
    }

    async fn initialize_account_hash(
        om: &std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
        internal_account_data: &std::sync::Arc<tokio::sync::RwLock<InternalAccountData>>,
        account_number: &String,
    ) -> Result<(), Error> {
        internal_account_data.write().await.account_hash = 'outer: loop {
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
        internal_account_data: &mut std::sync::Arc<tokio::sync::RwLock<InternalAccountData>>,
        target_investments: &AccountInvestments,
    ) -> Result<(), Error> {
        if let Some(Ok(token)) = om.lock().await.get_unexpired_token() {
            let account = SchwabClient::new(token)
                .get_account(internal_account_data.read().await.account_hash.as_str())
                .await?;
            if let Some(securities_account) = account.securities_account {
                let iad = &mut internal_account_data.write().await;

                iad.account_data.total_account_value =
                    securities_account.initial_balances.account_value;
                iad.account_data.total_cash =
                    match securities_account.initial_balances.total_cash > 0.0 {
                        true => securities_account.initial_balances.total_cash,
                        false => securities_account.initial_balances.margin_balance,
                    };

                (
                    iad.account_data.total_market_value,
                    iad.account_data.total_day_change,
                    iad.account_data.total_profit_loss,
                    iad.securities,
                ) = securities_account.positions.iter().fold(
                    (0.0, 0.0, 0.0, HashMap::new()),
                    |(tmv, tdc, tpl, mut s), p| {
                        (
                            two_decimals(tmv + p.market_value),
                            two_decimals(tdc + p.current_day_profit_loss),
                            two_decimals(tpl + p.long_open_profit_loss),
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

                iad.account_data.investment_account_state_percent =
                    target_investments.priority_queue_investments.iter().fold(
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
                                        actual_investment: two_decimals(
                                            (equities.iter().fold(0.0, |t, e| {
                                                t + iad
                                                    .securities
                                                    .get(e)
                                                    .map(|s| s.total_value)
                                                    .unwrap_or(0.0)
                                            }) / iad.account_data.total_market_value)
                                                * 100.0,
                                        ),
                                    });
                                }
                                Amount::AmountValue(a) => log::info!(
                                    "Investment Group for ID: {}, Equities {:?}, Amount {}",
                                    group_name,
                                    equities,
                                    a
                                ),
                            }
                            v
                        },
                    );

                account_data.send_modify(|ad: &mut AccountData| ad.clone_from(&iad.account_data));
            }
        }
        Ok(())
    }

    pub async fn init(&mut self, timeout: tokio::time::Duration) -> Result<(), Error> {
        self.js.spawn({
            let om = self.om.clone();
            let account_data = self.account_data.clone();
            let mut internal_account_data = self.internal_account_data.clone();
            let account_number = self.account_number.clone();
            let investments = self.investments.clone();
            async move {
                Self::initialize_account_hash(&om, &mut internal_account_data, &account_number)
                    .await?;

                loop {
                    if let Err(e) = Self::update_account_data(
                        &om,
                        &account_data,
                        &mut internal_account_data,
                        &investments,
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
