use crate::models::messages::*;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::MySqlPool;

pub fn messages_scope() -> actix_web::Scope {
    web::scope("/messages")
        .service(get_messages)
        .service(add_message)
}

#[get("/")]
async fn get_messages(pool: web::Data<MySqlPool>) -> impl Responder {
    let messages = sqlx::query_as!(Message, "SELECT * FROM messages ORDER BY id")
        .fetch_all(pool.as_ref())
        .await;

    match messages {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/")]
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
