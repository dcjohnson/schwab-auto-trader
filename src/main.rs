use oauth2::basic::BasicClient;
use oauth2::reqwest;

use oauth2::{
    AuthUrl,  AuthorizationCode,  ClientId, ClientSecret, CsrfToken, 
    RedirectUrl, Scope,   TokenUrl,
};

use std::{fs, env};
use json;

use schwab_auto_trader::oauth::token_server;
// use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // add command line parsing later
    let args: Vec<String> = env::args().collect();

    let config = json::parse(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let tm  = std::sync::Arc::new(std::sync::Mutex::new(token_server::TokenManager::new()));
    let f = tokio::spawn(token_server::run_server(8182, tm.clone()));

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(ClientId::new(
    config["clientId"].to_string(),
    ))
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
    
    let mut token_receiver = tm.lock().unwrap().new_token_request(csrf_token.secret().to_string()).unwrap();
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
   
    println!("Got the token!: {:?}", token_result);

    f.await??;

    Ok(())
}
