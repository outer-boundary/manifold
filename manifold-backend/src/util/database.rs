use crate::Error;
use sqlx::{
    mysql::{MySql, MySqlPoolOptions},
    Pool,
};

pub type DBPool = Pool<MySql>;

pub async fn connect_db() -> Result<DBPool, Error> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
