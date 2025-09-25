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
        redirect_address: String,
    ) -> Result<Client, Box<dyn error::Error + Send + Sync>> {
        Ok(oauth2::basic::BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(AuthUrl::new(AUTHORIZE_ENDPOINT.to_string())?)
            .set_token_uri(TokenUrl::new(TOKEN_ENDPOINT.to_string())?)
            .set_redirect_uri(RedirectUrl::new(redirect_address)?))
    }




}
