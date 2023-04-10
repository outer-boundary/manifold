use crate::{models::messages::*, Error};
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::MySqlPool;

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
    let result: Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> = sqlx::query_as!(
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
