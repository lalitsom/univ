use crate::mock;
use crate::serve_types;

use actix_files::NamedFile;
use askama::Template;

use actix_web::{web, HttpResponse, Result, App, HttpServer, Responder};
use actix_web::http::header;
use oauth2::reqwest::async_http_client;
use oauth2::CsrfToken;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, TokenUrl, ClientId, ClientSecret, RedirectUrl, Scope, AuthorizationCode, StandardTokenResponse, EmptyExtraTokenFields, TokenResponse};
use std::sync::Mutex;
use serde::Deserialize;
use reqwest::Client;

// Serve static files like CSS
pub async fn serve_static_file(path: web::Path<String>) -> Result<NamedFile> {
    NamedFile::open(format!("templates/static/{}", path.into_inner()))
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))
}


pub async fn serve_home() -> Result<HttpResponse> {
    // Create the template instance with dynamic data
    let template = serve_types::HomeTemplate {
        logged_in: false
    };

    // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_problems() -> Result<HttpResponse> {
    

    let template = serve_types::ProblemsTemplate {
        logged_in: mock::is_logged_in(),
        problems: mock::get_all_problems()
    };

    // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_problem() -> Result<HttpResponse> {
    let template = serve_types::ProblemTemplate {
        logged_in: mock::is_logged_in(),
        problem: mock::get_problem()
    };

    // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}


pub async fn serve_leaderboard() -> Result<HttpResponse> {
   
    let template = serve_types::LeaderboardTemplate {
        logged_in: mock::is_logged_in(),
        users: mock::get_leaderboard_users()
    };

    // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("serve_leaderboard : Template error"))?;


    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}


pub async fn sign_in(data: web::Data<serve_types::AppState>) -> impl Responder {
    println!("Entering sign_in function");
    let client = match data.oauth_client.lock() {
        Ok(client) => client,
        Err(poisoned) => {
            eprintln!("Failed to acquire lock on oauth_client: {:?}", poisoned);
            return HttpResponse::InternalServerError().body("Internal server error");
        }
    };
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random).add_extra_param("prompt", "select_account")
        .add_scope(Scope::new("email".into()))
        .url();

    println!("Redirecting to Google Auth URL: {}", auth_url);
    HttpResponse::Found()
        .header(header::LOCATION, auth_url.to_string())
        .finish()
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
}


#[derive(Deserialize)]
pub struct UserInfo {
    pub email: String,
}

pub async fn serve_profile() -> Result<HttpResponse> {
    // Create the template instance with dynamic data
    let template = serve_types::ProfileTemplate {
        logged_in: mock::is_logged_in(),
        user: mock::get_user_profile()
    };

    // // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("serve_profile : Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_profile_home(user: serve_types::User) -> Result<HttpResponse> {
    // Create the template instance with dynamic data
    let template = serve_types::ProfileTemplate {
        logged_in: true,
        user,
    };

    // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("serve_profile : Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn oauth2callback(query: web::Query<CallbackQuery>, data: web::Data<serve_types::AppState>) -> impl Responder {
    let code = &query.code;

    let client = data.oauth_client.lock().unwrap();
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client).await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret();

            let user_info_url = format!("https://www.googleapis.com/oauth2/v1/userinfo?alt=json&access_token={}", access_token);
            let client = Client::new();
            let user_info_response = client.get(&user_info_url).send().await;

            match user_info_response {
                Ok(response) => {
                    if response.status().is_success() {
                        let user_info: UserInfo = response.json().await.unwrap();
                        let user =serve_types::User {
                            id: 1, // Default id
                            name: "Google User".to_string(), // Default name
                            email: user_info.email,
                            attempted: 0, // Default attempted problems
                            solved: 0, // Default solved problems
                            rank: 0, // Default rank
                            created_at: "2024-01-01T00:00:00Z".to_string(), // Default created_at
                            updated_at: "2024-01-01T00:00:00Z".to_string(), // Default updated_at
                        };
                        serve_profile_home(user).await
                    } else {
                        Ok(HttpResponse::BadRequest().body("Failed to fetch user info"))
                    }
                },
                Err(_) => Ok(HttpResponse::BadRequest().body("Error during user info request")),
            }
        },
        Err(_) => Ok(HttpResponse::BadRequest().body("Error during token exchange")),
    }
}