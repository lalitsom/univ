// src/models.rs

use crate::schema::{attempted_problems, problems, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use diesel::Insertable;

#[derive(Queryable, Insertable, Serialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub solved: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    // exclude fields like `id` and `created_at` that have defaults
}




#[derive(Queryable, Insertable, Serialize, Debug)]
#[diesel(table_name = problems)]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub problem_statement: String,
    pub tags: String, // TEXT type for tags as defined in schema.rs
    pub difficulty: i32,
    pub hint: Option<String>,
    pub answer: String,
    pub solved_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Serialize, Debug)]
#[diesel(table_name = attempted_problems)]
pub struct AttemptedProblem {
    pub id: i32,
    pub user_email: String,
    pub problem_id: i32,
    pub is_solved: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Insertable)]
#[diesel(table_name = attempted_problems)]
pub struct NewAttempt<'a> {
    pub user_email: &'a str,
    pub problem_id: i32,
    pub is_solved: bool,
    // exclude fields like `id` and `created_at` that have defaults
}


impl Problem {
    // Method to get hint as a String with a default if None
    pub fn hint_or_default(&self) -> &str {
        self.hint.as_deref().unwrap_or("No hint available")
    }

    pub fn created_ts_show(&self) -> String {
        self.created_at.format("%Y-%m-%d").to_string()
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.split(",").map(|s| s.to_string()).collect()
    }
}


impl User {
    pub fn created_ts_show(&self) -> String {
        self.created_at.format("%Y-%m-%d").to_string()
    }
}