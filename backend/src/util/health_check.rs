use deadpool_redis::{
    redis::{cmd, Value},
    Pool,
};
use sqlx::MySqlPool;

use crate::common::MFResult;

pub async fn database_connection_check(pool: &MySqlPool) -> MFResult<()> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|err| err.into())
}

pub async fn redis_connection_check(pool: &Pool) -> MFResult<()> {
    let mut conn = pool.get().await?;

    cmd("SET")
        .arg(&["connection_check", "success"])
        .query_async(&mut conn)
        .await?;

    let get_value: Value = cmd("GET")
        .arg(&["connection_check"])
        .query_async(&mut conn)
        .await?;

    if get_value != Value::Data("success".into()) {
        return Err("Unexpected redis value returned".into());
    }

    Ok(())
}
