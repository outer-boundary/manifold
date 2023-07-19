use std::pin::Pin;

use crate::{
    models::error::ErrorResponse,
    util::{auth::session::get_user_id_from_session, users::get_user},
};

use super::login_identity::ClientLoginIdentity;
use actix_session::Session;
use actix_web::{
    dev::Payload,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    FromRequest, HttpRequest,
};
use chrono::NaiveDateTime;
use futures::Future;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

// Model representing a user entry in the users table.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,

    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Model representing the data sent from the client to create a new user.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub username: String,
    pub identity: ClientLoginIdentity,
}

#[derive(Debug)]
pub struct CurrentUser {
    pub user: User,
}

impl FromRequest for CurrentUser {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let session =
                req.app_data::<Session>()
                    .ok_or(ErrorUnauthorized(serde_json::to_string(
                        &ErrorResponse::new(0, "There is no logged in user"),
                    )?))?;

            let user_id = get_user_id_from_session(session).map_err(|err| {
                let response = serde_json::to_string(
                    &ErrorResponse::new(0, "Unable to get user id from session").description(err),
                );

                ErrorInternalServerError(match response {
                    Ok(response_json) => response_json,
                    Err(err) => format!("{{\"code\":0,\"message\":\"Failed to serialize error response\",\"description\":\"{}\"}}", err) 
                })
            })?.ok_or(ErrorUnauthorized(serde_json::to_string(
                    &ErrorResponse::new(0, "Unable to get user id from session"),
                )?))?;

            let db_pool = req.app_data::<MySqlPool>().ok_or(ErrorInternalServerError(
                serde_json::to_string(&ErrorResponse::new(0, "Unable to get the db pool"))?,
            ))?;

            let user = get_user(user_id, db_pool).await.map_err(|err| {
                let response = serde_json::to_string(
                    &ErrorResponse::new(0, "Unable to get user from db").description(err),
                );

                ErrorInternalServerError(match response {
                    Ok(response_json) => response_json,
                    Err(err) => format!("{{\"code\":0,\"message\":\"Failed to serialize error response\",\"description\":\"{}\"}}", err) 
                })
            })?.ok_or(ErrorBadRequest(serde_json::to_string(
                        &ErrorResponse::new(0, "User id stored in session does not match any existing user"),
                    )?))?;

            Ok(CurrentUser { user })
        })
    }
}
