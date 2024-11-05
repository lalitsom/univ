use crate::oauth;
use crate::serve_types;
use crate::storage::db;
use actix_files::NamedFile;
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use askama::Template;
use uuid::Uuid;

// Serve static files like CSS
pub async fn serve_static_file(path: web::Path<String>) -> Result<NamedFile> {
    NamedFile::open(format!("templates/static/{}", path.into_inner()))
        .map_err(|_| actix_web::error::ErrorNotFound("File not found"))
}

pub async fn serve_home(session: Session) -> Result<HttpResponse> {
    let user_token_exists = session
        .get::<String>("user_token")
        .unwrap_or(None)
        .is_some();

    // Create the template instance with dynamic data
    let template = serve_types::HomeTemplate {
        logged_in: user_token_exists,
    };

    // Render the template and return as an HTTP response
    let rendered = template
        .render()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_problems(session: Session) -> Result<HttpResponse> {
    let user_token_exists = session
        .get::<String>("user_token")
        .unwrap_or(None)
        .is_some();
    let template = serve_types::ProblemsTemplate {
        logged_in: user_token_exists,
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

pub async fn serve_problem(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    // problem/id
    // extract id from request

    let user_token_exists = session
        .get::<String>("user_token")
        .unwrap_or(None)
        .is_some();

    match req
        .match_info()
        .get("problemId")
        .and_then(|id| id.parse::<i32>().ok())
    {
        Some(problem_id) => {
            // Successfully parsed problem_id as an integer, use it as needed
            let mut already_solved = false;
            if user_token_exists {
                let user_email_: String = session.get::<String>("user_email").unwrap().unwrap();
                already_solved = db::check_already_solved(&user_email_, problem_id)
                    .await
                    .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
            }

            let template = serve_types::ProblemTemplate {
                logged_in: user_token_exists,
                problem: db::get_problem_by_id(problem_id)
                    .await
                    .map_err(|err| actix_web::error::ErrorInternalServerError(err))?,
                already_solved: already_solved,
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

pub async fn serve_leaderboard(session: Session) -> Result<HttpResponse> {
    let user_token_exists = session
        .get::<String>("user_token")
        .unwrap_or(None)
        .is_some();
    let template = serve_types::LeaderboardTemplate {
        logged_in: user_token_exists,
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

pub async fn serve_profile(session: Session) -> Result<HttpResponse> {
    // Create the template instance with dynamic data

    let user_token_exists = session
        .get::<String>("user_token")
        .unwrap_or(None)
        .is_some();

    if !user_token_exists {
        return Ok(HttpResponse::Found()
            .insert_header(("Location", "/sign_in"))
            .finish());
    }

    let user_email_ = session.get::<String>("user_email").unwrap().unwrap();
    let user_ = db::get_user_profile(user_email_)
        .await.unwrap();
    
    if user_.is_none() {
        return Ok(HttpResponse::Found()
            .insert_header(("Location", "/sign_out"))
            .finish());
    }
    
    
    let template = serve_types::ProfileTemplate {
        logged_in: user_token_exists,
        user: user_.unwrap(),
    };

    // // Render the template and return as an HTTP response
    let rendered = template.render().map_err(|_| {
        actix_web::error::ErrorInternalServerError("serve_profile : Template error")
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn serve_sign_in() -> impl Responder {
    let auth_url = oauth::get_authorize_url();

    // Redirect the user to Google OAuth
    HttpResponse::Found()
        .insert_header(("Location", auth_url))
        .finish()
}

pub async fn serve_sign_out(session: Session) -> impl Responder {
    session.clear();
    // Redirect the user to Google OAuth
    HttpResponse::Found()
        .insert_header(("Location", "/"))
        .finish()
}

// apis without html

pub async fn oauth2callback(
    session: Session,
    query: web::Query<oauth::OAuthRequest>,
) -> impl Responder {
    // Get the code from the query string from req

    let result = oauth::handle_oauth2callback(query.code.clone()).await;

    match result {
        Ok(user_info) => {
            let email = user_info["email"].as_str().unwrap();
            // get username from email by splitting from @
            let user_name: &str = email.split('@').collect::<Vec<&str>>()[0];

            // let org = user_info["hd"].as_str().unwrap();
            // if org != "juspay.in" {
            //     return HttpResponse::Found()
            //     .insert_header(("Location", "/"))
            //     .finish();
            // }

            let user = db::get_or_create_user(email, user_name)
                .await
                .map_err(|err| actix_web::error::ErrorInternalServerError(err));

            let email = user.as_ref().unwrap().email.clone();

            let user_token = Uuid::new_v4().to_string();
            session.insert("user_token", &user_token).unwrap();
            session.insert("user_email", &email).unwrap();

            // Redirect to profile page
            return HttpResponse::Found()
                .insert_header(("Location", "/profile"))
                .finish();
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return HttpResponse::Found()
                .insert_header(("Location", "/"))
                .finish();
        }
    }
}

pub async fn check_answer(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    // problem/{problemId}/{answer}
    // extract id from request

    let user_token_exists = session
        .get::<String>("user_token")
        .unwrap_or(None)
        .is_some();

    if !user_token_exists {
        return Ok(HttpResponse::Ok().json("Unauthorized"));
    }

    let user_email_ = session.get::<String>("user_email").unwrap().unwrap();

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

            let already_solved = db::check_already_solved(&user_email_, problem_id)
                .await
                .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

            let _ = db::insert_attempted_problems(&user_email_, problem_id, result).await;
            if result && !already_solved {
                let _ = db::update_user_solved_count(&user_email_).await;
                let _ = db::update_problem_solved_count(problem_id).await;
            }

            Ok(HttpResponse::Ok().json(result))
        }
        _ => {
            // Handle the case where problemId is missing or invalid
            Ok(HttpResponse::Ok().json("Bad request"))
        }
    }
}
