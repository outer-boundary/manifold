use crate::{
  models::{error::ErrorResponse, users::CurrentUser, domains::NewDomain},
  util::{domains::add_domain},
};
use actix_web::{post, web, HttpResponse};
use sqlx::MySqlPool;

pub fn domains_scope(cfg: &mut web::ServiceConfig) {
  cfg.service(add_domain_route);
}

#[tracing::instrument(skip(db_pool))]
#[post("/domains")]
async fn add_domain_route(
  db_pool: web::Data<MySqlPool>,
  new_domain: web::Json<NewDomain>,
  current_user: CurrentUser,
) -> HttpResponse {
  tracing::debug!("Creating new domain...");

  let domain = add_domain(current_user.0.id, new_domain.into_inner(), &db_pool).await;

  match domain {
    Ok(domain) => {
      tracing::info!("Returning created domain");
        HttpResponse::Ok().json(domain)
    }
    Err(_) => {
      tracing::error!("Failed to create a domain.");
      HttpResponse::InternalServerError().json(
      ErrorResponse::new(0, "Failed to create a domain")
          .description("Unexpected value returned"),
      )
    }
  }
}