use actix_web::{web, HttpResponse, post};
use sqlx::MySqlPool;

use crate::{models::{domains::{NewDomain, NewDomainMembership}, users::CurrentUser}, util::domains::{add_domain, add_domain_membership}};

pub fn domains_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(add_domain_route);
}

#[tracing::instrument(skip(db_pool))]
#[post("")]
async fn add_domain_route(
    db_pool: web::Data<MySqlPool>,
    domain: web::Json<NewDomain>,
    current_user: CurrentUser, 
) -> HttpResponse {
    tracing::debug!("Creating a new domain...");

    let domain = add_domain(domain.into_inner(), current_user.user.id, &db_pool).await;

    match domain {
        Ok(domain) => {
            tracing::info!(
                "Created domain '{}'",
                domain.id
            );

            HttpResponse::Created().json(domain)
        },
        Err(error) => {
            let error_str = format!("Error creating domain. {}", error);

            tracing::error!(error_str);
            HttpResponse::InternalServerError().json(error_str)
        }
    }
}

#[tracing::instrument(skip(db_pool))]
#[post("")]
async fn add_membership_route(
    db_pool: web::Data<MySqlPool>,
    membership: web::Json<NewDomainMembership>,
) -> HttpResponse {
    tracing::debug!("Adding a membership for user '{}' in domain '{}'...", membership.user_id, membership.domain_id);

    let added_membership = add_domain_membership(membership.clone(), &db_pool).await;

    match added_membership {
        Ok(_) => {
            tracing::info!(
                "Created membership for user '{}' in domain '{}'",
                membership.user_id,
                membership.domain_id
            );

            HttpResponse::NoContent().finish()
        },
        Err(error) => {
            let error_str = format!(
                "Unable to create membership for user '{}' in domain '{}'. {}",
                membership.user_id,
                membership.domain_id,
                error
            );

            tracing::error!(error_str);

            HttpResponse::InternalServerError().json(error_str)
        }
    }
}
