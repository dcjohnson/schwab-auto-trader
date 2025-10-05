use crate::{Error, oauth::token::OauthManager, schwab::client::SchwabClient};
use tokio::task::JoinSet;

pub struct AccountManager {
    account_number: String,
    account_hash: String,
    om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    js: JoinSet<Result<(), Error>>,
}

#[derive(Default)]
pub struct AccountData {
    pub account_value: f64,
}

impl AccountData {
    pub fn update(&mut self, securities_account: &SecuritiesAccount) {
      self.account_value = securities_account.account_value;
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
            js: JoinSet::new(),
        }
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        self.account_hash = 'outer: loop {
            if let Some(Ok(token)) = self.om.lock().await.get_token() {
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
            async move {
                loop {
                    if let Some(Ok(token)) = om.lock().await.get_token() {
                        let sc = SchwabClient::new(token);
                        println!("Account: {:?}", sc.get_account(&ah).await?);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        });

        Ok(())
    }
}
