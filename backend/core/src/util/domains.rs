use sqlx::MySqlPool;
use uuid::Uuid;
use color_eyre::{eyre::eyre, Result};

use crate::models::domains::*;

#[tracing::instrument(skip(db_pool))]
pub async fn add_domain(user_id: Uuid, domain: NewDomain, db_pool: &MySqlPool) -> Result<Domain> {
    let id = Uuid::new_v4();
  
    sqlx::query!(
        "INSERT INTO domains (id, display_name, description_text, icon_url, banner_url) VALUES (?, ?, ?, ?, ?)",
        id,
        domain.display_name,
        domain.description_text.unwrap_or_else(|| String::from("")),
        domain.icon_url.unwrap_or_else(|| String::from("")),
        domain.banner_url.unwrap_or_else(|| String::from("")),
    )
    .execute(db_pool)
    .await?;

    add_domain_membership(DomainMembership { domain_id: id, user_id, role_name: String::from("owner") }, &db_pool)
    .await?;

    match get_domain(id, db_pool).await? {
        Some(domain) => Ok(domain),
        None => Err(eyre!(
            "Could not get domain with id '{}' after creating them",
            id
        )),
    }
}

#[tracing::instrument(skip(db_pool))]
pub async fn get_domain(id: Uuid, db_pool: &MySqlPool) -> Result<Option<Domain>> {
    let domain = sqlx::query_as!(
        Domain,
        "SELECT id AS `id: Uuid`, display_name, description_text, icon_url, banner_url, created_at FROM domains WHERE id = ?",
        id
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(domain)
}

#[tracing::instrument(skip(db_pool))]
pub async fn add_domain_membership(membership: DomainMembership, db_pool: &MySqlPool) -> Result<()> {
    sqlx::query_as!(
        DomainMembership,
        "INSERT INTO domain_memberships (domain_id, user_id, role_name) VALUES (?, ?, ?)",
        membership.domain_id, 
        membership.user_id,
        membership.role_name
    )
    .execute(db_pool)
    .await?;

    Ok(())
}