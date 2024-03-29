use actix_web::{get, web, HttpResponse};
use deadpool_redis::Pool;

use crate::{
    models::error::ErrorResponse,
    types::db::DBPool,
    util::health_check::{database_connection_check, redis_connection_check},
};

#[tracing::instrument(skip(db_pool, redis))]
#[get("/health-check")]
async fn health_check_route(db_pool: web::Data<DBPool>, redis: web::Data<Pool>) -> HttpResponse {
    tracing::debug!("Running health check route...");

    // Determine whether the database connection is working.
    let db_result = database_connection_check(&db_pool).await;

    if let Err(err) = db_result {
        tracing::error!("Failed database connection health check failed. {}", err);
        return HttpResponse::ServiceUnavailable()
            .json(ErrorResponse::new(0, "Unable to connect to database"));
    };

    let redis_result = redis_connection_check(&redis).await;

    if let Err(err) = redis_result {
        tracing::error!("Failed redis connection health check. {}", err);
        return HttpResponse::ServiceUnavailable()
            .json(ErrorResponse::new(0, "Unable to connect to redis"));
    };

    tracing::info!("Server is healthy.");
    HttpResponse::NoContent().finish()
}
