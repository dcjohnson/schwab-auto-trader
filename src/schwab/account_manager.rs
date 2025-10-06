use crate::{
    Error,
    config::TradingConfig,
    oauth::token::OauthManager,
    schwab::{client::SchwabClient, schemas::accounts_and_trading::accounts::SecuritiesAccount},
};
use tokio::{sync::watch, task::JoinSet};

pub struct AccountManager {
    trading_config: TradingConfig,
    om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    account_data: watch::Sender<AccountData>,
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
            js: JoinSet::new(),
        }
    }

    pub fn account_data_watcher(&mut self) -> watch::Receiver<AccountData> {
        self.account_data.subscribe()
    }

    pub async fn init(&mut self, timeout: tokio::time::Duration) -> Result<(), Error> {
        self.js.spawn({
            let om = self.om.clone();
            let account_data = self.account_data.clone();
            let trading_config = self.trading_config.clone();
            async move {
                let account_hash = 'outer: loop {
                    if let Some(Ok(token)) = om.lock().await.get_unexpired_token() {
                        for an in SchwabClient::new(token).get_account_numbers().await?.iter() {
                            if an.account_number == trading_config.account_number {
                                log::info!("Retrieved the account hash.");
                                break 'outer an.hash_value.clone();
                            }
                        }
                    }
                    tokio::time::sleep(timeout).await;
                };

                loop {
                    if let Some(Ok(token)) = om.lock().await.get_unexpired_token() {
                        let sc = SchwabClient::new(token);
                        if let Ok(account) = sc.get_account(&account_hash).await {
                            if let Some(securities_account) = account.securities_account {
                                account_data.send_modify(|ad| {
                                    ad.update(&securities_account);
                                });
                            }
                        }
                    }
                    tokio::time::sleep(timeout).await;
                }
            }
        });

        Ok(())
    }
}
