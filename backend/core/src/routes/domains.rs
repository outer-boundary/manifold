use crate::{
    models::error::ErrorResponse,
    types::{db::DBPool, redis::RedisPool},
};
use actix_web::{cookie::Cookie, post, web, HttpRequest, HttpResponse};

pub fn auth_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(create_domain_route)
}

#[tracing::instrument(skip(db_pool, redis, token))]
#[post("/verify")]
async fn create_domain_route(
    db_pool: web::Data<DBPool>,
    redis: web::Data<RedisPool>,
) -> HttpResponse {
}
