use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/health-check")]
async fn health_check(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query("SELECT 1").execute(&app_state.pool).await;

    if result.is_ok() {
        return HttpResponse::Ok().finish();
    }

    HttpResponse::ServiceUnavailable().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{util::tests::TestPool, Error};
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

        pool.close().await?;

        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }
}
