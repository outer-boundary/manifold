use crate::{
  models::{error::ErrorResponse},
  types::redis::RedisPool,
};
use actix_web::{cookie::Cookie, post, web, HttpRequest, HttpResponse};
use sqlx::MySqlPool;

pub fn domains_scope(cfg: &mut web::ServiceConfig) {
  cfg.service(create_domain_route)
}

#[tracing::instrument(skip(db_pool, redis, token))]
#[post("/domains")]
async fn create_domain_route(
  db_pool: web::Data<MySqlPool>,
  redis: web::Data<RedisPool>,
) -> HttpResponse {
  
}