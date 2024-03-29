use crate::{
    models::{
        error::ErrorResponse,
        login_identity::{ClientLoginIdentity, LoginIdentityType},
        users::*,
    },
    types::{db::DBPool, redis::RedisPool},
    util::{
        configuration::{get_config, Environment},
        email::send_multipart_email,
        url::full_uri,
        users::{add_user, delete_user, get_user, get_users},
    },
};
use actix_web::{delete, get, http::header, post, web, HttpRequest, HttpResponse};
use macros::require_role;
use uuid::Uuid;

pub fn users_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users_route)
        .service(get_user_route)
        .service(add_user_route)
        .service(delete_user_route);
}

#[tracing::instrument(skip(db_pool, current_user), fields(current_user_id = %current_user.0.id))]
#[get("")]
#[require_role(role = "sys-admin")]
async fn get_users_route(db_pool: web::Data<DBPool>, current_user: CurrentUser) -> HttpResponse {
    tracing::debug!("Requesting all users...");

    let users = get_users(&db_pool).await;

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

#[tracing::instrument(skip(db_pool, current_user), fields(current_user_id = %current_user.0.id))]
#[get("/{user_id}")]
async fn get_user_route(
    db_pool: web::Data<DBPool>,
    user_id: web::Path<Uuid>,
    current_user: CurrentUser,
) -> HttpResponse {
    let user_id = user_id.into_inner();

    tracing::debug!("Requesting user with id '{}'...", user_id);

    if current_user.0.id != user_id && current_user.0.account_role != AccountRole::SysAdmin {
        tracing::warn!(
            "User '{}' trying to access details for user with id '{}'.",
            current_user.0.id,
            user_id
        );
        return HttpResponse::Forbidden().json(ErrorResponse::new(
            0,
            format!(
                "User '{}' trying to access details for user with id '{}'",
                current_user.0.id, user_id
            ),
        ));
    }

    let user = get_user(user_id, &db_pool).await;

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

#[tracing::instrument(skip(db_pool, redis, request))]
#[post("")]
async fn add_user_route(
    db_pool: web::Data<DBPool>,
    redis: web::Data<RedisPool>,
    request: HttpRequest,
    new_user: web::Json<NewUser>,
    current_user: OptionalCurrentUser,
) -> HttpResponse {
    tracing::debug!("Creating new user...");

    let config = get_config();

    if let Err(err) = config {
        return HttpResponse::InternalServerError()
            .json(ErrorResponse::new(0, "Unable to get app config").description(err));
    }

    if config.unwrap().environment != Environment::Development {
        if let Some(current_user) = current_user.0 {
            if current_user.account_role != AccountRole::SysAdmin {
                tracing::warn!("User '{}' is not a sys-admin.", current_user.id);
                return HttpResponse::Forbidden().json(ErrorResponse::new(
                    0,
                    format!("User '{}' is not a sys-admin", current_user.id),
                ));
            }
        } else {
            tracing::warn!("No active session");
            return HttpResponse::Forbidden().json(ErrorResponse::new(0, "No active session"));
        }
    }

    // Create the user
    let user = add_user(new_user.clone(), &db_pool).await;

    match user {
        Ok(user) => match new_user.clone().identity {
            ClientLoginIdentity::Email(li) => {
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

#[tracing::instrument(skip(db_pool, current_user), fields(current_user_id = %current_user.0.id))]
#[delete("/{user_id}")]
async fn delete_user_route(
    db_pool: web::Data<DBPool>,
    user_id: web::Path<Uuid>,
    current_user: CurrentUser,
) -> HttpResponse {
    let user_id = user_id.into_inner();

    tracing::debug!("Deleting user with id '{}'...", user_id);

    if current_user.0.id != user_id && current_user.0.account_role != AccountRole::SysAdmin {
        tracing::warn!(
            "User '{}' trying to delete user with id '{}'.",
            current_user.0.id,
            user_id
        );
        return HttpResponse::Forbidden().json(ErrorResponse::new(
            0,
            format!(
                "User '{}' trying to delete user with id '{}'",
                current_user.0.id, user_id
            ),
        ));
    }

    let user = get_user(user_id, &db_pool).await;

    match user {
        Ok(Some(_)) => {
            let result = delete_user(user_id, &db_pool).await;

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
