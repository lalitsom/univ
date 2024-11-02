// src/db.rs

use crate::global_state as GS;
use crate::model::{Problem, User};
use crate::schema::problems::dsl::*;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

// problem queries

pub async fn get_all_problems() -> Result<Vec<Problem>, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");
    problems.load::<Problem>(&mut conn)
}

pub async fn get_one_problem(problem_id: i32) -> Result<Problem, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");
    problems.find(problem_id).first(&mut conn)
}

// User queries

pub async fn get_user_profile(user_id: i32) -> Result<User, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");
    users.find(user_id).first(&mut conn)
}

pub async fn get_leaderboard_users() -> Result<Vec<User>, Error> {
    let state = GS::get_global_state().await;
    let mut conn = state
        .db_pool
        .get()
        .expect("couldn't get db connection from pool");
    users.load::<User>(&mut conn)
}
