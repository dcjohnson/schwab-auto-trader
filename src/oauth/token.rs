use crate::oauth::token_server;
use std::sync;
use tokio::{sync as tSync, sync::oneshot, time as tTime};
use oauth2::{reqwest, AuthorizationCode, CsrfToken, Scope};

pub mod oauth_utils {
    pub type Client = oauth2::basic::BasicClient<
        oauth2::EndpointSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointSet,
    >;
    use std::error;

    use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

    const AUTHORIZE_ENDPOINT: &str = "https://api.schwabapi.com/v1/oauth/authorize";
    const TOKEN_ENDPOINT: &str = "https://api.schwabapi.com/v1/oauth/token";

    pub fn new_oauth_basic_client(
        client_id: String,
        client_secret: String,
    ) -> Result<Client, Box<dyn error::Error + Send + Sync>> {
        Ok(oauth2::basic::BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(AuthUrl::new(AUTHORIZE_ENDPOINT.to_string())?)
            .set_token_uri(TokenUrl::new(TOKEN_ENDPOINT.to_string())?)
            .set_redirect_uri(RedirectUrl::new("https://127.0.0.1:8182".to_string())?))
    }
}


type OauthTokenResponse =
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
    token_manager: sync::Arc<sync::Mutex<token_server::TokenManager>>,
    receivers: sync::Arc<tokio::sync::Mutex<Vec<TokenMessenger>>>,
    token_receiver_manager_join_handle: Option<tokio::task::JoinHandle<()>>,
    client: oauth_utils::Client,
}

impl OauthManager {
    pub fn new(
        token_manager: sync::Arc<sync::Mutex<token_server::TokenManager>>,
        client: oauth_utils::Client,
    ) -> Self {
        Self {
            token_manager: token_manager,
            receivers: sync::Arc::new(tSync::Mutex::new(Vec::new())),
            token_receiver_manager_join_handle: None,
            client: client,
        }
    }

    pub async fn spawn_token_receiver(&mut self, period: core::time::Duration) -> () {
        if self.token_receiver_manager_join_handle.is_none() {
            let receivers = self.receivers.clone();
            let client = self.client.clone();
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
                                    //acc.push(v);
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
