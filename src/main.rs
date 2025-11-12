use clap::Parser;
use schwab_auto_trader::{
    Error,
    config::Config,
    oauth::{token, token_storage, utils},
    schwab::account_manager::AccountManager,
    server::server,
};
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

// Next steps; since we can't access the basis of the stocks lots, we can only do Tax Loss
// Harvesting by buying put options. There might be some more details in the accounts section but I
// am not sure what that is at the moment. 
//
// This means that I can start on implementing the purcashing algorithm based on percent allocation
// of certain equities. We can figure out the TLH later.

#[tokio::main]
async fn main() -> Result<(), Error> {
    // todo: add a nice login web page

    env_logger::init();

    let args = Args::parse();
    let config = Config::load(&args.config_file_path)?;
    log::info!("Validating config");
    config.validate()?;
    log::info!("Config validated");

    let cancellation_token = tokio_util::sync::CancellationToken::new();

    let om = std::sync::Arc::new(tokio::sync::Mutex::new(token::OauthManager::new(
        utils::oauth_utils::new_oauth_basic_client(
            config.client_id,
            config.client_secret,
            config.redirect_address,
        )?,
        token_storage::TokenStorage::load(config.token_file_path)?,
    )));

    token::OauthManager::spawn_token_receiver(om.clone(), core::time::Duration::from_millis(500))
        .await;
    token::OauthManager::spawn_token_refresher(om.clone(), core::time::Duration::from_secs(60))
        .await;
    let am = std::sync::Arc::new(tokio::sync::Mutex::new(AccountManager::new(
        config.trading_config.clone(),
        om.clone(),
    )));

    let jh = tokio::spawn(server::run_server(
        config.bind_address.parse()?,
        om.clone(),
        am.clone(),
        cancellation_token.clone(),
        config.cert_path,
        config.key_path,
    ));

    am.lock()
        .await
        .init(tokio::time::Duration::from_secs(5))
        .await?;

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
