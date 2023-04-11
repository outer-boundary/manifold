use crate::Error;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn connect_db(database_url: &str) -> Result<MySqlPool, Error> {
    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    Ok(pool)
}
