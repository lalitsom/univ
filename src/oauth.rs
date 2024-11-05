use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use std::env;
// use oauth2::CsrfToken;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl, TokenResponse
};
// use reqwest::Client;
use serde::Deserialize;

// use oauth2::{
//     AuthUrl, AuthorizationCode, ClientId, ClientSecret, EmptyExtraTokenFields, RedirectUrl, Scope,
//     StandardTokenResponse, TokenResponse, TokenUrl, CsrfToken
// };

#[derive(Debug, Deserialize)]
pub struct OAuthRequest {
    pub code: String,
    pub state: String,
}

pub fn initialize_oauth_client() -> BasicClient {
    let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("Missing client ID"));
    let client_secret =
        ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").expect("Missing client secret"));
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())
        .expect("Invalid Auth URL");
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .expect("Invalid Token URL");

    let redirect_url =
        RedirectUrl::new(env::var("OAUTH2_CALLBACK_URL").expect("Missing redirect url"))
            .expect("Invalid Redirect URL");

    let oauth_client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    oauth_client
}

pub fn get_authorize_url() -> String {
    let client = initialize_oauth_client();

    // Generate the authorization URL and CSRF token
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random).add_extra_param("prompt", "select_account")
        .add_scope(Scope::new("email".into()))
        .url();

    auth_url.to_string()
}

pub async fn handle_oauth2callback(
    query_code: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = initialize_oauth_client();

    // Exchange the code with a token
    let token_result = client
        .exchange_code(AuthorizationCode::new(query_code))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret();
            // Get the user info
            let user_info = reqwest::get(
                "https://www.googleapis.com/oauth2/v1/userinfo?alt=json&access_token=".to_string()
                    + &access_token,
            )
            .await?
            .json::<serde_json::Value>()
            .await?;

            Ok(user_info)
        }
        Err(e) => Err(Box::new(e)),
    }
    
}
