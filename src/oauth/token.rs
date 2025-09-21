use crate::{
    Error,
    oauth::{token_storage, utils},
    server::server,
};
use chrono::Local;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse, reqwest};
use std::{sync as sSync, time as sTime};
use tokio::{sync as tSync, sync::oneshot, time as tTime};

pub type OauthTokenResponse =
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>;

struct TokenMessenger {
    auth_code_receiver: oneshot::Receiver<String>,
}

impl TokenMessenger {
    fn new(auth_code_receiver: oneshot::Receiver<String>) -> Self {
        Self {
            auth_code_receiver: auth_code_receiver,
        }
    }
}

pub struct OauthManager {
    token_manager: server::TokenManager,
    receivers: sSync::Arc<tSync::Mutex<Option<TokenMessenger>>>,
    token_receiver_manager_join_handle: Option<tokio::task::JoinHandle<()>>,
    token_refresh_manager_join_handle: Option<tokio::task::JoinHandle<()>>,
    client: utils::oauth_utils::Client,
    token_storage: sSync::Arc<tSync::Mutex<token_storage::TokenStorage>>,
    current_auth_url: Option<String>,
}

impl OauthManager {
    pub fn new(
        token_manager: server::TokenManager,
        client: utils::oauth_utils::Client,
        token_storage: sSync::Arc<tSync::Mutex<token_storage::TokenStorage>>,
    ) -> Self {
        Self {
            token_manager: token_manager,
            receivers: sSync::Arc::new(tSync::Mutex::new(None)),
            token_receiver_manager_join_handle: None,
            token_refresh_manager_join_handle: None,
            client: client,
            token_storage: token_storage,
            current_auth_url: None,
        }
    }

    pub async fn has_token(&self) -> bool {
        self.token_storage.lock().await.has_token()
    }

    pub async fn get_token(&self) -> Option<Result<OauthTokenResponse, Error>> {
        self.token_storage.lock().await.get_token()
    }

    pub async fn spawn_token_refresher(&mut self, period: core::time::Duration) -> () {
        if self.token_refresh_manager_join_handle.is_none() {
            let receivers = self.receivers.clone();
            let client = self.client.clone();
            let token_storage = self.token_storage.clone();
            self.token_refresh_manager_join_handle = Some(tokio::spawn(async move {
                loop {
                    tTime::sleep(period).await;

                    let mut token_storage_handle = token_storage.lock().await;
                    if let Some(Ok((token, expir))) =
                        token_storage_handle.get_token_and_expiration()
                    {
                        // make the buffer time configurable
                        if chrono::prelude::Utc::now()
                            > (expir - std::time::Duration::from_secs(180))
                        {
                            match token.refresh_token() {
                                Some(refresh_token) => match client
                                    .exchange_refresh_token(refresh_token)
                                    .request_async(&reqwest::Client::new())
                                    .await
                                {
                                    Ok(token) => {}
                                    Err(e) => {}
                                },
                                None => {}
                            }
                        }
                    }
                }
            }));
        }
    }

    pub async fn spawn_token_receiver(&mut self, period: core::time::Duration) -> () {
        if self.token_receiver_manager_join_handle.is_none() {
            let receivers = self.receivers.clone();
            let client = self.client.clone();
            let token_storage = self.token_storage.clone();
            self.token_receiver_manager_join_handle = Some(tokio::spawn(async move {
                loop {
                    tTime::sleep(period).await;

                    {
                        let mut cleanup = false;
                        if let Some(r) = receivers.lock().await.as_mut() {
                            match r.auth_code_receiver.try_recv() {
                                Ok(code) => {
                                    match client
                                        .exchange_code(AuthorizationCode::new(code))
                                        .request_async(&reqwest::Client::new())
                                        .await
                                    {
                                        Ok(token) => {
                                            // calculate the expiration date
                                            let expiration = (Local::now()
                                                + (match token.expires_in() {
                                                    Some(duration) => duration,
                                                    None => sTime::Duration::from_secs(1800),
                                                }))
                                            .to_utc();

                                            if let Err(e) = token_storage
                                                .lock()
                                                .await
                                                .set_token(&token, expiration)
                                            {
                                                log::info!(
                                                    "Failed to set the received oauth token: {}",
                                                    e
                                                );
                                            }
                                        }
                                        Err(e) => {
                                            log::warn!("error exchanging a token: {}", e);
                                        }
                                    }
                                }
                                Err(oneshot::error::TryRecvError::Empty) => {
                                    log::info!("No oauth code has been received yet");
                                }
                                Err(oneshot::error::TryRecvError::Closed) => {
                                    log::debug!(
                                        "Current oauth code receiver channel is closed, removing..."
                                    );
                                    cleanup = true;
                                }
                            }
                        }
                        if cleanup {
                            *receivers.lock().await = None;
                        }
                    }
                }
            }));
        }
    }

    pub fn token_manager(&mut self) -> &mut server::TokenManager {
        &mut self.token_manager
    }

    pub fn get_auth_url(&self) -> Option<String> {
        self.current_auth_url.clone()
    }

    // returns auth url
    pub async fn reset_auth_url(&mut self) -> String {
        // Generate the full authorization URL.
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            // Set the desired scopes.
            .add_scope(Scope::new("readonly".to_string()))
            .url();

        let auth_code_receiver = self
            .token_manager
            .new_token_request(csrf_token.secret().to_string());

        *self.receivers.lock().await = Some(TokenMessenger::new(auth_code_receiver));

        self.current_auth_url = Some(auth_url.to_string());
        auth_url.to_string()
    }
}
