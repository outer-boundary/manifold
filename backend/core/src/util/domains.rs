use crate::models::domains::*;

#[tracing::instrument(skip(db_pool))]
pub async fn create_domain(id: Uuid, db_pool: &MySqlPool) -> Result<Option<Domain>> {
  
    let user = sqlx::query_as!(
        User,
        "SELECT id AS `id: Uuid`, username, account_role AS `account_role: AccountRole`, created_at, updated_at FROM users WHERE id = ?",
        id
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(user)
}