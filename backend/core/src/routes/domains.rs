use crate::{
  models::{error::ErrorResponse, users::CurrentUser, domains::{NewDomain, DomainMembership}},
  util::domains::{add_domain, get_domain, get_domain_memberships, add_domain_membership, get_user_domains},
};
use actix_web::{post, get, web, HttpResponse};
use sqlx::MySqlPool;
use uuid::Uuid;

pub fn domains_scope(cfg: &mut web::ServiceConfig) {
  cfg.service(add_domain_route);
}

#[tracing::instrument(skip(db_pool))]
#[post("")]
async fn add_domain_route(
  db_pool: web::Data<MySqlPool>,
  new_domain: web::Json<NewDomain>,
  current_user: CurrentUser,
) -> HttpResponse {
  tracing::debug!("Creating new domain...");

  let domain = add_domain(current_user.0.id, new_domain.into_inner(), &db_pool).await;

  match domain {
    Ok(domain) => {
      tracing::info!("Returning created domain.");
        HttpResponse::Ok().json(domain)
    }
    Err(err) => {
      let err_string = format!("Failed to create a domain. {}", err);
      tracing::error!(err_string);
      HttpResponse::InternalServerError().json(
      ErrorResponse::new(0, err_string)
          .description("Unexpected value returned"),
      )
    }
  }
}

#[tracing::instrument(skip(db_pool))]
#[get("/{domain_id}")]
async fn get_domain_route(db_pool: web::Data<MySqlPool>, domain_id: web::Path<Uuid>) -> HttpResponse {
  tracing::debug!("Getting domain with id '{}'...", domain_id);

  let domain_id = domain_id.into_inner();
  let domain = get_domain(domain_id, &db_pool).await;

  match domain {
    Ok(Some(domain)) => {
      tracing::info!("Returning domain.");
        HttpResponse::Ok().json(domain)
    }
    Err(err) => {
      let err_string = format!("Failed to get a domain with id '{}'. {}", domain_id, err);
      tracing::error!(err_string);
      HttpResponse::InternalServerError().json(
      ErrorResponse::new(0,err_string)
          .description(err),
      )
    }
    _ => {
      let err_string = format!("Failed to get a domain with id '{}'.", domain_id);
      tracing::error!(err_string);
      HttpResponse::InternalServerError().json(
      ErrorResponse::new(0, err_string)
          .description("Unexpected value returned"),
      )
    }
  }
}

#[tracing::instrument(skip(db_pool))]
#[get("/user/{user_id}")]
async fn get_user_domains_route(db_pool: web::Data<MySqlPool>, user_id: web::Path<Uuid>) -> HttpResponse {
  tracing::debug!("Getting all domains for user with id {}...", user_id);

  let user_id = user_id.into_inner();
  let domains = get_user_domains(user_id, &db_pool).await;

  match domains {
    Ok(domains) => {
      tracing::info!("Returning domains.");
        HttpResponse::Ok().json(domains)
    }
    Err(err) => {
      let err_string = format!("Failed to get domains that user with id '{}' is a member of. {}", user_id, err);
      tracing::error!(err_string);
      HttpResponse::InternalServerError().json(
      ErrorResponse::new(0,err_string)
          .description(err),
      )
    }
    _ => {
      let err_string = format!("Failed to get domains that user with id '{}' is a member of.", user_id);
      tracing::error!(err_string);
      HttpResponse::InternalServerError().json(
      ErrorResponse::new(0, err_string)
          .description("Unexpected value returned"),
      )
    }
  }
}

#[tracing::instrument(skip(db_pool))]
#[post("/memberships")]
async fn add_domain_membership_route(
  db_pool: web::Data<MySqlPool>,
  membership: web::Json<DomainMembership>,
) -> HttpResponse {
  tracing::debug!("Creating new domain membership...");

  let new_membership = membership.into_inner();
  let result = add_domain_membership(new_membership.clone(), &db_pool).await;

  match result {
    Ok(()) => {
      tracing::info!("Returning the domain membership.");
      HttpResponse::Ok().json(new_membership)
    }
    Err(err) => {
      let err_string = format!("Failed to create domain membership for domain with id '{}'. {}", new_membership.domain_id, err);
      tracing::error!(err_string);
      HttpResponse::InternalServerError().json(
      ErrorResponse::new(0, err_string)
          .description("Unexpected value returned"),
      )
    }
  }
}

#[tracing::instrument(skip(db_pool))]
#[get("/memberships/{domain_id}")]
async fn get_domain_memberships_route(db_pool: web::Data<MySqlPool>, domain_id: web::Path<Uuid>) -> HttpResponse {
  tracing::debug!("Getting all memberships for domain with id {}...", domain_id);

  let domain_id = domain_id.into_inner();
  let memberships = get_domain_memberships(domain_id, &db_pool).await;

  match memberships {
    Ok(memberships) => {
      tracing::info!("Returning the domain memberships.");
        HttpResponse::Ok().json(memberships)
    }
    Err(err) => {
      let err_string = format!("Failed to get all memberships for domain with id '{}'. {}", domain_id, err);
      tracing::error!(err_string);
      HttpResponse::InternalServerError().json(
      ErrorResponse::new(0, err_string)
          .description(err),
      )
    }
  }
}