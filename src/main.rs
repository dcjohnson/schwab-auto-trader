use oauth2::{TokenResponse, reqwest};

use json;
use schwab_auto_trader::{
    Error,
    oauth::{token, token_storage, utils},
    server::server,
};
use serde::{de::Deserialize, ser::Serialize};

use core::time as cTime;
use serde_json::{Deserializer as jsonDe, Serializer as jsonSer};
use std::{env, fs};
use tokio::{sync, time as tTime};

const MARKET_DATA_ENDPOINT: &str = "https://api.schwabapi.com/marketdata/v1";

async fn recieve_wait<T>(
    r: &mut sync::oneshot::Receiver<T>,
    d: cTime::Duration,
) -> Result<T, sync::oneshot::error::TryRecvError> {
    loop {
        match r.try_recv() {
            Ok(v) => break Ok(v),
            Err(sync::oneshot::error::TryRecvError::Empty) => tTime::sleep(d).await,
            e => break e,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // todo: Add command line parsing
    // todo: add nice logging
    // todo: add a nice login web page
    // todo: add graceful shutdown
    // todo: write recieved token to file and load/refresh it,
    // todo: implement token generation from within the server.
    // if it exists.
    let args: Vec<String> = env::args().collect();

    let config = json::parse(&fs::read_to_string(&args[1]).unwrap()).unwrap();

    let oauth_client = utils::oauth_utils::new_oauth_basic_client(
        config["clientId"].to_string(),
        config["clientSecret"].to_string(),
        config["redirectAddress"].to_string(),
    )?;

    let ts = std::sync::Arc::new(std::sync::Mutex::new(token_storage::TokenStorage::load(
        config["tokenFilePath"].to_string(),
    )?));
    let tm = std::sync::Arc::new(std::sync::Mutex::new(server::TokenManager::new()));

    let f = tokio::spawn(server::run_server(
        8182,
        tm.clone(),
        oauth_client.clone(),
        ts.clone(),
        config["clientId"].to_string(),
    ))
    .await??;

    Ok(())

    /*
    let mut oauth_manager = token::OauthManager::new(tm.clone(), oauth_client);
    oauth_manager
        .spawn_token_receiver(core::time::Duration::from_millis(500))
        .await;
    let (auth_url, mut token_receiver) = oauth_manager.auth_url().await;

    println!("Auth URL: {}", auth_url);

    let token_result = recieve_wait(&mut token_receiver, cTime::Duration::from_secs(1)).await?;
    println!("Got the token!: {:?}", token_result);

    let mut token: Vec<u8> = Vec::new();
    token_result.serialize(&mut jsonSer::pretty(&mut token))?;

    println!("TOKEN: {}\n", String::from_utf8(token)?);

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
    */
}
