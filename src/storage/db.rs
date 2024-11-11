// src/db.rs

use crate::global_state as GS;
use crate::model;
use crate::schema::attempted_problems::dsl::*;
use crate::schema::problems::dsl::{active as p_active, *};
use crate::schema::users::dsl::{active as u_active, *};
use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::result::Error;

// model::Problem queries
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------

pub async fn get_all_problems() -> Result<Vec<model::Problem>, Error> {
    let state = GS::get_global_state().await;

    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    // problems load with active true status
    problems
        .filter(p_active.eq(true))
        .load::<model::Problem>(&mut conn)
}

pub async fn get_problem_by_id(problem_id_: i32) -> Result<model::Problem, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    problems
        .find(problem_id_)
        .filter(p_active.eq(true))
        .first::<model::Problem>(&mut conn)
}

pub async fn check_answer(problem_id_: i32, answer_: String) -> Result<bool, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    let problem = problems
        .find(problem_id_)
        .filter(p_active.eq(true))
        .first::<model::Problem>(&mut conn)?;
    Ok(problem.answer.trim() == answer_.trim())
}

pub async fn update_problem_solved_count(problem_id_: i32) -> Result<(), Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    diesel::update(problems.find(problem_id_))
        .set(solved_count.eq(solved_count + 1))
        .execute(&mut conn)?;

    Ok(())
}

// model::User queries
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------

pub async fn get_user_profile(email_: String) -> Result<Option<model::User>, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    let user = users
        .filter(email.eq(email_))
        .filter(u_active.eq(true))
        .first::<model::User>(&mut conn)
        .optional()?;

    return Ok(user);
}

pub async fn get_leaderboard_users() -> Result<Vec<model::User>, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");
    users
        .filter(u_active.eq(true))
        .filter(solved.gt(0))
        .load::<model::User>(&mut conn)
}

pub async fn get_or_create_user(email_: &str, username_: &str) -> Result<model::User, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    let user = users
        .filter(email.eq(email_))
        .first::<model::User>(&mut conn)
        .optional()?;

    match user {
        Some(user) => Ok(user),
        None => {
            let new_user = model::NewUser {
                username: &username_,
                email: &email_,
            };
            let inserted_user = diesel::insert_into(users)
                .values(&new_user)
                .get_result(&mut conn)?;

            Ok(inserted_user)
        }
    }
}

pub async fn update_user_solved_count(email_: &str) -> Result<(), Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    diesel::update(users.filter(email.eq(email_)))
        .set(solved.eq(solved + 1))
        .execute(&mut conn)?;

    Ok(())
}

// model::AttemptedProblem queries
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------
// ---------------------------------------------------------------------

pub async fn insert_attempted_problems(
    email_: &str,
    problem_id_: i32,
    is_solved_: bool,
) -> Result<(), Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    let new_attempted_problem = model::NewAttempt {
        user_email: email_,
        problem_id: problem_id_,
        is_solved: is_solved_,
    };

    diesel::insert_into(attempted_problems)
        .values(&new_attempted_problem)
        .execute(&mut conn)?;

    Ok(())
}

pub async fn check_already_solved(email_: &str, problem_id_: i32) -> Result<bool, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    let attempted_problem = attempted_problems
        .filter(user_email.eq(email_))
        .filter(problem_id.eq(problem_id_))
        .filter(is_solved.eq(true))
        .first::<model::AttemptedProblem>(&mut conn)
        .optional()?;

    match attempted_problem {
        Some(_attempted_problem) => Ok(true),
        None => Ok(false),
    }
}
