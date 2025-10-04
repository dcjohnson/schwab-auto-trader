use clap::Parser;
use json;
use schwab_auto_trader::{
    Error,
    oauth::{token, token_storage, utils},
    schwab::account_manager::AccountManager,
    server::server,
    config::Config, 
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
    let config = Config::load(&args.config_file_path)?; 
    let cancellation_token = tokio_util::sync::CancellationToken::new();

    let om = std::sync::Arc::new(tokio::sync::Mutex::new(token::OauthManager::new(
        utils::oauth_utils::new_oauth_basic_client(
            config.client_id,
            config.client_secret ,
            config.redirect_address,
        )?,
        token_storage::TokenStorage::load(config.token_file_path )?,
    )));

    let mut am = AccountManager::new(om.clone());
    am.init().await;

    token::OauthManager::spawn_token_receiver(om.clone(), core::time::Duration::from_millis(500))
        .await;
    token::OauthManager::spawn_token_refresher(om.clone(), core::time::Duration::from_secs(60))
        .await;

    let jh = tokio::spawn(server::run_server(
        config.bind_address.parse()?,
        om.clone(),
        cancellation_token.clone(),
        config.cert_path ,
        config.key_path ,
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
