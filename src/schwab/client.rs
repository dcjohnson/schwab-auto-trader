use crate::{
    Error,
    oauth::token,
    schwab::{
        endpoints,
        schemas::accounts_and_trading::accounts::{
            AccountNumbers, AccountTypes, Accounts, Transaction, TransactionType, Transactions,
        },
    },
};
use chrono::{DateTime, Utc};
use oauth2::{TokenResponse, reqwest};
use serde::{de::Deserialize, ser::Serialize};

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

    pub async fn post(&self, endpoint: String, body: String) -> Result<String, Error> {
        Ok(self
            .client
            .post(endpoint)
            .body(body)
            .bearer_auth(self.auth_token.access_token().secret())
            .send()
            .await?
            .text()
            .await?)
    }

    pub async fn post_json<S: Serialize, R: for<'a> Deserialize<'a>>(
        &self,
        endpoint: String,
        s: S,
    ) -> Result<R, Error> {
        Ok(serde_json::from_str(
            &self.post(endpoint, serde_json::to_string(&s)?).await?,
        )?)
    }

    pub async fn get_json<T: for<'a> Deserialize<'a>>(&self, endpoint: String) -> Result<T, Error> {
        let json = self.get(endpoint.clone()).await?;
        log::debug!("get_json for endpoint: {} , json: {}", endpoint, json);
        Ok(serde_json::from_str(&json)?)
    }

    pub async fn get_account_numbers(&self) -> Result<AccountNumbers, Error> {
        self.get_json(endpoints::account_numbers()).await
    }

    pub async fn get_accounts(&self) -> Result<Accounts, Error> {
        self.get_json(endpoints::accounts()).await
    }

    pub async fn get_account(&self, account_hash: &str) -> Result<AccountTypes, Error> {
        self.get_json(endpoints::account(account_hash)).await
    }

    pub async fn get_quotes(&self, ticker: &str) -> Result<String, Error> {
        self.get(endpoints::ticker_quotes_data(ticker)).await
    }

    pub async fn get_orders(
        &self,
        account_hash: &str,
        from_entered_time: DateTime<Utc>,
        to_entered_time: DateTime<Utc>,
    ) -> Result<String, Error> {
        self.get(endpoints::orders(
            account_hash,
            from_entered_time,
            to_entered_time,
        ))
        .await
    }

    pub async fn get_transactions(
        &self,
        account_hash: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        transaction_type: TransactionType,
    ) -> Result<Transactions, Error> {
        self.get_json(endpoints::transactions(
            account_hash,
            start_date,
            end_date,
            transaction_type,
        ))
        .await
    }

    pub async fn get_transaction(
        &self,
        account_hash: &str,
        transaction_id: &str,
    ) -> Result<Transaction, Error> {
        self.get_json(endpoints::transaction(account_hash, transaction_id))
            .await
    }
}
