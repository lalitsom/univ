mod serve;
mod mock;
mod serve_types;

use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let port: u16 = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8090);                

    println!("Starting Server at port {}", port);
    HttpServer::new(|| {
        App::new()
            .route("/static/{filename:.*}", web::get().to(serve::serve_static_file)) // Serve static files (CSS, JS, images, etc.)
            // .route("/api/", web::get().to(serve_apis)) // user apis
            // .route("/admin/", web::get().to(serve_admin_apis)) // admin apis
            
            // serve dynamic html pages
            .route("/", web::get().to(serve::serve_home)) // Home page
            .route("/problems", web::get().to(serve::serve_problems)) // Problems list page
            .route("/problem/{problemId}", web::get().to(serve::serve_problem)) // Problem details page
            .route("/profile", web::get().to(serve::serve_profile)) // user profile page
            .route("/leaderboard", web::get().to(serve::serve_leaderboard)) // Leaderboard page
            .route("/signIn", web::get().to(serve::serve_sign_in)) // Sign in page
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}