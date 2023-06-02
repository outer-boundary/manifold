use argonautica::{input::Salt, Hasher, Verifier};
use color_eyre::{eyre::eyre, Result};

#[tracing::instrument(skip(password))]
pub async fn hash(password: String) -> Result<(String, Salt)> {
    let pepper = std::env::var("MANIFOLD__AUTHENTICATION_PEPPER")?;

    let mut hasher = Hasher::default();
    hasher.configure_memory_size(20480);
    hasher.configure_iterations(40);

    let hash = hasher
        .with_password(password)
        .with_secret_key(pepper)
        .hash()
        .map_err(|err| eyre!(err))?;

    Ok((hash, hasher.salt().clone()))
}

#[tracing::instrument(skip(hash, password))]
pub async fn verify(hash: String, password: String) -> Result<bool> {
    let pepper = std::env::var("MANIFOLD__AUTHENTICATION_PEPPER")?;

    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(pepper)
        .verify()
        .map_err(|err| eyre!(err))?;

    Ok(is_valid)
}
