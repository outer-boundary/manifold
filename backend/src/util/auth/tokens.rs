use color_eyre::{eyre::eyre, Result};
use deadpool_redis::redis::AsyncCommands;
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    keys::SymmetricKey,
    local,
    token::UntrustedToken,
    version4::V4,
    Local,
};
use rand::{rngs::OsRng, RngCore};
use uuid::Uuid;

use crate::{
    types::{redis::RedisConnection, tokens::ConfirmationToken},
    util::configuration::get_config,
};

const SESSION_KEY_PREFIX: &str = "MANIFOLD";
const PASSWORD_CHANGE_KEY_PREFIX: &str = "PWD_CHG";

#[tracing::instrument(skip(redis))]
pub async fn issue_confirmation_token(
    user_id: Uuid,
    redis: &mut RedisConnection,
    is_for_password_change: bool,
) -> Result<(String, i64)> {
    let session_key: String = {
        let mut buff = [0u8; 128];
        OsRng.fill_bytes(&mut buff);
        hex::encode(buff)
    };

    let redis_key = if is_for_password_change {
        format!(
            "{}_{}_{}",
            SESSION_KEY_PREFIX, PASSWORD_CHANGE_KEY_PREFIX, session_key
        )
    } else {
        format!("{}_{}", SESSION_KEY_PREFIX, session_key)
    };

    redis
        .set(redis_key.clone(), String::new())
        .await
        .map_err(|err| {
            tracing::error!("Redis error (set): {}", err);
            err
        })?;

    let config = get_config()?;
    let current_date_time = chrono::Utc::now();

    let ttl = if is_for_password_change {
        60
    } else {
        config.secret.token_expiration
    };
    let time_to_live = chrono::Duration::minutes(ttl);

    let dt = current_date_time + time_to_live;

    redis
        .expire(redis_key.clone(), time_to_live.num_seconds().try_into()?)
        .await
        .map_err(|err| {
            tracing::error!("Redis error (expiry): {}", err);
            err
        })?;

    let mut claims = Claims::new()?;
    claims.expiration(&dt.to_rfc3339())?;
    claims.add_additional("user_id", serde_json::json!(user_id))?;
    claims.add_additional("session_key", serde_json::json!(session_key))?;

    let sk = SymmetricKey::<V4>::from(config.secret.secret_key.as_bytes())?;
    Ok((
        local::encrypt(
            &sk,
            &claims,
            None,
            Some(config.secret.hmac_secret.as_bytes()),
        )?,
        ttl,
    ))
}

#[tracing::instrument(skip(redis))]
pub async fn verify_confirmation_token(
    token: String,
    redis: &mut RedisConnection,
    is_password: bool,
) -> Result<ConfirmationToken> {
    let config = get_config()?;
    let sk = SymmetricKey::<V4>::from(config.secret.secret_key.as_bytes())?;

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&token)?;
    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(config.secret.hmac_secret.as_bytes()),
    )?;
    let claims = trusted_token
        .payload_claims()
        .ok_or(eyre!("No claims found"))?;

    let uid = serde_json::to_value(
        claims
            .get_claim("user_id")
            .ok_or(eyre!("No claim with key 'user_id'"))?,
    )?;

    let uuid_string = serde_json::from_value::<String>(uid)?;
    let user_id = uuid::Uuid::parse_str(&uuid_string)?;

    let sss_key = serde_json::to_value(
        claims
            .get_claim("session_key")
            .ok_or(eyre!("No claim with key 'session_key'"))?,
    )?;
    let session_key = serde_json::from_value::<String>(sss_key)?;

    let redis_key = if is_password {
        format!(
            "{}_{}_{}",
            SESSION_KEY_PREFIX, PASSWORD_CHANGE_KEY_PREFIX, session_key
        )
    } else {
        format!("{}_{}", SESSION_KEY_PREFIX, session_key)
    };

    if redis
        .get::<_, Option<String>>(redis_key.clone())
        .await?
        .is_none()
    {
        return Err(eyre!("Could not find redis key"));
    };

    redis.del(redis_key.clone()).await?;
    Ok(ConfirmationToken { user_id })
}
