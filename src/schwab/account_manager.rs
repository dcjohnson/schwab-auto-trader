use crate::{
    Error,
    oauth::token::OauthManager,
    schwab::{client::SchwabClient, schemas::accounts_and_trading::accounts::SecuritiesAccount},
};
use tokio::{sync::watch, task::JoinSet};

pub struct AccountManager {
    account_number: String,
    account_hash: String,
    om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    account_data: watch::Sender<AccountData>,
    js: JoinSet<Result<(), Error>>,
}

#[derive(Default)]
pub struct AccountData {
    pub account_value: f64,
}

impl AccountData {
    pub fn update(&mut self, securities_account: &SecuritiesAccount) {
        self.account_value = securities_account.initial_balances.account_value;
    }
}

impl AccountManager {
    pub fn new(
        account_number: String,
        om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    ) -> Self {
        Self {
            account_number,
            account_hash: String::default(),
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

    pub async fn init(&mut self) -> Result<(), Error> {
        self.account_hash = 'outer: loop {
            if let Some(Ok(token)) = self.om.lock().await.get_unexpired_token() {
                for an in SchwabClient::new(token).get_account_numbers().await?.iter() {
                    if an.account_number == self.account_number {
                        log::info!("Retrieved the account hash.");
                        break 'outer an.hash_value.clone();
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        };

        self.js.spawn({
            let om = self.om.clone();
            let ah = self.account_hash.clone();
            let account_data = self.account_data.clone();
            async move {
                loop {
                    if let Some(Ok(token)) = om.lock().await.get_unexpired_token() {
                        let sc = SchwabClient::new(token);
                        if let Ok(account) = sc.get_account(&ah).await {
                            if let Some(securities_account) = account.securities_account {
                                account_data.send_modify(|ad| {
                                    ad.update(&securities_account);
                                });
                            }
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        });

        Ok(())
    }
}
