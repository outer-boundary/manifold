use crate::common::AppState;
use crate::{
    models::{error::ErrorResponse, users::*},
    util::{
        url::full_uri,
        users::{add_user, delete_user, get_user, get_users},
    },
};
use actix_web::{delete, get, http::header, post, web, HttpRequest, HttpResponse};

pub fn users_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users_route)
        .service(get_user_route)
        .service(add_user_route)
        .service(delete_user_route);
}

#[tracing::instrument]
#[get("/users")]
async fn get_users_route(app_state: web::Data<AppState>) -> HttpResponse {
    tracing::debug!("Requesting all users...");

    let users = get_users(&app_state.pool).await;

    match users {
        Ok(users) => {
            tracing::info!("Returning list of all users.");
            HttpResponse::Ok().json(users)
        }
        Err(err) => {
            tracing::error!("Failed while trying to get a list of all users. {}", err);
            HttpResponse::InternalServerError().json(
                ErrorResponse::new(0, "Error occurred while trying to list all users")
                    .description(err),
            )
        }
    }
}

#[tracing::instrument]
#[get("/users/{id}")]
async fn get_user_route(app_state: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let user_id = id.into_inner();

    tracing::debug!("Requesting user with id '{}'...", user_id.clone());

    let user = get_user(user_id.clone(), &app_state.pool).await;

    match user {
        Ok(Some(user)) => {
            tracing::info!("Found user with id '{}'.", user_id.clone());
            HttpResponse::Ok().json(user)
        }
        Ok(None) => {
            tracing::warn!("No user found with id '{}'.", user_id.clone());
            HttpResponse::NotFound().json(ErrorResponse::new(
                0,
                format!("No user with id '{}'", user_id),
            ))
        }
        Err(err) => {
            tracing::error!(
                "Failed while trying to find user with id '{}'. {}",
                user_id.clone(),
                err
            );
            HttpResponse::InternalServerError().json(
                ErrorResponse::new(
                    0,
                    format!(
                        "Error occurred while trying to get user with id '{}'",
                        user_id
                    ),
                )
                .description(err),
            )
        }
    }
}

#[tracing::instrument]
#[post("/users")]
async fn add_user_route(
    app_state: web::Data<AppState>,
    request: HttpRequest,
    new_user: web::Json<NewUser>,
) -> HttpResponse {
    tracing::debug!("Creating new user...");

    let result = add_user(new_user.into_inner(), &app_state.pool).await;

    match result {
        Ok(user_id) => {
            let user = get_user(user_id.clone(), &app_state.pool).await;

            match user {
                Ok(Some(user)) => {
                    tracing::info!("Created new user with id '{}'.", user_id.clone());
                    HttpResponse::Created()
                        .append_header((
                            header::LOCATION,
                            format!("{}/{}", full_uri(&request), user_id),
                        ))
                        .json(user)
                }
                Ok(None) => {
                    tracing::error!(
                        "Could not find newly created user with id '{}'.",
                        user_id.clone()
                    );
                    HttpResponse::InternalServerError().json(ErrorResponse::new(
                        0,
                        format!("Could not find newly created user with id '{}'", user_id),
                    ))
                }
                Err(err) => {
                    tracing::error!(
                        "Error occurred while trying to get newly created user with id '{}'. {}",
                        user_id.clone(),
                        err
                    );
                    HttpResponse::InternalServerError().json(
                        ErrorResponse::new(
                            0,
                            format!(
                            "Error occurred while trying to get newly created user with id '{}'",
                            user_id
                        ),
                        )
                        .description(err),
                    )
                }
            }
        }
        Err(err) => {
            tracing::error!("Failed while trying to create new user. {}", err);
            HttpResponse::InternalServerError().json(
                ErrorResponse::new(0, "Error occurred while trying to create new user")
                    .description(err),
            )
        }
    }
}

#[tracing::instrument]
#[delete("/users/{id}")]
async fn delete_user_route(app_state: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let user_id = id.into_inner();

    tracing::debug!("Deleting user with id '{}'...", user_id.clone());

    let result = delete_user(user_id.clone(), &app_state.pool).await;

    match result {
        Ok(_) => {
            tracing::info!("Deleted user with id '{}'.", user_id.clone());
            HttpResponse::NoContent().finish()
        }
        Err(err) => {
            tracing::error!(
                "Failed while trying to delete user with id '{}'. {}",
                user_id.clone(),
                err
            );
            HttpResponse::InternalServerError().json(
                ErrorResponse::new(0, format!("Unable to delete user with id '{}'", user_id))
                    .description(err),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Error;
    use crate::util::tests::TestPool;
    use actix_web::{http::StatusCode, test, web::Data, App};
    use uuid::Uuid;

    #[actix_web::test]
    async fn test_get_users() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(get_users_route),
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
                .service(get_user_route),
        )
        .await;

        let new_user = NewUser {
            username: "test_user".into(),
        };

        let user_id = add_user(new_user.clone(), &app_state.pool).await?;

        let req = test::TestRequest::get()
            .uri(&format!("/users/{}", user_id))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let user: User = test::read_body_json(res).await;
        assert_eq!(user.id, user_id);
        assert_eq!(user.username, new_user.username);

        Ok(())
    }

    #[actix_web::test]
    async fn test_get_user_not_found() -> Result<(), Error> {
        let pool = TestPool::connect().await?;

        let app_state = AppState { pool: pool.get() };

        let app = test::init_service(
            App::new()
                .app_data(Data::new(app_state.clone()))
                .service(get_user_route),
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
                .service(add_user_route),
        )
        .await;

        let new_user = NewUser {
            username: "test_user".into(),
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
