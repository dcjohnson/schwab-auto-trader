use oauth2::basic::BasicClient;
use oauth2::reqwest;

use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl, TokenResponse,
};

use json;
use schwab_auto_trader::oauth::token_server;
use serde::ser::Serialize;
use serde_json::Serializer as jsonSer;
use std::{env, fs};
// use url::Url;

const MARKET_DATA_ENDPOINT: &str = "https://api.schwabapi.com/marketdata/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // todo: Add command line parsing
    // todo: add nice logging
    // todo: add a nice login web page
    // todo: add graceful shutdown
    // move client stuff to a nice module
    // todo: write recieved token to file and load/refresh it
    // if it exists.
    let args: Vec<String> = env::args().collect();

    let config = json::parse(&fs::read_to_string(&args[1]).unwrap()).unwrap();

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
            .bearer_auth(token_result.access_token().secret(),/*{
                let s = String::from_utf8(token)?;
                println!("{}", s);
                s
            }*/)
            .send()
            .await?
            .text()
            .await?
    );

    f.await??;

    Ok(())
}
