use super::{error::ExtractorError, login_identity::ClientLoginIdentity};
use crate::{
    models::error::ErrorResponse,
    util::{
        auth::{login::logout_user, session::get_user_id_from_session},
        users::get_user,
    },
};
use actix_session::Session;
use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use chrono::NaiveDateTime;
use futures::Future;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::pin::Pin;
use uuid::Uuid;

// Model representing a user entry in the users table.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,

    pub username: String,

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
    type Error = ExtractorError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        let session_fut = Session::from_request(&req, payload);

        Box::pin(async move {
            let session = session_fut.await.map_err(|err| {
                ExtractorError::InternalServerError(
                    ErrorResponse::new(0, "Unable to get session information").description(err),
                )
            })?;

            let user_id = get_user_id_from_session(&session)
                .map_err(|err| {
                    ExtractorError::InternalServerError(
                        ErrorResponse::new(0, "Unable to get user id from session")
                            .description(err),
                    )
                })?
                .ok_or(ExtractorError::Unauthorized(ErrorResponse::new(
                    0,
                    "No active session",
                )))?;

            let db_pool = req.app_data::<web::Data<MySqlPool>>().ok_or(
                ExtractorError::InternalServerError(ErrorResponse::new(
                    0,
                    "Unable to get the db pool",
                )),
            )?;

            let user_result = get_user(user_id, db_pool)
                .await
                .map_err(|err| {
                    ExtractorError::InternalServerError(
                        ErrorResponse::new(0, "Unable to get user from db").description(err),
                    )
                })?;

            let user = if let Some(user) = user_result {
                user
            } else {
                let logout_result = async {
                    match logout_user(&session, db_pool).await {
                        Ok(_) => ExtractorError::BadRequest(ErrorResponse::new(
                            0,
                            "User id stored in session does not match any existing user. Session has been forcefully ended",
                        )),
                        Err(err) => ExtractorError::InternalServerError(ErrorResponse::new(
                            0,
                            "User id stored in session does not match any existing user but unable to end session",
                        ).description(err)),
                    }
                }
                .await;

                return Err(logout_result);
            };

            Ok(CurrentUser { user })
        })
    }
}
