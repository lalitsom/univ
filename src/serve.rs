use crate::mock;
use crate::serve_types;
use crate::storage::db;

use actix_files::NamedFile;
use askama::Template;

use actix_web::{web, HttpRequest, HttpResponse, Result};

// Serve static files like CSS
pub async fn serve_static_file(path: web::Path<String>) -> Result<NamedFile> {
    NamedFile::open(format!("templates/static/{}", path.into_inner()))
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))
}

pub async fn serve_home() -> Result<HttpResponse> {
    // Create the template instance with dynamic data
    let template = serve_types::HomeTemplate {
        logged_in: mock::is_logged_in(),
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
        problems: db::get_all_problems()
            .await
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?,
    };

    // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_problem(req: HttpRequest) -> Result<HttpResponse> {
    // problem/id
    // extract id from request

    match req
        .match_info()
        .get("problemId")
        .and_then(|id| id.parse::<i32>().ok())
    {
        Some(problem_id) => {
            // Successfully parsed problem_id as an integer, use it as needed

            let template = serve_types::ProblemTemplate {
                logged_in: mock::is_logged_in(),
                problem: db::get_one_problem(problem_id)
                    .await
                    .map_err(|err| actix_web::error::ErrorInternalServerError(err))?,
            };

            // Render the template and return as an HTTP response
            let rendered = template
                .render()
                .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

            Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
        }
        None => {
            // Handle the case where problemId is missing or invalid
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body("Bad request"))
        }
    }
}

pub async fn serve_leaderboard() -> Result<HttpResponse> {
    let template = serve_types::LeaderboardTemplate {
        logged_in: mock::is_logged_in(),
        users: db::get_leaderboard_users()
            .await
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?,
    };

    // Render the template and return as an HTTP response
    let rendered = template.render().map_err(|_| {
        actix_web::error::ErrorInternalServerError("serve_leaderboard : Template error")
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_profile() -> Result<HttpResponse> {
    // Create the template instance with dynamic data
    let template = serve_types::ProfileTemplate {
        logged_in: mock::is_logged_in(),
        user: db::get_user_profile(1)
            .await
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?,
    };

    // // Render the template and return as an HTTP response
    let rendered = template.render().map_err(|_| {
        actix_web::error::ErrorInternalServerError("serve_profile : Template error")
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_sign_in() -> Result<HttpResponse> {
    // Create the template instance with dynamic data

    // Render the template and return as an HTTP response
    // let rendered = template
    //     .render()
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("serve_sign_in: Template error"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("rendered"))
}

// apis without html

pub async fn check_answer(req: HttpRequest) -> Result<HttpResponse> {
    // problem/{problemId}/{answer}
    // extract id from request

    // get problemId and answer from request
    let problem_id = req
        .match_info()
        .get("problemId")
        .and_then(|id| id.parse::<i32>().ok());

    let ans = req.match_info().get("answer").map(|s| s.to_string());

    match (problem_id, ans) {
        (Some(problem_id), Some(ans)) => {
            // Successfully parsed problem_id as an integer, use it as needed

            let result = db::check_answer(problem_id, ans)
                .await
                .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

            Ok(HttpResponse::Ok().json(result))
        }
        _ => {
            // Handle the case where problemId is missing or invalid
            Ok(HttpResponse::Ok().json("Bad request"))
        }
    }
}
