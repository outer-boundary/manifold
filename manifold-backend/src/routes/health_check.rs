use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

#[get("/health-check")]
async fn health_check(pool: web::Data<MySqlPool>) -> impl Responder {
    let connection = pool.acquire().await;

    if let Ok(mut connection) = connection {
        let result = sqlx::query("SELECT 1").execute(&mut connection).await;

        if result.is_ok() {
            return HttpResponse::Ok().finish();
        }
    }

    HttpResponse::ServiceUnavailable().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{util::environment, Error};
    use actix_web::{http::StatusCode, test, web::Data, App};

    #[actix_web::test]
    async fn test_health_check() -> Result<(), Error> {
        let config = environment::init().await?;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(config.db.pool))
                .service(health_check),
        )
        .await;

        let req = test::TestRequest::get().uri("/health-check").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        Ok(())
    }
}
