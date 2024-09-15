use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn get_strings() -> impl Responder {
    let strings = vec!["apple", "banana", "cherry"];
    HttpResponse::Ok().json(strings) // Returns a JSON response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_strings) // Register the route
    })
    .bind("127.0.0.1:8080")? // Binds to localhost on port 8080
    .run()
    .await
}
