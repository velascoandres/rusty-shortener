use crate::db::ConnectionPool;

pub struct AppState {
    pub db: ConnectionPool
}
