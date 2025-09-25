use clap::Parser;
use json;
use schwab_auto_trader::{
    Error,
    oauth::{token, token_storage, utils},
    server::server,
};
use std::fs;
use tokio::signal::{
    ctrl_c,
    unix::{SignalKind, signal},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config_file_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // todo: add a nice login web page

    env_logger::init();

    let args = Args::parse();
    let config = json::parse(&fs::read_to_string(&args.config_file_path).unwrap()).unwrap();
    let cancellation_token = tokio_util::sync::CancellationToken::new();

    let om = std::sync::Arc::new(tokio::sync::Mutex::new(token::OauthManager::new(
        utils::oauth_utils::new_oauth_basic_client(
            config["clientId"].to_string(),
            config["clientSecret"].to_string(),
            config["redirectAddress"].to_string(),
        )?,
        token_storage::TokenStorage::load(config["tokenFilePath"].to_string())?,
    )));

    token::OauthManager::spawn_token_receiver(om.clone(), core::time::Duration::from_millis(500))
        .await;
    token::OauthManager::spawn_token_refresher(om.clone(), core::time::Duration::from_secs(60))
        .await;

    let jh = tokio::spawn(server::run_server(
        config["bindAddress"].to_string().parse()?,
        om,
        cancellation_token.clone(),
        config["certPath"].to_string(),
        config["keyPath"].to_string(),
    ));

    let mut quit_signal = signal(SignalKind::quit())?;
    let mut terminate_signal = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = ctrl_c() => {},
        _ = quit_signal.recv() => {},
        _ = terminate_signal.recv() => {},
    };

    cancellation_token.cancel();

    jh.await??;

    Ok(())
}
