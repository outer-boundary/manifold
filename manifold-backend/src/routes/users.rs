use crate::{models::users::*, AppState};
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn users_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users).service(get_user).service(add_user);
}

#[get("/users")]
async fn get_users(app_state: web::Data<AppState>) -> impl Responder {
    let users: sqlx::Result<Vec<User>> = sqlx::query_as!(
        DbUser,
        "SELECT BIN_TO_UUID(id, true) as id, content FROM users ORDER BY id"
    )
    .fetch_all(&app_state.pool)
    .await
    .map(|db_users| {
        db_users
            .iter()
            .map(|db_user| db_user.clone().into())
            .collect()
    });

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/users/{id}")]
async fn get_user(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let user: sqlx::Result<Option<User>> = sqlx::query_as!(
        DbUser,
        "SELECT BIN_TO_UUID(id, true) as id, content FROM users WHERE id = UUID_TO_BIN(?, true)",
        id.into_inner()
    )
    .fetch_optional(&app_state.pool)
    .await
    .map(|db_user| db_user.map(|db_user| db_user.into()));

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/users")]
async fn add_user(app_state: web::Data<AppState>, new_user: web::Json<NewUser>) -> impl Responder {
    let result = sqlx::query!("INSERT INTO users (content) VALUES (?)", new_user.content)
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
    async fn test_get_users() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(get_users),
        )
        .await;

        let req = test::TestRequest::get().uri("/users").to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }

    #[actix_web::test]
    async fn test_get_user() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(get_user),
        )
        .await;

        let user_id = Uuid::new_v4().to_string();
        let new_user = NewUser {
            content: "Test user".into(),
        };

        let result: sqlx::Result<mysql::MySqlQueryResult> = sqlx::query!(
            "INSERT INTO users (id, content) VALUES (UUID_TO_BIN(?, true), ?)",
            user_id,
            new_user.content
        )
        .execute(&app_state.pool)
        .await;

        if result.is_ok() {
            let req = test::TestRequest::get()
                .uri(&format!("/users/{}", user_id))
                .to_request();
            let res = test::call_service(&app, req).await;

            assert_eq!(res.status(), StatusCode::OK);

            let user: User = test::read_body_json(res).await;
            assert_eq!(user.id, user_id);
            assert_eq!(user.content, new_user.content);

            return Ok(());
        }

        Err("Failed to insert value into DB".into())
    }

    #[actix_web::test]
    async fn test_get_user_not_found() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(get_user),
        )
        .await;

        let non_existent_id = Uuid::new_v4().to_string();
        let req = test::TestRequest::get()
            .uri(&format!("/users/{}", non_existent_id))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        Ok(())
    }

    #[actix_web::test]
    async fn test_add_user() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(add_user),
        )
        .await;

        let new_user = NewUser {
            content: "Test user".into(),
        };

        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::CREATED);

        Ok(())
    }
}
