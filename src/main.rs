mod global_state;
mod mock;
mod serve;
mod serve_types;
mod storage;
mod oauth;

use storage::model;
use storage::schema;

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
    HttpServer::new(move || {
        App::new()
            .route(
                "/static/{filename:.*}",
                web::get().to(serve::serve_static_file),
            ) // Serve static files (CSS, JS, images, etc.)
            // .route("/api/", web::get().to(serve_apis)) // user apis
            .route("/api/check_answer/{problemId}/{answer}", web::get().to(serve::check_answer))
            // .route("/api/oauth2callback", web::get().to(serve::oauth2callback))
            // .route("/admin/", web::get().to(serve_admin_apis)) // admin apis
            // serve dynamic html pages
            .route("/", web::get().to(serve::serve_home)) // Home page
            .route("/problems", web::get().to(serve::serve_problems)) // Problems list page
            .route("/problem/{problemId}", web::get().to(serve::serve_problem)) // Problem details page
            .route("/sign_in", web::get().to(serve::serve_sign_in)) // Sign in page
            .route("/profile", web::get().to(serve::serve_profile)) // user profile page
            .route("/leaderboard", web::get().to(serve::serve_leaderboard)) // Leaderboard page
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
