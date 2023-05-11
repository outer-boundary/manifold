use sqlx::MySqlPool;

pub type Error = Box<dyn std::error::Error>;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}
