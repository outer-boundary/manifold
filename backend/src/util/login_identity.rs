use crate::models::login_identity::*;
use color_eyre::Result;
use futures::prelude::*;
use sqlx::MySqlPool;
use uuid::Uuid;

use super::auth::password::hash_password;

#[tracing::instrument(skip(db_pool))]
pub async fn get_login_identity(
    user_id: Uuid,
    li_type: LoginIdentityType,
    db_pool: &MySqlPool,
) -> Result<Option<LoginIdentity>> {
    let li = match li_type {
        LoginIdentityType::EmailPassword => {
            let li = sqlx::query_as!(
                LIEmailPassword,
                "SELECT user_id AS `user_id: Uuid`, email, password_hash, salt, created_at, updated_at FROM login_identity__email_password WHERE user_id = ?",
                user_id
            )
            .fetch_optional(db_pool)
            .await?;

            li.map(LoginIdentity::EmailPassword)
        }
    };

    Ok(li)
}

#[tracing::instrument(skip(db_pool))]
pub async fn get_login_identities(
    user_id: Uuid,
    db_pool: &MySqlPool,
) -> Result<Vec<LoginIdentity>> {
    let all_li: Vec<LoginIdentity> = futures::stream::iter(LoginIdentityType::all())
        .then(|li_type| async move { get_login_identity(user_id, li_type.clone(), db_pool).await })
        .try_collect::<Vec<Option<LoginIdentity>>>()
        .await?
        .into_iter()
        .flatten()
        .collect();

    Ok(all_li)
}

#[tracing::instrument(skip(db_pool))]
pub async fn add_login_identity(
    user_id: Uuid,
    new_li: NewLoginIdentity,
    db_pool: &MySqlPool,
) -> Result<()> {
    match new_li {
        NewLoginIdentity::EmailPassword(li) => {
            let (password_hash, salt) = hash_password(li.password).await?;
            let salt = hex::encode(salt.as_bytes());

            sqlx::query_as!(
                LIEmailPassword,
                "INSERT INTO login_identity__email_password (user_id, email, password_hash, salt) VALUES (?, ?, ?, ?)",
                user_id,
                li.email,
                password_hash,
                salt
            )
            .fetch_optional(db_pool)
            .await?;
        }
    };

    Ok(())
}

#[tracing::instrument(skip(db_pool))]
pub async fn delete_login_identity(
    user_id: Uuid,
    li_type: LoginIdentityType,
    db_pool: &MySqlPool,
) -> Result<()> {
    match li_type {
        LoginIdentityType::EmailPassword => {
            sqlx::query!(
                "DELETE FROM login_identity__email_password WHERE user_id = ?",
                user_id
            )
            .execute(db_pool)
            .await?;
        }
    }

    Ok(())
}

#[tracing::instrument(skip(db_pool))]
pub async fn delete_all_login_identities(
    user_id: Uuid,
    li_type: LoginIdentityType,
    db_pool: &MySqlPool,
) -> Result<()> {
    futures::stream::iter(LoginIdentityType::all())
        .then(
            |li_type| async move { delete_login_identity(user_id, li_type.clone(), db_pool).await },
        )
        .try_collect()
        .await?;

    Ok(())
}
