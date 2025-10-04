use crate::{
    Error,
    oauth::token,
    schwab::{endpoints, schemas},
};
use oauth2::{TokenResponse, reqwest};
use serde::de::Deserialize;

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

    pub async fn get(&self, endpoint: String) -> Result<String, Error> {
        Ok(self
            .client
            .get(endpoint)
            .bearer_auth(self.auth_token.access_token().secret())
            .send()
            .await?
            .text()
            .await?)
    }

    pub async fn get_json<T: for<'a> Deserialize<'a>>(&self, endpoint: String) -> Result<T, Error> {
        let json = self.get(endpoint.clone()).await?;
        log::debug!("get_json for endpoint: {} , json: {}", endpoint, json);
        Ok(serde_json::from_str(&json)?)
    }

    pub async fn get_account_numbers(
        &self,
    ) -> Result<schemas::accounts_and_trading::accounts::AccountNumbers, Error> {
        self.get_json(endpoints::account_numbers()).await
    }

    pub async fn get_accounts(
        &self,
    ) -> Result<schemas::accounts_and_trading::accounts::Accounts, Error> {
        self.get_json(endpoints::accounts()).await
    }

    pub async fn get_account(
        &self,
        account_hash_number: &str,
    ) -> Result<schemas::accounts_and_trading::accounts::AccountTypes, Error> {
        self.get_json(endpoints::account(account_hash_number)).await
    }

    pub async fn get_quotes(&self, ticker: &str) -> Result<String, Error> {
        self.get(endpoints::ticker_quotes_data(ticker)).await
    }
}
