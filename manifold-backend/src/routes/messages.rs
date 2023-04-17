use crate::{models::messages::*, AppState};
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn messages_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(get_messages)
        .service(get_message)
        .service(add_message);
}

#[get("/messages")]
async fn get_messages(app_state: web::Data<AppState>) -> impl Responder {
    let messages: sqlx::Result<Vec<Message>> = sqlx::query_as!(
        DbMessage,
        "SELECT BIN_TO_UUID(id, true) as id, content FROM messages ORDER BY id"
    )
    .fetch_all(&app_state.pool)
    .await
    .map(|db_messages| {
        db_messages
            .iter()
            .map(|db_message| db_message.clone().into())
            .collect()
    });

    match messages {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/messages/{id}")]
async fn get_message(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let message: sqlx::Result<Option<Message>> = sqlx::query_as!(
        DbMessage,
        "SELECT BIN_TO_UUID(id, true) as id, content FROM messages WHERE id = UUID_TO_BIN(?, true)",
        id.into_inner()
    )
    .fetch_optional(&app_state.pool)
    .await
    .map(|db_message| db_message.map(|db_message| db_message.into()));

    match message {
        Ok(Some(message)) => HttpResponse::Ok().json(message),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/messages")]
async fn add_message(
    app_state: web::Data<AppState>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    let result = sqlx::query!(
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
    use sqlx::mysql;
    use uuid::Uuid;

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

        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }

    #[actix_web::test]
    async fn test_get_message() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(get_message),
        )
        .await;

        let message_id = Uuid::new_v4().to_string();
        let new_message = NewMessage {
            content: "Test message".into(),
        };

        let result: sqlx::Result<mysql::MySqlQueryResult> = sqlx::query!(
            "INSERT INTO messages (id, content) VALUES (UUID_TO_BIN(?, true), ?)",
            message_id,
            new_message.content
        )
        .execute(&app_state.pool)
        .await;

        if result.is_ok() {
            let req = test::TestRequest::get()
                .uri(&format!("/messages/{}", message_id))
                .to_request();
            let res = test::call_service(&app, req).await;

            assert_eq!(res.status(), StatusCode::OK);

            let message: Message = test::read_body_json(res).await;
            assert_eq!(message.id, message_id);
            assert_eq!(message.content, new_message.content);

            return Ok(());
        }

        Err("Failed to insert value into DB".into())
    }

    #[actix_web::test]
    async fn test_get_message_not_found() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(get_message),
        )
        .await;

        let non_existent_id = Uuid::new_v4().to_string();
        let req = test::TestRequest::get()
            .uri(&format!("/messages/{}", non_existent_id))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);

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

        assert_eq!(res.status(), StatusCode::CREATED);

        Ok(())
    }
}
