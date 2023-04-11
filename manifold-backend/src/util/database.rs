use crate::Error;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub struct DatabaseConnectionConfig {
    pub max_connections: u32,
}

pub async fn connect_db<C>(database_url: &str, config: C) -> Result<MySqlPool, Error>
where
    C: Into<Option<DatabaseConnectionConfig>>,
{
    let mut max_connections = 10;

    if let Some(config) = config.into() {
        max_connections = config.max_connections;
    }

    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await?;

    Ok(pool)
}
