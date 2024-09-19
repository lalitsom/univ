mod serve;
mod mock;
mod serve_types;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Server at port 8080...");
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}