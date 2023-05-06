use crate::{
    models::{error::ErrorResponse, users::*},
    util::{
        url::full_uri,
        users::{
            add_user as add_new_user, delete_user as delete_user_by_id, get_user as get_user_by_id,
            get_users as get_all_users,
        },
    },
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
    let users = get_all_users(&app_state.pool).await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(
            ErrorResponse::new(0, "Error occurred while trying to list all users".into())
                .description(e.to_string()),
        ),
    }
}

#[get("/users/{id}")]
async fn get_user(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let user_id = id.into_inner();

    let user = get_user_by_id(user_id.clone(), &app_state.pool).await;

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse::new(
            0,
            format!("No user with id '{}'", user_id),
        )),
        Err(e) => HttpResponse::InternalServerError().json(
            ErrorResponse::new(
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
    let result = add_new_user(new_user.into_inner(), &app_state.pool).await;

    match result {
        Ok(user_id) => {
            let user = get_user_by_id(user_id.clone(), &app_state.pool).await;

            match user {
                Ok(Some(user)) => HttpResponse::Created()
                    .append_header((
                        header::LOCATION,
                        format!("{}/{}", full_uri(&request), user_id),
                    ))
                    .json(user),
                Ok(None) => HttpResponse::InternalServerError().json(ErrorResponse::new(
                    0,
                    format!("Could not find newly created user with id '{}'", user_id),
                )),
                Err(e) => HttpResponse::InternalServerError().json(
                    ErrorResponse::new(
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
            ErrorResponse::new(0, "Error occurred while trying to create new user".into())
                .description(e.to_string()),
        ),
    }
}

#[delete("/users/{id}")]
async fn delete_user(app_state: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let user_id = id.into_inner();
    let result = delete_user_by_id(user_id.clone(), &app_state.pool).await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(
            ErrorResponse::new(0, format!("Unable to delete user with id '{}'", user_id))
                .description(e.to_string()),
        ),
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
            display_name: "test_user".into(),
            first_name: "Test".into(),
            last_name: "User".into(),
        };

        let result: sqlx::Result<mysql::MySqlQueryResult> = sqlx::query!(
            "INSERT INTO users (id, display_name, first_name, last_name) VALUES (as_bin(?), ?, ?, ?)",
            user_id,
            new_user.display_name,
            new_user.first_name,
            new_user.last_name
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
            assert_eq!(user.display_name, new_user.display_name);
            assert_eq!(user.first_name, new_user.first_name);
            assert_eq!(user.last_name, new_user.last_name);

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
            display_name: "test_user".into(),
            first_name: "Test".into(),
            last_name: "User".into(),
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
