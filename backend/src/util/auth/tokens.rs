use color_eyre::Result;
use deadpool_redis::redis::AsyncCommands;
use rand::{rngs::OsRng, RngCore};
use uuid::Uuid;

const SESSION_KEY_PREFIX: &str = "MANIFOLD";
const PASSWORD_CHANGE_KEY_PREFIX: &str = "PWD_CHG";

#[tracing::instrument(skip(redis))]
pub async fn issue_confirmation_token(
    user_id: Uuid,
    redis: &mut deadpool_redis::redis::aio::Connection,
    is_for_password_change: bool,
) -> Result<String> {
    let session_key: String = {
        let mut buff = [0u8; 128];
        OsRng.fill_bytes(&mut buff);
        hex::encode(buff)
    };

    let redis_key = match is_for_password_change {
        true => format!(
            "{}_{}_{}",
            SESSION_KEY_PREFIX, PASSWORD_CHANGE_KEY_PREFIX, session_key
        ),
        false => format!("{}_{}", SESSION_KEY_PREFIX, session_key),
    };

    redis
        .set(redis_key.clone(), String::new())
        .await
        .map_err(|err| {
            tracing::error!("Redis error (set): {}", err);
            err
        })?;

    let config = crate::util::configuration::get_config()?;
    let currect_date_time = chrono::Utc::now();

    Ok("".to_string())
}
