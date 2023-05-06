use crate::models::users::*;
use sqlx::{MySql, Pool};
use uuid::Uuid;

use crate::Error;

pub async fn get_users(db_pool: &Pool<MySql>) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as!(
        DbUser,
        "SELECT as_uuid(id) AS id, display_name, first_name, last_name FROM users ORDER BY id"
    )
    .fetch_all(db_pool)
    .await
    .map(|db_users| {
        db_users
            .iter()
            .map(|db_user| db_user.clone().into())
            .collect()
    })?;

    Ok(users)
}

pub async fn get_user(id: String, db_pool: &Pool<MySql>) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        DbUser,
        "SELECT as_uuid(id) AS id, display_name, first_name, last_name FROM users WHERE id = as_bin(?)",
        id
    )
    .fetch_optional(db_pool)
    .await
    .map(|db_user| db_user.map(|db_user| db_user.into()))?;

    Ok(user)
}

pub async fn add_user(new_user: NewUser, db_pool: &Pool<MySql>) -> Result<String, Error> {
    let user_id = Uuid::new_v4().to_string();
    sqlx::query!(
        "INSERT INTO users (id, display_name, first_name, last_name) VALUES (as_bin(?), ?, ?, ?)",
        user_id.clone(),
        new_user.display_name,
        new_user.first_name,
        new_user.last_name
    )
    .execute(db_pool)
    .await?;

    Ok(user_id)
}

pub async fn delete_user(id: String, db_pool: &Pool<MySql>) -> Result<(), Error> {
    sqlx::query!("DELETE FROM users WHERE id = as_bin(?)", id)
        .execute(db_pool)
        .await?;

    Ok(())
}
