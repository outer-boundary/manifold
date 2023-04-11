use crate::{models::messages::*, AppState};
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn messages_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(get_messages).service(add_message);
}

#[get("/messages")]
async fn get_messages(app_state: web::Data<AppState>) -> impl Responder {
    let messages = sqlx::query_as!(Message, "SELECT * FROM messages ORDER BY id")
        .fetch_all(&app_state.pool)
        .await;

    match messages {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/messages")]
async fn add_message(
    app_state: web::Data<AppState>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Message,
        "INSERT INTO messages (content) VALUES (?)",
        new_message.content
    )
    .execute(&app_state.pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{util::tests::TestPool, Error};
    use actix_web::{http::StatusCode, test, web::Data, App};

    #[actix_web::test]
    async fn test_get_messages() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(get_messages),
        )
        .await;

        let req = test::TestRequest::get().uri("/messages").to_request();
        let res = test::call_service(&app, req).await;

        pool.close().await?;

        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }

    #[actix_web::test]
    async fn test_add_message() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(add_message),
        )
        .await;

        let new_message = NewMessage {
            content: "Test message".into(),
        };

        let req = test::TestRequest::post()
            .uri("/messages")
            .set_json(&new_message)
            .to_request();
        let res = test::call_service(&app, req).await;

        pool.close().await?;

        assert_eq!(res.status(), StatusCode::CREATED);

        Ok(())
    }
}
