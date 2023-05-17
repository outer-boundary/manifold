use sqlx::MySqlPool;

pub type Error = Box<dyn std::error::Error>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: MySqlPool,
}
