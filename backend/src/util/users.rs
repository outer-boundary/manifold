use crate::models::users::*;
use color_eyre::Result;
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn get_users(db_pool: &MySqlPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        "SELECT id AS `id: Uuid`, username, display_name, first_name, last_name, created_at, updated_at FROM users ORDER BY id"
    )
    .fetch_all(db_pool)
    .await?;

    Ok(users)
}

pub async fn get_user(id: Uuid, db_pool: &MySqlPool) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "SELECT id AS `id: Uuid`, username, display_name, first_name, last_name, created_at, updated_at FROM users WHERE id = ?",
        id
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(user)
}

pub async fn add_user(new_user: NewUser, db_pool: &MySqlPool) -> Result<Uuid> {
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, username, display_name) VALUES (?, ?, ?)",
        id,
        new_user.username,
        new_user.username
    )
    .execute(db_pool)
    .await?;

    Ok(id)
}

pub async fn delete_user(id: Uuid, db_pool: &MySqlPool) -> Result<()> {
    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(db_pool)
        .await?;

    Ok(())
}
