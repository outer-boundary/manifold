use sqlx::MySqlPool;
use uuid::Uuid;
use color_eyre::Result;

use crate::models::domains::*;

#[tracing::instrument(skip(db_pool))]
pub async fn create_domain(domain: NewDomain, db_pool: &MySqlPool) -> Result<()> {
    let id = Uuid::new_v4();
  
    sqlx::query_as!(
        Domain,
        "INSERT INTO domains (id, display_name, description_text, icon_url, banner_url) VALUES (?, ?, ?, ?, ?)",
        id,
        domain.display_name,
        domain.description_text.unwrap_or_else(|| String::from("")),
        domain.icon_url.unwrap_or_else(|| String::from("")),
        domain.banner_url.unwrap_or_else(|| String::from("")),
    )
    .execute(db_pool)
    .await?;

    // add the user that created the domain as the owner

    Ok(())
}