use crate::{Error, oauth::{utils, token_storage}, server::server};
use oauth2::{AuthorizationCode, CsrfToken, Scope, reqwest};
use std::sync;
use tokio::{sync as tSync, sync::oneshot, time as tTime};

pub type OauthTokenResponse =
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>;

struct TokenMessenger {
    auth_code_receiver: oneshot::Receiver<String>,
    auth_token_sender: Option<oneshot::Sender<OauthTokenResponse>>,
}

impl TokenMessenger {
    fn new(
        auth_code_receiver: oneshot::Receiver<String>,
        auth_token_sender: oneshot::Sender<OauthTokenResponse>,
    ) -> Self {
        Self {
            auth_code_receiver: auth_code_receiver,
            auth_token_sender: Some(auth_token_sender),
        }
    }
}

pub struct OauthManager {
    token_manager: sync::Arc<sync::Mutex<server::TokenManager>>,
    receivers: sync::Arc<tokio::sync::Mutex<Vec<TokenMessenger>>>,
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
            receivers: sync::Arc::new(tSync::Mutex::new(Vec::new())),
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
            let receivers = self.receivers.clone();
            let client = self.client.clone();
            let token_storage = self.token_storage.clone();
            self.token_receiver_manager_join_handle = Some(tokio::spawn(async move {
                loop {
                    tTime::sleep(period).await;

                    {
                        let mut r = receivers.lock().await;

                        let mut i = 0;
                        while i < r.len() {
                            match r[i].auth_code_receiver.try_recv() {
                                Ok(code) => {
                                    match client
                                        .exchange_code(AuthorizationCode::new(code))
                                        .request_async(&reqwest::Client::new())
                                        .await
                                    {
                                        Ok(token) => match r[i].auth_token_sender.take() {
                                            Some(ts) => {
                                                if let Ok(mut token_storage_handle) = token_storage.lock() {
                                                if let Err(e) = token_storage_handle.set_token( &token) {
                                                    // handle error
                                                }
                                                }

                                                // Get rid of this auth token sender
                                                if let Err(_) = ts.send(token) {
                                                    println!("Error sending token");
                                                }
                                            }
                                            None => println!("No token sender!"),
                                        },
                                        Err(e) => {
                                            println!("Error exchanging token: {}", e);
                                        }
                                    }
                                    r.remove(i);
                                }
                                Err(oneshot::error::TryRecvError::Empty) => {
                                    i += 1;
                                }
                                Err(oneshot::error::TryRecvError::Closed) => {
                                    r.remove(i);
                                    // log an error saying that the receiver is closed
                                }
                            }
                        }
                    }
                }
            }));
        }
    }

    // returns auth url and token receiver one shot
    pub async fn auth_url(&mut self) -> (String, oneshot::Receiver<OauthTokenResponse>) {
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

        let (token_sender, token_receiver) = oneshot::channel();

        self.receivers
            .lock()
            .await
            .push(TokenMessenger::new(auth_code_receiver, token_sender));

        (auth_url.to_string(), token_receiver)
    }
}
