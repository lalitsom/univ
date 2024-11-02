use tokio::sync::OnceCell;
use crate::storage::conn;
use crate::oauth;
use oauth2::basic::BasicClient;

#[derive(Debug)]
pub struct GlobalState {
    pub db_pool: conn::DbPool,
    pub oauth_client: BasicClient
}

impl GlobalState {
    pub async fn new() -> GlobalState {
        GlobalState {
            db_pool: conn::initialize_db_pool().clone(),
            oauth_client: oauth::initialize_oauth_client()
        }
    }
}

pub async fn get_global_state() -> &'static GlobalState {
    static GLOBAL_STATE: OnceCell<GlobalState> = OnceCell::const_new();
    GLOBAL_STATE.get_or_init(|| GlobalState::new()).await
}
