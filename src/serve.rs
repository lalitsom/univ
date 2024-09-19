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
        logged_in: mock::is_logged_in(),
        user_initials: mock::user_initials()
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
        user_initials: mock::user_initials(),
        problems: mock::get_problems()
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
        user_initials: mock::user_initials(),
        problem_name: "Two Sum",
        problem_description: "Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.",
        problem_hint: "try hintttt",
        problem_id: 1
    };

    // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_sign_in() -> Result<HttpResponse> {
    // Create the template instance with dynamic data
    let template = serve_types::ProblemTemplate {
        logged_in: mock::is_logged_in(),
        user_initials: mock::user_initials(),
        problem_name: "Two Sum",
        problem_description: "Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.",
        problem_hint: "try hintttt",
        problem_id: 1
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
        user_initials: mock::user_initials(),
        users: mock::get_users()
    };

    // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;


    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_profile() -> Result<HttpResponse> {
    // Create the template instance with dynamic data
    // let template = HomeTemplate {

    // };

    // // Render the template and return as an HTTP response
    // let rendered = template
    //     .render()
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body("rendered"))
}

// pub async fn serve_problems() -> Result<HttpResponse> {
//     // Create the template instance with dynamic data
//     let template = HomeTemplate { quote: "Hello, dynamic world!" };

//     // Render the template and return as an HTTP response
//     let rendered = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

//     Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
// }