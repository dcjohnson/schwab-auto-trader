use crate::{
    Error,
    oauth::{token_storage, utils},
    server::server,
};
use oauth2::{AuthorizationCode, CsrfToken, Scope, reqwest};
use std::sync;
use tokio::{sync as tSync, sync::oneshot, time as tTime};

pub type OauthTokenResponse =
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>;

struct TokenMessenger {
    auth_code_receiver: oneshot::Receiver<String>,
}

impl TokenMessenger {
    fn new(
        auth_code_receiver: oneshot::Receiver<String>,
    ) -> Self {
        Self {
            auth_code_receiver: auth_code_receiver,
        }
    }
}

pub struct OauthManager {
    token_manager: sync::Arc<sync::Mutex<server::TokenManager>>,
    receivers: sync::Arc<tokio::sync::Mutex<Option<TokenMessenger>>>,
    token_receiver_manager_join_handle: Option<tokio::task::JoinHandle<()>>,
    client: utils::oauth_utils::Client,
    token_storage: sync::Arc<sync::Mutex<token_storage::TokenStorage>>,
}

impl OauthManager {
    pub fn new(
        token_manager: sync::Arc<sync::Mutex<server::TokenManager>>,
        client: utils::oauth_utils::Client,
        token_storage: sync::Arc<sync::Mutex<token_storage::TokenStorage>>,
    ) -> Self {
        Self {
            token_manager: token_manager,
            receivers: sync::Arc::new(tSync::Mutex::new(None)),
            token_receiver_manager_join_handle: None,
            client: client,
            token_storage: token_storage,
        }
    }

    pub fn has_token(&self) -> bool {
        if let Ok(lr) = self.token_storage.lock() {
            lr.has_token()
        } else {
            false
        }
    }

    pub fn get_token(&self) -> Option<Result<OauthTokenResponse, Error>> {
        self.token_storage.lock().ok()?.get_token()
    }

    pub async fn spawn_token_receiver(&mut self, period: core::time::Duration) -> () {
        if self.token_receiver_manager_join_handle.is_none() {
                                            }



            let receivers = self.receivers.clone();
            let client = self.client.clone();
            let token_storage = self.token_storage.clone();
            self.token_receiver_manager_join_handle = Some(tokio::spawn(async move {
                loop {
                    tTime::sleep(period).await;

                    {
                        if let Some(mut r)  = receivers.lock().await {

                            match r.auth_code_receiver.try_recv() {
                                Ok(code) => {
                                    match client
                                        .exchange_code(AuthorizationCode::new(code))
                                        .request_async(&reqwest::Client::new())
                                        .await
                                    {
                                        Ok(token) => {
                                                if let Ok(mut token_storage_handle) =
                                                    token_storage.lock()
                                                {
                                                    if let Err(e) =
                                                        token_storage_handle.set_token(&token)
                                                    {
                                                        // handle error
                                                    }
                                                }

                                        },
                                        Err(e) => {
                                            println!("Error exchanging token: {}", e);
                                        }
                                    }
                                }
                                Err(oneshot::error::TryRecvError::Empty) => {
                                // log
                                }
                                Err(oneshot::error::TryRecvError::Closed) => {
                                    // log an error saying that the receiver is closed
                            }
                            }
                        }
                    }
                }
            }));
        }

    // returns auth url and token receiver one shot
    pub async fn auth_url(&mut self) -> String {
        // Generate the full authorization URL.
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            // Set the desired scopes.
            .add_scope(Scope::new("readonly".to_string()))
            .url();

        let auth_code_receiver = self
            .token_manager
            .lock()
            .unwrap()
            .new_token_request(csrf_token.secret().to_string())
            .unwrap();

        *self.receivers.lock().await = Some(TokenMessenger::new(auth_code_receiver));

        auth_url.to_string()
    }
}
