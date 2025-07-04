use oauth2::{TokenResponse, reqwest};

use json;
use schwab_auto_trader::oauth::{token, token_server, utils};
use serde::ser::Serialize;
use serde_json::Serializer as jsonSer;
use std::{env, fs};

const MARKET_DATA_ENDPOINT: &str = "https://api.schwabapi.com/marketdata/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // todo: Add command line parsing
    // todo: add nice logging
    // todo: add a nice login web page
    // todo: add graceful shutdown
    // todo: write recieved token to file and load/refresh it
    // if it exists.
    let args: Vec<String> = env::args().collect();

    let config = json::parse(&fs::read_to_string(&args[1]).unwrap()).unwrap();

    let oauth_client = utils::oauth_utils::new_oauth_basic_client(
        config["clientId"].to_string(),
        config["clientSecret"].to_string(),
        config["redirectAddress"].to_string(),
    )?;

    let tm = std::sync::Arc::new(std::sync::Mutex::new(token_server::TokenManager::new()));

    let f = tokio::spawn(token_server::run_server(8182, tm.clone()));

    let mut oauth_manager = token::OauthManager::new(tm.clone(), oauth_client);

    let (auth_url, mut token_receiver) = oauth_manager.auth_url().await;

    println!("Auth URL: {}", auth_url);

    let token_result = token_receiver.try_recv()?;
    println!("Got the token!: {:?}", token_result);

    let mut token: Vec<u8> = Vec::new();
    token_result.serialize(&mut jsonSer::pretty(&mut token))?;
    println!("YES!");

    println!(
        "{}",
        reqwest::Client::new()
            .get({
                let mut endpoint = MARKET_DATA_ENDPOINT.to_string();
                endpoint.push_str("/voo/quotes");
                endpoint
            })
            .bearer_auth(token_result.access_token().secret())
            .send()
            .await?
            .text()
            .await?
    );

    f.await??;

    Ok(())
}
