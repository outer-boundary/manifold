use crate::{
    models::{
        login_identity::{LoginIdentity, LoginIdentityType},
        users::*,
    },
    types::{error::ErrorResponse, redis::RedisPool},
    util::{
        auth::{login::login_user, login_identity::verify_login_identity},
        email::send_multipart_email,
        url::full_uri,
        users::{add_user, delete_user, get_user, get_users},
    },
};
use actix_web::{delete, get, http::header, post, web, HttpRequest, HttpResponse};
use sqlx::MySqlPool;
use uuid::Uuid;

pub fn users_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users_route)
        .service(get_user_route)
        .service(add_user_route)
        .service(delete_user_route)
        .service(verify_user_li_route)
        .service(user_login_route);
}

#[tracing::instrument(skip(pool))]
#[get("")]
async fn get_users_route(pool: web::Data<MySqlPool>) -> HttpResponse {
    tracing::debug!("Requesting all users...");

    let users = get_users(&pool).await;

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

#[tracing::instrument(skip(pool))]
#[get("/{id}")]
async fn get_user_route(pool: web::Data<MySqlPool>, id: web::Path<Uuid>) -> HttpResponse {
    let user_id = id.into_inner();

    tracing::debug!("Requesting user with id '{}'...", user_id);

    let user = get_user(user_id, &pool).await;

    match user {
        Ok(Some(user)) => {
            tracing::info!("Found user with id '{}'.", user_id);
            HttpResponse::Ok().json(user)
        }
        Ok(None) => {
            tracing::warn!("No user found with id '{}'.", user_id);
            HttpResponse::NotFound().json(ErrorResponse::new(
                0,
                format!("No user with id '{}'", user_id),
            ))
        }
        Err(err) => {
            tracing::error!(
                "Failed while trying to find user with id '{}'. {}",
                user_id,
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

#[tracing::instrument(skip(pool, redis, request))]
#[post("")]
async fn add_user_route(
    pool: web::Data<MySqlPool>,
    redis: web::Data<RedisPool>,
    request: HttpRequest,
    new_user: web::Json<NewUser>,
) -> HttpResponse {
    tracing::debug!("Creating new user...");

    // Create the user
    let user = add_user(new_user.clone(), &pool).await;

    match user {
        Ok(user) => match new_user.clone().identity {
            LoginIdentity::Email(li) => {
                let result = send_multipart_email(
                    "Manifold Account Verification".to_string(),
                    user.id,
                    li.email,
                    user.username.clone(),
                    "verification_email.html",
                    LoginIdentityType::Email,
                    &redis,
                )
                .await;

                match result {
                    Ok(_) => {
                        tracing::info!("Created new user with id '{}'.", user.id);
                        HttpResponse::Created()
                            .append_header((
                                header::LOCATION,
                                format!("{}/{}", full_uri(&request), user.id),
                            ))
                            .json(user)
                    }
                    Err(err) => {
                        tracing::error!(
                                    "Error occurred while trying to send verification email to user with id '{}'. {}",
                                    user.id,
                                    err
                                );
                        HttpResponse::InternalServerError().json(
                                    ErrorResponse::new(
                                        0,
                                        format!(
                                            "Error occurred while trying to send verification email to user with id '{}'",
                                            user.id
                                        ),
                                    )
                                    .description(err),
                                )
                    }
                }
            }
        },
        Err(err) => {
            tracing::error!("Failed while trying to create new user. {}", err);
            HttpResponse::InternalServerError().json(
                ErrorResponse::new(0, "Error occurred while trying to create new user")
                    .description(err),
            )
        }
    }
}

#[tracing::instrument(skip(pool))]
#[delete("/{id}")]
async fn delete_user_route(pool: web::Data<MySqlPool>, id: web::Path<Uuid>) -> HttpResponse {
    let user_id = id.into_inner();

    tracing::debug!("Deleting user with id '{}'...", user_id);

    let user = get_user(user_id, &pool).await;

    match user {
        Ok(Some(_)) => {
            let result = delete_user(user_id, &pool).await;

            match result {
                Ok(_) => {
                    tracing::info!("Deleted user with id '{}'.", user_id);
                    HttpResponse::NoContent().finish()
                }
                Err(err) => {
                    tracing::error!(
                        "Failed while trying to delete user with id '{}'. {}",
                        user_id,
                        err
                    );
                    HttpResponse::InternalServerError().json(
                        ErrorResponse::new(
                            0,
                            format!("Unable to delete user with id '{}'", user_id),
                        )
                        .description(err),
                    )
                }
            }
        }
        Ok(None) => {
            tracing::warn!("Trying to delete non-existent user with id '{}'.", user_id);
            HttpResponse::NotFound().json(ErrorResponse::new(
                0,
                format!("Trying to delete non-existent user with id '{}'", user_id),
            ))
        }
        Err(err) => {
            tracing::error!(
                "Failed while trying to delete user with id '{}'. {}",
                user_id,
                err
            );
            HttpResponse::InternalServerError().json(
                ErrorResponse::new(0, format!("Unable to delete user with id '{}'", user_id))
                    .description(err),
            )
        }
    }
}

#[tracing::instrument(skip(pool, redis, token))]
#[post("/verify")]
async fn verify_user_li_route(
    pool: web::Data<MySqlPool>,
    redis: web::Data<RedisPool>,
    token: web::Json<String>,
) -> HttpResponse {
    tracing::debug!("Verifying login identity...");

    let result = verify_login_identity(token.into_inner(), &pool, &redis).await;

    match result {
        Ok(user_id) => {
            tracing::info!(
                "Successfully verified login identity for user with id '{}'.",
                user_id
            );
            HttpResponse::NoContent().finish()
        }
        Err(err) => {
            tracing::error!("Failed while trying to verify login identity. {}", err);
            HttpResponse::InternalServerError().json(
                ErrorResponse::new(0, "Failed while trying to verify login identity")
                    .description(err),
            )
        }
    }
}

#[tracing::instrument(skip(login_identity, pool, session))]
#[post("/login")]
async fn user_login_route(
    login_identity: web::Json<LoginIdentity>,
    pool: web::Data<MySqlPool>,
    session: actix_session::Session,
) -> HttpResponse {
    tracing::debug!("Logging in user...");

    let login_result = login_user(login_identity.into_inner(), &pool, &session).await;

    match login_result {
        Ok((Some(user_id), true)) => {
            tracing::info!("Successfully logged in user with id '{}'.", user_id);
            HttpResponse::NoContent().finish()
        }
        Ok((user_id, false)) => {
            if let Some(user_id) = user_id {
                tracing::warn!("Failed login attempt for user with id '{}'.", user_id);
            } else {
                tracing::warn!("Failed login attempt.");
            }

            HttpResponse::Unauthorized().finish()
        }
        Err(err) => {
            tracing::error!("Failed while trying to login user. {}", err);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::new(0, "Failed while trying to login user").description(err))
        }
        // The below arm should not be possible. We should only return true if we are able to find a matching user
        // for the login identity. This means that we will always have the user id to return if the login is successful.
        Ok((None, true)) => {
            tracing::error!("Failed while trying to login user. Unexpected value returned.");
            HttpResponse::InternalServerError().json(
                ErrorResponse::new(0, "Failed while trying to login user")
                    .description("Unexpected value returned"),
            )
        }
    }
}
