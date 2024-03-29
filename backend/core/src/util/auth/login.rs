use super::{
    login_identity::{get_login_identity, get_user_id_from_login_identity},
    password::verify_password_hash,
    session::{create_session_for_user, get_user_id_from_session},
};
use crate::{
    models::login_identity::{ClientLoginIdentity, LoginIdentity},
    types::db::DBPool,
};
use color_eyre::{eyre::eyre, Result};
use uuid::Uuid;

#[tracing::instrument(skip(login_identity, db_pool, session))]
pub async fn login_user(
    login_identity: ClientLoginIdentity,
    db_pool: &DBPool,
    session: &actix_session::Session,
) -> Result<(Option<Uuid>, bool)> {
    let user_id = get_user_id_from_login_identity(login_identity.clone(), db_pool).await?;

    match user_id {
        Some(user_id) => {
            let li_db = get_login_identity(user_id, login_identity.get_type(), db_pool)
                .await?
                .ok_or(eyre!("Could not get the login identity data from the db"))?;

            let is_login_successful = match login_identity.clone() {
                ClientLoginIdentity::Email(li) => {
                    if let LoginIdentity::Email(li_db) = li_db {
                        verify_password_hash(li_db.password_hash, li.password).await
                    } else {
                        Err(eyre!(
                            "Incorrect db login identity type returned for email login identity"
                        ))
                    }
                }
            }?;

            if is_login_successful {
                create_session_for_user(user_id, login_identity.identifier(), session, db_pool)
                    .await?;
            }

            Ok((Some(user_id), is_login_successful))
        }
        None => Ok((None, false)),
    }
}

#[tracing::instrument(skip(session))]
pub async fn logout_user(
    session: &actix_session::Session,
    db_pool: &DBPool,
) -> Result<Option<Uuid>> {
    let user_id = get_user_id_from_session(session)?;

    session.purge();

    // Remove session information from the database
    sqlx::query!("DELETE FROM login_sessions WHERE user_id = $1", user_id)
        .execute(db_pool)
        .await?;

    Ok(user_id)
}
