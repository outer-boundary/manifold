use crate::models::login_identity::{LoginIdentity, LoginIdentityDB};
use color_eyre::{eyre::eyre, Result};
use sqlx::MySqlPool;
use uuid::Uuid;

use super::{
    login_identity::{get_login_identity, get_user_id_from_login_identity},
    password::verify_password_hash,
};

#[tracing::instrument(skip(login_identity, db_pool))]
pub async fn login_user(
    login_identity: LoginIdentity,
    db_pool: &MySqlPool,
) -> Result<(Uuid, bool)> {
    let user_id = get_user_id_from_login_identity(login_identity.clone(), db_pool)
        .await?
        .ok_or(eyre!(
            "Could not find an existing login identity that matched"
        ))?;

    let li_db = get_login_identity(user_id, login_identity.get_type(), db_pool)
        .await?
        .ok_or(eyre!("Could not get the login identity data from the db"))?;

    let login_result = match login_identity {
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

    Ok((user_id, login_result))
}
