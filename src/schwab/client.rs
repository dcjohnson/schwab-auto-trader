use crate::{Error, oauth::token, schwab::endpoints};
use oauth2::{TokenResponse, reqwest};

pub struct SchwabClient {
    client: reqwest::Client,
    auth_token: token::OauthTokenResponse,
}

impl SchwabClient {
    pub fn new(auth_token: token::OauthTokenResponse) -> Self {
        Self {
            client: reqwest::Client::new(),
            auth_token: auth_token,
        }
    }

    pub async fn get(&mut self, endpoint: String) -> Result<String, Error> {
        Ok(self
            .client
            .get(endpoint)
            .bearer_auth(self.auth_token.access_token().secret())
            .send()
            .await?
            .text()
            .await?)
    }

    pub async fn get_accounts(&mut self) -> Result<String, Error> {
        self.get(endpoints::accounts()).await
    }

    pub async fn get_quotes(&mut self, ticker: &str) -> Result<String, Error> {
        self.get(endpoints::ticker_quotes_data(ticker)).await
    }
}
