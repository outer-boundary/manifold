use actix_web::{get, web, HttpResponse};

use crate::{common::AppState, models::error::ErrorResponse};

#[get("/health-check")]
async fn health_check_route(app_state: web::Data<AppState>) -> HttpResponse {
    tracing::debug!("Running health check route...");

    let result = sqlx::query("SELECT 1").execute(&app_state.pool).await;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::Error, util::tests::TestPool};
    use actix_web::{http::StatusCode, test, web::Data, App};

    #[actix_web::test]
    async fn test_health_check() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(health_check_route),
        )
        .await;

        let req = test::TestRequest::get().uri("/health-check").to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        Ok(())
    }
}
