use actix_web::HttpResponse;
use sqlx::MySqlPool;
use uuid::Uuid;
use color_eyre::{Result, eyre::eyre};

use crate::models::domains::*;

#[tracing::instrument(skip(db_pool))]
pub async fn get_domain(id: Uuid, db_pool: &MySqlPool) -> Result<Option<Domain>> {
    let domain = sqlx::query_as!(
        Domain,
        "SELECT id AS `id: Uuid`, display_name AS name, description_text AS description, banner_url, icon_url, created_at, updated_at FROM domains WHERE id = ?",
        id
    ).fetch_optional(db_pool).await?;

    Ok(domain)
}

#[tracing::instrument(skip(db_pool))]
pub async fn get_all_user_memberships(user_id: Uuid, db_pool: &MySqlPool) -> Result<Vec<DomainMembership>> {
    let membership = sqlx::query_as!(
        DomainMembership,
        "SELECT domain_id AS `domain_id: Uuid`, user_id AS `user_id: Uuid`, role_name, created_at, updated_at FROM domain_memberships WHERE user_id = ?",
        user_id
    ).fetch_all(db_pool).await?;

    Ok(membership)
}

#[tracing::instrument(skip(db_pool))]
pub async fn get_all_domain_memberships(domain_id: Uuid, db_pool: &MySqlPool) -> Result<Vec<DomainMembership>> {
    let membership = sqlx::query_as!(
        DomainMembership,
        "SELECT domain_id AS `domain_id: Uuid`, user_id AS `user_id: Uuid`, role_name, created_at, updated_at FROM domain_memberships WHERE domain_id = ?",
        domain_id
    ).fetch_all(db_pool).await?;

    Ok(membership)
}

#[tracing::instrument(skip(db_pool))]
pub async fn add_domain(new_domain: NewDomain, current_user_id: Uuid, db_pool: &MySqlPool) -> Result<Domain> {
    // Generate domain's id
    let id = Uuid::new_v4();

    // Add the domain to the database
    let details = match new_domain.details {
        Some(details) => details,
        None => {
            UpdateDomain {
                description: None,
                banner_url: None,
                icon_url: None,
            }
        }
    };
    
    sqlx::query!(
        "INSERT INTO domains (id, display_name, description_text, icon_url, banner_url) VALUES (?, ?, ?, ?, ?)",
        id,
        new_domain.name,
        details.description,
        details.icon_url,
        details.banner_url,
    )
    .execute(db_pool)
    .await?;

    // Add the membership for the current user.
    let domain_membership = NewDomainMembership {
        domain_id: id,
        user_id: current_user_id,
        role_name: "owner".to_string(),
    };
    
    match add_domain_membership(domain_membership, db_pool).await {
        Ok(_) => match get_domain(id, db_pool).await? {
            Some(domain) => Ok(domain),
            None => Err(eyre!(
                "Could not get domain '{}'",
                id
            )),
        },
        Err(error) => {
            tracing::error!(
                "Unable to create membership for user '{}' in domain '{}'. {}",
                current_user_id,
                id,
                error
            );

            delete_domain(id, db_pool).await?;

            Err(error.into())
        }
    }
}

#[tracing::instrument(skip(db_pool))]
pub async fn add_domain_membership(domain_membership: NewDomainMembership, db_pool: &MySqlPool) -> Result<()> {
    // Add the domain membership to the database
    sqlx::query!(
        "INSERT INTO domain_memberships (domain_id, user_id, role_name) VALUES (?, ?, ?)",
        domain_membership.domain_id,
        domain_membership.user_id,
        domain_membership.role_name
    ).execute(db_pool).await?;

    Ok(())
}

#[tracing::instrument(skip(db_pool))]
pub async fn delete_all_domain_memberships(domain_id: Uuid, db_pool: &MySqlPool) -> Result<()> {
    // Delete the user.
    sqlx::query!("DELETE FROM domain_memberships WHERE domain_id = ?", domain_id)
        .execute(db_pool)
        .await?;

    Ok(())
}

#[tracing::instrument(skip(db_pool))]
pub async fn delete_all_user_memberships(user_id: Uuid, db_pool: &MySqlPool) -> Result<()> {
    // Delete the user.
    sqlx::query!("DELETE FROM domain_memberships WHERE user_id = ?", user_id)
        .execute(db_pool)
        .await?;

    Ok(())
}

#[tracing::instrument(skip(db_pool))]
pub async fn delete_membership(domain_id: Uuid, user_id: Uuid, db_pool: &MySqlPool) -> Result<()> {
    // Delete the user.
    sqlx::query!("DELETE FROM domain_memberships WHERE domain_id = ? AND user_id = ?", domain_id, user_id)
        .execute(db_pool)
        .await?;

    Ok(())
}

#[tracing::instrument(skip(db_pool))]
pub async fn delete_domain(id: Uuid, db_pool: &MySqlPool) -> Result<()> {
    delete_all_domain_memberships(id, db_pool).await?;

    sqlx::query!(
        "DELETE FROM domains WHERE id = ?",
        id
    ).execute(db_pool).await?;

    Ok(())
}