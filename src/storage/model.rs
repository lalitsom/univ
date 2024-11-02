// src/models.rs

use crate::schema::{attempted_problems, problems, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    pub user_id: i32,
    pub problemid: i32,
    pub is_solved: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Problem {
    // Method to get hint as a String with a default if None
    pub fn hint_or_default(&self) -> &str {
        self.hint.as_deref().unwrap_or("No hint available")
    }
}
