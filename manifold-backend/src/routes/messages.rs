use crate::models::messages::*;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::MySqlPool;

pub fn messages_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(get_messages).service(add_message);
}

#[get("/messages")]
async fn get_messages(pool: web::Data<MySqlPool>) -> impl Responder {
    let messages = sqlx::query_as!(Message, "SELECT * FROM messages ORDER BY id")
        .fetch_all(pool.as_ref())
        .await;

    match messages {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/messages")]
async fn add_message(
    pool: web::Data<MySqlPool>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Message,
        "INSERT INTO messages (content) VALUES (?)",
        new_message.content
    )
    .execute(pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{util::environment, Error};
    use actix_web::{http::StatusCode, test, web::Data, App};

    #[actix_web::test]
    async fn test_get_messages() -> Result<(), Error> {
        let config = environment::init().await?;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(config.db.pool.clone()))
                .service(get_messages),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        Ok(())
    }

    #[actix_web::test]
    async fn test_add_message() -> Result<(), Error> {
        let config = environment::init().await?;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(config.db.pool.clone()))
                .service(add_message),
        )
        .await;

        let new_message = NewMessage {
            content: "Test message".into(),
        };

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&new_message)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::CREATED);

        Ok(())
    }
}
