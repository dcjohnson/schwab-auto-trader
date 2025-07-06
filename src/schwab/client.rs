use oauth2::{TokenResponse, reqwest};
use crate::{oauth::token, Error, schwab::endpoints};

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



    pub async fn get_quotes(&mut self, ticker: &str) -> Result<String, Error> {
        Ok(self.client
            .get(endpoints::ticker_quotes_data(ticker))
            .bearer_auth(self.auth_token.access_token().secret())
            .send()
            .await?
            .text()
            .await?)
    }

}
