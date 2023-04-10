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
