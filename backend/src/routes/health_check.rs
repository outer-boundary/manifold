use actix_web::{get, web, HttpResponse};

use crate::{common::AppState, models::error::ErrorResponse};

#[get("/health-check")]
async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {
    let result = sqlx::query("SELECT 1").execute(&app_state.pool).await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::ServiceUnavailable().json(ErrorResponse::new(
            0,
            "Unable to connect to database".into(),
        )),
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
                .service(health_check),
        )
        .await;

        let req = test::TestRequest::get().uri("/health-check").to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        Ok(())
    }
}
