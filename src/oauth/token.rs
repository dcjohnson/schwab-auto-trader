use std::sync;
use crate::oauth::token_server;

const AUTHORIZE_ENDPOINT: &str = "https://api.schwabapi.com/v1/oauth/authorize";
const TOKEN_ENDPOINT: &str = "https://api.schwabapi.com/v1/oauth/token";

pub struct OauthManager { 
    token_manager: sync::Arc<sync::Mutex<token_server::TokenManager>>, 

    // manage the token receivers internally
}

impl OauthManager {
    pub fn new(token_manager: sync::Arc<sync::Mutex<token_server::TokenManager>>) -> Self {
        Self { token_manager }
    }

    pub fn auth_url(client_id: String, client_secret: String) /* returns auth_url */ {

        /*
    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(ClientId::new(config["clientId"].to_string()))
        .set_client_secret(ClientSecret::new(config["clientSecret"].to_string()))
        .set_auth_uri(AuthUrl::new(
            "https://api.schwabapi.com/v1/oauth/authorize".to_string(),
        )?)
        .set_token_uri(TokenUrl::new(
            "https://api.schwabapi.com/v1/oauth/token".to_string(),
        )?)
        .set_redirect_uri(RedirectUrl::new("https://127.0.0.1:8182".to_string())?);

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("readonly".to_string()))
        .url();



    let mut token_receiver = tm
        .lock()
        .unwrap()
        .new_token_request(csrf_token.secret().to_string())
        .unwrap();
        */
    }

    async pub fn exchange_token( /* csrf_token */ ) /* The outputs of token result */ {

        /*
    let http_client = reqwest::Client::new();

    let code = token_receiver.recv().await.unwrap();
    println!("code: {}", code);
    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(&http_client)
        .await?;

token_result is the access token
*/ 
    }


}
/*
    let tm = std::sync::Arc::new(std::sync::Mutex::new(token_server::TokenManager::new()));
    
    // THIS PART should be launched outside of the oauth flow. 
    let f = tokio::spawn(token_server::run_server(8182, tm.clone()));

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(ClientId::new(config["clientId"].to_string()))
        .set_client_secret(ClientSecret::new(config["clientSecret"].to_string()))
        .set_auth_uri(AuthUrl::new(
            "https://api.schwabapi.com/v1/oauth/authorize".to_string(),
        )?)
        .set_token_uri(TokenUrl::new(
            "https://api.schwabapi.com/v1/oauth/token".to_string(),
        )?)
        .set_redirect_uri(RedirectUrl::new("https://127.0.0.1:8182".to_string())?);

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("readonly".to_string()))
        .url();

    println!("Browse to: {}", auth_url);

    let mut token_receiver = tm
        .lock()
        .unwrap()
        .new_token_request(csrf_token.secret().to_string())
        .unwrap();
    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_token`.

    let http_client = reqwest::Client::new();

    let code = token_receiver.recv().await.unwrap();
    println!("code: {}", code);
    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(&http_client)
        .await?;
*/
