use crate::mock;
use crate::serve_types;

use actix_files::NamedFile;
use askama::Template;

use actix_web::{web, HttpResponse, Result};

// Serve static files like CSS
pub async fn serve_static_file(path: web::Path<String>) -> Result<NamedFile> {
    NamedFile::open(format!("templates/static/{}", path.into_inner()))
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))
}


pub async fn serve_home() -> Result<HttpResponse> {
    // Create the template instance with dynamic data
    let template = serve_types::HomeTemplate {
        logged_in: mock::is_logged_in()
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


pub async fn serve_sign_in() -> Result<HttpResponse> {
    // Create the template instance with dynamic data
    

    // Render the template and return as an HTTP response
    // let rendered = template
    //     .render()
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("serve_sign_in: Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body("rendered"))
}