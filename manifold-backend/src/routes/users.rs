use crate::{
    models::{error::Error, users::*},
    util::url::full_uri,
    AppState,
};
use actix_web::{delete, get, http::header, post, web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

pub fn users_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user)
        .service(add_user)
        .service(delete_user);
}

#[get("/users")]
async fn get_users(app_state: web::Data<AppState>) -> impl Responder {
    let users: sqlx::Result<Vec<User>> = sqlx::query_as!(
        DbUser,
        "SELECT bin_to_uuid(id, true) AS id, username FROM users ORDER BY id"
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
        Err(e) => HttpResponse::InternalServerError().json(
            Error::new(0, "Error occurred while trying to list all users".into())
                .description(e.to_string()),
        ),
    }
}

#[get("/users/{id}")]
async fn get_user(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let user_id = id.into_inner();

    let user: sqlx::Result<Option<User>> = sqlx::query_as!(
        DbUser,
        "SELECT bin_to_uuid(id, true) AS id, username FROM users WHERE id = uuid_to_bin(?, true)",
        user_id
    )
    .fetch_optional(&app_state.pool)
    .await
    .map(|db_user| db_user.map(|db_user| db_user.into()));

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().json(Error::new(0, format!("No user with id '{}'", user_id)))
        }
        Err(e) => HttpResponse::InternalServerError().json(
            Error::new(
                0,
                format!(
                    "Error occurred while trying to get user with id '{}'",
                    user_id
                ),
            )
            .description(e.to_string()),
        ),
    }
}

#[post("/users")]
async fn add_user(
    app_state: web::Data<AppState>,
    request: HttpRequest,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let user_id = Uuid::new_v4().to_string();
    let result = sqlx::query!(
        "INSERT INTO users (id, username) VALUES (uuid_to_bin(?, true), ?)",
        user_id,
        new_user.username
    )
    .execute(&app_state.pool)
    .await;

    match result {
        Ok(_) => {
            let user: sqlx::Result<Option<User>> = sqlx::query_as!(
                DbUser,
                "SELECT bin_to_uuid(id, true) AS id, username FROM users WHERE id = uuid_to_bin(?, true)",
                user_id
            )
            .fetch_optional(&app_state.pool)
            .await
            .map(|db_user| db_user.map(|db_user| db_user.into()));

            match user {
                Ok(Some(user)) => HttpResponse::Created()
                    .append_header((
                        header::LOCATION,
                        format!("{}/{}", full_uri(&request), user_id),
                    ))
                    .json(user),
                Ok(None) => HttpResponse::InternalServerError().json(Error::new(
                    0,
                    format!("Could not find newly created user with id '{}'", user_id),
                )),
                Err(e) => HttpResponse::InternalServerError().json(
                    Error::new(
                        0,
                        format!(
                            "Error occurred while trying to get newly created user with id '{}'",
                            user_id
                        ),
                    )
                    .description(e.to_string()),
                ),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(
            Error::new(0, "Error occurred while trying to create new user".into())
                .description(e.to_string()),
        ),
    }
}

#[delete("/users/{id}")]
async fn delete_user(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let user = sqlx::query!(
        "DELETE FROM users WHERE id = uuid_to_bin(?, true)",
        id.into_inner()
    )
    .execute(&app_state.pool)
    .await;

    match user {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError()
            .json(Error::new(0, "Unable to create new user".into()).description(e.to_string())),
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
            username: "Test user".into(),
        };

        let result: sqlx::Result<mysql::MySqlQueryResult> = sqlx::query!(
            "INSERT INTO users (id, username) VALUES (uuid_to_bin(?, true), ?)",
            user_id,
            new_user.username
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
            assert_eq!(user.username, new_user.username);

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
            username: "Test user".into(),
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
