use oauth2::basic::BasicClient;
use oauth2::reqwest;

use oauth2::{
    AuthUrl, /* AuthorizationCode, */ ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, /* TokenResponse, */ TokenUrl,
};

use schwab_auto_trader::oauth::token_server;
// use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //token_server::run_server(1337).await?;

    let mut tm = std::sync::Arc::new(std::sync::Mutex::new(token_server::TokenManager::new()));
    let f = tokio::spawn(token_server::run_server(8182, tm));

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(ClientId::new(
        "".to_string(),
    ))
    .set_client_secret(ClientSecret::new("".to_string()))
    .set_auth_uri(AuthUrl::new(
        "https://api.schwabapi.com/v1/oauth/authorize".to_string(),
    )?)
    .set_token_uri(TokenUrl::new(
        "https://api.schwabapi.com/v1/oauth/token".to_string(),
    )?)
    .set_redirect_uri(RedirectUrl::new("https://127.0.0.1:8182".to_string())?);

    // Generate a PKCE challenge.
    let (_pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("readonly".to_string()))
        // .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        // .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Token: {}", csrf_token.secret());
    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    /*
        // Once the user has been redirected to the redirect URL, you'll have access to the
        // authorization code. For security reasons, your code should verify that the `state`
        // parameter returned by the server matches `csrf_token`.

        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        // Now you can trade it for an access token.
        let token_result = client
            .exchange_code(AuthorizationCode::new(
                "some authorization code".to_string(),
            ))
            // Set the PKCE code verifier.
           //  .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await?;
    */
    // Unwrapping token_result will either produce a Token or a RequestTokenError.

    f.await??;

    Ok(())
}
