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
use sqlx::{MySql, MySqlPool};
use std::{pin::Pin, str::FromStr};
use uuid::Uuid;

// Model representing a user entry in the users table.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,

    pub username: String,
    pub account_role: AccountRole,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Model representing a user's account's role. This is a global role that applies across the whole application.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum AccountRole {
    User,
    SysAdmin,
}

impl FromStr for AccountRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "User" | "user" => Ok(AccountRole::User),
            "SysAdmin" | "sys-admin" => Ok(AccountRole::SysAdmin),
            _ => Err(()),
        }
    }
}

impl ToString for AccountRole {
    fn to_string(&self) -> String {
        match self {
            AccountRole::User => "user".to_string(),
            AccountRole::SysAdmin => "sys-admin".to_string(),
        }
    }
}

impl sqlx::Type<sqlx::MySql> for AccountRole {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
        <String as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

impl<'r> sqlx::Decode<'r, MySql> for AccountRole {
    fn decode(
        value: <MySql as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<MySql>>::decode(value)?;
        tracing::info!("{}", s);

        AccountRole::from_str(s).map_err(|_| format!("Unknown variant: {}", s).into())
    }
}

impl sqlx::Encode<'_, MySql> for AccountRole {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> sqlx::encode::IsNull {
        sqlx::Encode::<MySql>::encode_by_ref(&self.to_string(), buf)
    }
}

// Model representing the data sent from the client to create a new user.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub username: String,
    pub identity: ClientLoginIdentity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_role: Option<AccountRole>,
}

#[derive(Debug)]
pub struct CurrentUser(pub User);

#[derive(Debug)]
pub struct OptionalCurrentUser(pub Option<User>);

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

            let user = get_user(user_id, db_pool)
                .await
                .map_err(|err| {
                    ExtractorError::InternalServerError(
                        ErrorResponse::new(0, "Unable to get user from db").description(err),
                    )
                })?
                .ok_or(async move {
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
                });

            match user {
                Ok(user) => Ok(CurrentUser(user)),
                Err(err) => Err(err.await),
            }
        })
    }
}

impl FromRequest for OptionalCurrentUser {
    type Error = ExtractorError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let current_user = CurrentUser::from_request(req, payload);

        Box::pin(async move {
            match current_user.await {
                Ok(user) => Ok(OptionalCurrentUser(Some(user.0))),
                Err(err) => {
                    match err {
                        // In the case of Unauthorized or BadRequest, return None, meaning no user found.
                        ExtractorError::Unauthorized(_) | ExtractorError::BadRequest(_) => {
                            Ok(OptionalCurrentUser(None))
                        }
                        // For other errors, propagate them as is.
                        _ => Err(err),
                    }
                }
            }
        })
    }
}
