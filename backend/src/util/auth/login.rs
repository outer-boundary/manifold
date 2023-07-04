use super::{
    login_identity::{get_login_identity, get_user_id_from_login_identity},
    password::verify_password_hash,
};
use crate::models::login_identity::{LoginIdentity, LoginIdentityDB};
use color_eyre::{eyre::eyre, Result};
use sqlx::MySqlPool;
use uuid::Uuid;

const USER_ID_KEY: &str = "MANIFOLD__USER_ID";
const IDENTITY_KEY: &str = "MANIFOLD__IDENTITY";

#[tracing::instrument(skip(login_identity, db_pool, session))]
pub async fn login_user(
    login_identity: LoginIdentity,
    db_pool: &MySqlPool,
    session: &actix_session::Session,
) -> Result<(Option<Uuid>, bool)> {
    let user_id = get_user_id_from_login_identity(login_identity.clone(), db_pool).await?;

    match user_id {
        Some(user_id) => {
            let li_db = get_login_identity(user_id, login_identity.get_type(), db_pool)
                .await?
                .ok_or(eyre!("Could not get the login identity data from the db"))?;

            let is_login_successful = match login_identity.clone() {
                LoginIdentity::Email(li) => {
                    if let LoginIdentityDB::Email(li_db) = li_db {
                        verify_password_hash(li_db.password_hash, li.password).await
                    } else {
                        Err(eyre!(
                            "Incorrect db login identity type returned for email login identity"
                        ))
                    }
                }
            }?;

            if is_login_successful {
                tracing::debug!("Generating session key for user with id '{}'", user_id);
                session.renew();
                session.insert(USER_ID_KEY, user_id)?;
                session.insert(IDENTITY_KEY, login_identity.identifier())?;
            }

            Ok((Some(user_id), is_login_successful))
        }
        None => Ok((None, false)),
    }
}
