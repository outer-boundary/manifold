use crate::common::MFResult;
use argonautica::{Hasher, Verifier};
use uuid::Uuid;

#[tracing::instrument(skip(password, salt))]
pub async fn hash(password: String, salt: Option<String>) -> MFResult<(String, String)> {
    let salt = salt.unwrap_or(Uuid::new_v4().as_simple().to_string());
    let pepper = std::env::var("MANIFOLD__AUTHENTICATION_PEPPER")?;

    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .with_salt(salt)
        .with_secret_key(pepper)
        .hash()
        .map_err(|err| err.into())?;

    Ok((hash, salt))
}

#[tracing::instrument(skip(hash, password))]
pub async fn verify(hash: String, password: String) -> MFResult<bool> {
    let pepper = std::env::var("MANIFOLD__AUTHENTICATION_PEPPER")?;

    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(pepper)
        .verify()
        .map_err(|err| err.into())?;

    Ok(is_valid)
}
