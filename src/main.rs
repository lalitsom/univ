mod global_state;
mod mock;
mod oauth;
mod serve;
mod serve_types;
mod storage;

use storage::model;
use storage::schema;

use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8070);

    println!("Starting Server at port {}", port);

    let session_key: [u8; 32] = env::var("SESSION_KEY")
        .unwrap_or_else(|_| "00000000000000000000000000000000".to_string()) // Fallback default key
        .as_bytes()
        .try_into()
        .expect("SESSION_KEY must be exactly 32 bytes long");

    HttpServer::new(move || {
        App::new()
            .wrap(
                CookieSession::private(&session_key)
                    .secure(true) // Only allow over HTTPS
                    .http_only(true)
                    .max_age(86400),
            )
            .route(
                "/static/{filename:.*}",
                web::get().to(serve::serve_static_file),
            ) // Serve static files (CSS, JS, images, etc.)
            // .route("/api/", web::get().to(serve_apis)) // user apis
            .route(
                "/api/check_answer/{problemId}/{answer}",
                web::get().to(serve::check_answer),
            )
            .route("/api/oauth2callback", web::get().to(serve::oauth2callback))
            // .route("/admin/", web::get().to(serve_admin_apis)) // admin apis
            // serve dynamic html pages
            .route("/", web::get().to(serve::serve_home)) // Home page
            .route("/problems", web::get().to(serve::serve_problems)) // Problems list page
            .route("/problem/{problemId}", web::get().to(serve::serve_problem)) // Problem details page
            .route("/sign_in", web::get().to(serve::serve_sign_in)) // Sign in page
            .route("/sign_out", web::get().to(serve::serve_sign_out)) // Sign in page
            .route("/profile", web::get().to(serve::serve_profile)) // user profile page
            .route("/about", web::get().to(serve::serve_about)) // user profile page
            .route("/leaderboard", web::get().to(serve::serve_leaderboard)) // Leaderboard page
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
