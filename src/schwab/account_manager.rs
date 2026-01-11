use crate::{
    Error,
    config::TradingConfig,
    oauth::token::OauthManager,
    schwab::{client::SchwabClient, math::two_decimals},
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
    _target_cash_balance: f64,
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

// this is the data which is displayed on the website.
#[derive(Default, Clone)]
pub struct AccountData {
    pub total_account_value: f64,
    pub total_cash_balance: f64,
    pub target_cash_balance: f64,
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
                let (s, _) = watch::channel({
                    let mut ad = AccountData::default();
                    ad.target_cash_balance = trading_config.target_cash_balance;
                    ad
                });
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
            _target_cash_balance: trading_config.target_cash_balance,
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
            let account_hash = internal_account_data.read().await.account_hash.clone();
            let client = SchwabClient::new(token);
            let account = client.get_account(account_hash.as_str()).await?;
            let now = chrono::Utc::now();

            println!(
                "ORDERS: {:#?}",
                client
                    .get_orders(account_hash.as_str(), now, now - chrono::Days::new(365))
                    .await?
            );

            if let Some(securities_account) = account.securities_account {
                // When the algorithm decides to make a purchase, it must hold all other potential
                // purches until after that set of orders are fulfilled and it has reflected in the
                // account balance.
                // I need to figure out when and how it affects the account balance.

                // Order example
                // {
                //  "orderType": "MARKET",
                //  "session": "NORMAL",
                //  "duration": "DAY",
                //  "orderStrategyType": "SINGLE",
                //  "orderLegCollection": [
                //   {
                //    "instruction": "BUY",
                //    "quantity": 15,
                //    "instrument": {
                //     "symbol": "XYZ",
                //     "assetType": "EQUITY"
                //    }
                //   }
                //  ]
                // }

                let iad = &mut internal_account_data.write().await;

                // Update the account data
                iad.account_data.total_account_value =
                    securities_account.initial_balances.account_value;
                iad.account_data.total_cash_balance =
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

    async fn has_unsettled_trades(
        om: &std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    ) -> Result<bool, Error> {
        match om.lock().await.get_unexpired_token() {
            Some(Ok(_token)) => Ok(true),
            _ => Err("Couldn't get valid oauth token".into()),
        }
    }

    async fn perform_trades(
        om: &std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
        _internal_account_data: &mut std::sync::Arc<tokio::sync::RwLock<InternalAccountData>>,
        _target_investments: &AccountInvestments,
    ) -> Result<(), Error> {
        if let Some(Ok(_token)) = om.lock().await.get_unexpired_token() {
            // Calculate allocations and perform trades sequentially. If the amount allocated is
            // not enough to buy that position, buy the alternate.
            // For each trade, it should determine if a position for that stock had been sold
            // recently so it can buy the alternate.
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
                log::info!("Initializing trading system");
                Self::initialize_account_hash(&om, &mut internal_account_data, &account_number)
                    .await?;

                log::info!("Begining update/trade loop");
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

                    if let Ok(false) = Self::has_unsettled_trades(&om).await {
                        if let Err(e) =
                            Self::perform_trades(&om, &mut internal_account_data, &investments)
                                .await
                        {
                            log::error!("Error when executing trades: '{}'", e);
                        }
                    }

                    tokio::time::sleep(timeout).await;
                }
            }
        });

        Ok(())
    }
}
