// src/db.rs

use diesel::{r2d2, PgConnection};
use std::env;

// Short-hand for the database pool type to use throughout the app.
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;


pub fn initialize_db_pool()  -> DbPool {
    let conn_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_string);
    r2d2::Pool::builder()
      .build(manager)
      .expect("database URL should be valid path to SQLite DB file")
  }