// use crate::config::env as E;
// use crate::logger::logger::_log_error;
// use crate::storage::db::types::pool::DbPool;
// use crate::storage::redis::utils::{get_pub_sub_redis_connection, get_redis_connection};
// use crate::storage::{initialize_db_pool, initialize_replica_db_pool};
// use crate::utils::utils as U;
// use lru::LruCache;
// use openssl::{
//   pkey::{PKey, Private, Public},
//   x509::X509,
// };
// use redis::aio::MultiplexedConnection;
// use reqwest::{header, Client};
// use std::collections::HashMap;
// use std::fs;
// use std::io::Read;
// use std::num::NonZeroUsize;
// use std::sync::Arc;
// use tokio::sync::oneshot as Oneshot;
use tokio::sync::OnceCell;
// use tokio::sync::RwLock as TokioRwLock;
// use tokio::time::Duration;
// use uuid::Uuid;

use crate::storage::conn;

#[derive(Debug)]
pub struct GlobalState {
    pub db_pool: conn::DbPool,
}

impl GlobalState {
    pub async fn new() -> GlobalState {
        GlobalState {
            db_pool: conn::initialize_db_pool().clone(),
        }
    }
}

pub async fn get_global_state() -> &'static GlobalState {
    static GLOBAL_STATE: OnceCell<GlobalState> = OnceCell::const_new();
    GLOBAL_STATE.get_or_init(|| GlobalState::new()).await
}
