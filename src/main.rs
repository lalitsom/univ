mod serve;
mod mock;
mod serve_types;

use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpServer};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, TokenUrl, ClientId, ClientSecret, RedirectUrl, Scope, AuthorizationCode, StandardTokenResponse, EmptyExtraTokenFields, TokenResponse};
use oauth2::reqwest::async_http_client;
use std::sync::Mutex;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let port: u16 = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8070);                

    let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("Missing client ID"));
    let client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").expect("Missing client secret"));
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).expect("Invalid Auth URL");
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).expect("Invalid Token URL");

    let redirect_url = RedirectUrl::new("https://7b81-27-4-59-212.ngrok-free.app/oauth2callback".to_string()).expect("Invalid Redirect URL");

    let oauth_client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    let app_data = web::Data::new(serve_types::AppState {
        oauth_client: Mutex::new(oauth_client),
    });

    println!("Starting Server at port {}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/static/{filename:.*}", web::get().to(serve::serve_static_file)) // Serve static files (CSS, JS, images, etc.)
            // .route("/api/", web::get().to(serve_apis)) // user apis
            // .route("/admin/", web::get().to(serve_admin_apis)) // admin apis
            
            // serve dynamic html pages
            .route("/", web::get().to(serve::serve_home)) // Home page
            .route("/problems", web::get().to(serve::serve_problems)) // Problems list page
            .route("/problem/{problemId}", web::get().to(serve::serve_problem)) // Problem details page
            .route("/profile", web::get().to(serve::serve_profile)) // user profile page
            .route("/leaderboard", web::get().to(serve::serve_leaderboard)) // Leaderboard page
            
            .route("/sign_in", web::get().to(serve::sign_in))
            .route("/oauth2callback", web::get().to(serve::oauth2callback))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await

}