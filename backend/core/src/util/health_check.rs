use crate::types::{db::DBPool, redis::RedisPool};
use color_eyre::{eyre::eyre, Result};
use deadpool_redis::redis::{cmd, Value};

pub async fn database_connection_check(db_pool: &DBPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .execute(db_pool)
        .await
        .map(|_| ())
        .map_err(|err| eyre!(err))
}

pub async fn redis_connection_check(db_pool: &RedisPool) -> Result<()> {
    let mut conn = db_pool.get().await?;

    cmd("SET")
        .arg(&["connection_check", "success"])
        .query_async(&mut conn)
        .await?;

    let get_value: Value = cmd("GET")
        .arg(&["connection_check"])
        .query_async(&mut conn)
        .await?;

    cmd("DEL")
        .arg(&["connection_check"])
        .query_async(&mut conn)
        .await?;

    if get_value != Value::Data("success".into()) {
        return Err(eyre!("Unexpected redis value returned"));
    }

    Ok(())
}
