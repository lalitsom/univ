// src/db.rs

use crate::global_state as GS;
use crate::model;
use crate::schema::problems::dsl::*;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::result::Error;

// problem queries

pub async fn get_all_problems() -> Result<Vec<model::Problem>, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");
    problems.load::<model::Problem>(&mut conn)
}

pub async fn get_one_problem(problem_id: i32) -> Result<model::Problem, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");
    problems.find(problem_id).first(&mut conn)
}

// model::User queries

pub async fn get_user_profile(_email: String) -> Result<model::User, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    let user = users
        .filter(email.eq(_email))
        .first::<model::User>(&mut conn)
        .optional()?;

    Ok(user.unwrap())
}

pub async fn get_leaderboard_users() -> Result<Vec<model::User>, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");
    users.load::<model::User>(&mut conn)
}

pub async fn check_answer(problem_id: i32, ans: String) -> Result<bool, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");
    let problem = problems
        .find(problem_id)
        .first::<model::Problem>(&mut conn)?;
    Ok(problem.answer.parse::<i32>() == ans.parse::<i32>())
}

pub async fn get_or_create_user(_email: &str, _username: &str) -> Result<model::User, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");

    println!("email: {}, username: {}", _email, _username);

    let user = users
        .filter(email.eq(_email))
        .first::<model::User>(&mut conn)
        .optional()?;

    // let temp = users
    // .filter(email.eq(_email)).limit(1);

    // // let sql = debug_query::<Pg, _>(&user).to_string();
    // // println!("Generated SQL: {}", sql);
    // let sql = debug_query::<Pg, _>(&temp).to_string();
    // println!("Generated SQL: {}", sql);

    match user {
        Some(user) => Ok(user),
        None => {
            let new_user = model::NewUser {
                username: &_username,
                email: &_email,
            };
            let inserted_user = diesel::insert_into(users)
                .values(&new_user)
                .get_result(&mut conn)?;

            Ok(inserted_user)
        }
    }
}
