use super::auth::login_identity::{add_login_identity, delete_all_login_identities};
use crate::models::users::*;
use color_eyre::{eyre::eyre, Result};
use sqlx::MySqlPool;
use uuid::Uuid;

#[tracing::instrument(skip(db_pool))]
pub async fn get_user(id: Uuid, db_pool: &MySqlPool) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "SELECT id AS `id: Uuid`, username, account_role AS `account_role: AccountRole`, created_at, updated_at FROM users WHERE id = ?",
        id
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(user)
}

#[tracing::instrument(skip(db_pool))]
pub async fn get_users(db_pool: &MySqlPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        "SELECT id AS `id: Uuid`, username, account_role AS `account_role: AccountRole`, created_at, updated_at FROM users ORDER BY id"
    )
    .fetch_all(db_pool)
    .await?;

    Ok(users)
}

#[tracing::instrument(skip(db_pool))]
pub async fn add_user(new_user: NewUser, db_pool: &MySqlPool) -> Result<User> {
    let id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, username, account_role) VALUES (?, ?, ?)",
        id,
        new_user.username,
        new_user.account_role.unwrap_or(AccountRole::User)
    )
    .execute(db_pool)
    .await?;

    match add_login_identity(id, new_user.identity, db_pool).await {
        Ok(_) => match get_user(id, db_pool).await? {
            Some(user) => Ok(user),
            None => Err(eyre!(
                "Could not get user with id '{}' after creating them",
                id
            )),
        },
        Err(err) => {
            // If adding the login identity failed, try to roll back creating the user.
            // TODO: Handle automatically rolling this back with transactions instead.
            tracing::error!(
                "Unable to create login identity for user with id '{}'. {}",
                id,
                err
            );

            delete_user(id, db_pool).await?;

            Err(err)
        }
    }
}

#[tracing::instrument(skip(db_pool))]
pub async fn delete_user(id: Uuid, db_pool: &MySqlPool) -> Result<()> {
    // Delete all of a user's login identities before deleting the actual user.
    delete_all_login_identities(id, db_pool).await?;

    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(db_pool)
        .await?;

    Ok(())
}
