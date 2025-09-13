use clap::Parser;
use json;
use schwab_auto_trader::{
    Error,
    oauth::{token, token_storage, utils},
    server::server,
};
use std::{
    fs,
    net::{Ipv4Addr, SocketAddr},
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    // todo: add a nice login web page
    // todo: Implement token refresh for expiring tokens.
    // if it exists.
    //
    //

    env_logger::init();

    let args = Args::parse();
    let config = json::parse(&fs::read_to_string(&args.config_file_path).unwrap()).unwrap();
    let cancellation_token = tokio_util::sync::CancellationToken::new();

    let om = std::sync::Arc::new(tokio::sync::Mutex::new(token::OauthManager::new(
        server::TokenManager::new(),
        utils::oauth_utils::new_oauth_basic_client(
            config["clientId"].to_string(),
            config["clientSecret"].to_string(),
            config["redirectAddress"].to_string(),
        )?,
        std::sync::Arc::new(std::sync::Mutex::new(token_storage::TokenStorage::load(
            config["tokenFilePath"].to_string(),
        )?)),
    )));

    om.lock()
        .await
        .spawn_token_receiver(core::time::Duration::from_millis(500))
        .await;

    let jh = tokio::spawn(server::run_server(
        SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8182),
        om,
        cancellation_token.clone(),
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
