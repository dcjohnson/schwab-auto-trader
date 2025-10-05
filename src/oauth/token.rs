use crate::{
    Error,
    oauth::{token_storage, utils},
};
use chrono::{DateTime, Local, Utc};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse, reqwest};
use std::{sync as sSync, time as sTime};
use tokio::{sync as tSync, sync::oneshot, time as tTime};

pub struct TokenManager {
    state_token: Option<String>,
    sender: Option<oneshot::Sender<String>>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            state_token: None,
            sender: None,
        }
    }

    pub fn send_token(
        &mut self,
        auth_token: String,
        state_token: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(cached_state_token) = self.state_token.as_ref() {
            if cached_state_token == state_token {
                match self.sender.take() {
                    Some(sender) => {
                        sender.send(auth_token)?;
                        Ok(())
                    }
                    None => Err("No sender for state token".to_string().into()),
                }
            } else {
                Err("State Tokens didn't match".to_string().into())
            }
        } else {
            Err("No stored state token".to_string().into())
        }
    }

    pub fn new_token_request(&mut self, state_token: String) -> oneshot::Receiver<String> {
        let (s, r) = oneshot::channel();

        self.state_token = Some(state_token);
        self.sender = Some(s);

        r
    }
}

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
    token_manager: TokenManager,
    receiver: Option<TokenMessenger>,
    token_receiver_manager_join_handle: Option<tokio::task::JoinHandle<()>>,
    token_refresh_manager_join_handle: Option<tokio::task::JoinHandle<()>>,
    client: utils::oauth_utils::Client,
    token_storage: token_storage::TokenStorage,
    current_auth_url: Option<String>,
}

impl OauthManager {
    pub fn new(
        client: utils::oauth_utils::Client,
        token_storage: token_storage::TokenStorage,
    ) -> Self {
        Self {
            token_manager: TokenManager::new(),
            receiver: None,
            token_receiver_manager_join_handle: None,
            token_refresh_manager_join_handle: None,
            client: client,
            token_storage: token_storage,
            current_auth_url: None,
        }
    }

    pub fn has_token(&self) -> bool {
        self.token_storage.has_token()
    }

    pub fn reset(&mut self) -> Result<(), Error> {
        self.token_storage.reset()
    }

    pub fn get_unexpired_token(&self) -> Option<Result<OauthTokenResponse, Error>> {
        match self.token_storage.get_token_and_expiration() {
            Some(Ok((t, e))) => {
                if chrono::prelude::Utc::now() >= e {
                    None
                } else {
                    Some(Ok(t))
                }
            }
            None => None,
            Some(Err(e)) => Some(Err(e)),
        }
    }

    fn calculate_expiration(expires_in: Option<core::time::Duration>) -> DateTime<Utc> {
        (Local::now()
            + (match expires_in {
                Some(duration) => duration,
                None => sTime::Duration::from_secs(1800), // some default value
            }))
        .to_utc()
    }

    pub async fn spawn_token_refresher(
        s: sSync::Arc<tSync::Mutex<Self>>,
        period: core::time::Duration,
    ) {
        let s_c = s.clone();

        {
            let mut s_lock_handle = s.lock().await;
            if s_lock_handle.token_refresh_manager_join_handle.is_none() {
                s_lock_handle.token_refresh_manager_join_handle = Some(tokio::spawn(async move {
                    loop {
                        log::info!("Attempting token refresh");

                        {
                            let mut s_handle = s_c.lock().await;
                            if let Some(Ok((token, expir))) =
                                s_handle.token_storage.get_token_and_expiration()
                            {
                                // make the buffer time configurable
                                if chrono::prelude::Utc::now()
                                    > (expir - std::time::Duration::from_secs(180))
                                {
                                    log::info!("Token is expired, refreshing...");
                                    if let Some(refresh_token) = token.refresh_token() {
                                        if let Err(e) = async {
                                            let token = s_handle
                                                .client
                                                .exchange_refresh_token(refresh_token)
                                                .request_async(&reqwest::Client::new())
                                                .await?;
                                            log::info!("token refreshed, storing...");
                                            s_handle.token_storage.set_token(
                                                &token,
                                                Self::calculate_expiration(token.expires_in()),
                                            )?;
                                            log::info!("token refresh complete");
                                            Ok::<(), Error>(())
                                        }
                                        .await
                                        {
                                            log::error!(
                                                "Couldn't refresh oauth token: '{}'; resetting...",
                                                e
                                            );
                                            if let Err(e) = s_handle.reset() {
                                                // unrecoverable error, panicing
                                                panic!("Can't reset oauth backend: '{}'", e);
                                            }
                                        }
                                    }
                                }
                            } else {
                                log::info!("No token to refresh");
                            }
                        }
                        log::info!(
                            "Token is not expiring; sleeping until next token refresh check."
                        );
                        tTime::sleep(period).await;
                    }
                }));
            }
        }
    }

    pub async fn spawn_token_receiver(
        s: sSync::Arc<tSync::Mutex<Self>>,
        period: core::time::Duration,
    ) {
        let s_c = s.clone();

        {
            let mut s_lock_handle = s.lock().await;
            if s_lock_handle.token_receiver_manager_join_handle.is_none() {
                s_lock_handle.token_receiver_manager_join_handle = Some(tokio::spawn(async move {
                    loop {
                        tTime::sleep(period).await;

                        {
                            let mut s_lock_handle = s_c.lock().await;
                            let mut cleanup = false;
                            if let Some(r) = s_lock_handle.receiver.as_mut() {
                                match r.auth_code_receiver.try_recv() {
                                    Ok(code) => {
                                        match s_lock_handle
                                            .client
                                            .exchange_code(AuthorizationCode::new(code))
                                            .request_async(&reqwest::Client::new())
                                            .await
                                        {
                                            Ok(token) => {
                                                if let Err(e) =
                                                    s_lock_handle.token_storage.set_token(
                                                        &token,
                                                        Self::calculate_expiration(
                                                            token.expires_in(),
                                                        ),
                                                    )
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
                                s_lock_handle.receiver = None;
                            }
                        }
                    }
                }));
            }
        }
    }

    pub fn token_manager(&mut self) -> &mut TokenManager {
        &mut self.token_manager
    }

    pub fn get_auth_url(&self) -> Option<String> {
        self.current_auth_url.clone()
    }

    // returns auth url
    pub fn reset_auth_url(&mut self) -> String {
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

        self.receiver = Some(TokenMessenger::new(auth_code_receiver));

        self.current_auth_url = Some(auth_url.to_string());
        auth_url.to_string()
    }
}
