use oauth2::basic::BasicClient;
use std::env;
// use oauth2::reqwest::async_http_client;
// use oauth2::CsrfToken;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, CsrfToken, Scope};
// use reqwest::Client;
// use serde::Deserialize;

// use oauth2::{
//     AuthUrl, AuthorizationCode, ClientId, ClientSecret, EmptyExtraTokenFields, RedirectUrl, Scope,
//     StandardTokenResponse, TokenResponse, TokenUrl, CsrfToken
// };

pub fn initialize_oauth_client() -> BasicClient {
    let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("Missing client ID"));
    let client_secret =
        ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").expect("Missing client secret"));
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())
        .expect("Invalid Auth URL");
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .expect("Invalid Token URL");

    let redirect_url =
        RedirectUrl::new("https://7b81-27-4-59-212.ngrok-free.app/oauth2callback".to_string())
            .expect("Invalid Redirect URL");

    let oauth_client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    oauth_client
}

pub fn get_authorize_url() -> String {
    let client = initialize_oauth_client();

    // Generate the authorization URL and CSRF token
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .url();

    auth_url.to_string()
}
