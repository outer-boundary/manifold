use actix_web::{get, web, HttpResponse};
use sqlx::MySqlPool;

use crate::models::error::ErrorResponse;

#[tracing::instrument]
#[get("/health-check")]
async fn health_check_route(pool: web::Data<MySqlPool>) -> HttpResponse {
    tracing::debug!("Running health check route...");

    let result = sqlx::query("SELECT 1").execute(&**pool).await;

    match result {
        Ok(_) => {
            tracing::info!("Server is healthy.");
            HttpResponse::NoContent().finish()
        }
        Err(err) => {
            tracing::error!("Server failed health check. {}", err);
            HttpResponse::ServiceUnavailable()
                .json(ErrorResponse::new(0, "Unable to connect to database"))
        }
    }
}
