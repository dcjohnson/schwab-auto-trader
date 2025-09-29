use crate::{Error, oauth::token::OauthManager, schwab::client::SchwabClient};
use tokio::task::JoinSet;

pub struct AccountManager {
    om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>,
    js: JoinSet<Result<(), Error>>,
}

impl AccountManager {
    pub fn new(om: std::sync::Arc<tokio::sync::Mutex<OauthManager>>) -> Self {
        Self {
            om,
            js: JoinSet::new(),
        }
    }

    pub async fn init(&mut self) {
        self.js.spawn({
            let om = self.om.clone();
            async move {
                loop {
                    if let Some(Ok(token)) = om.lock().await.get_token() {
                        let sc = SchwabClient::new(token);

                        let account_numbers = sc.get_account_numbers().await?;

                        println!("Got the account numbers!: {:?}", account_numbers);

                        return Ok(());
                    }
                }
            }
        });
    }
}
