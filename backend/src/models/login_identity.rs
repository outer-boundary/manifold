use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum LoginIdentityType {
    EmailPassword,
}

impl LoginIdentityType {
    pub fn all() -> Vec<LoginIdentityType> {
        vec![LoginIdentityType::EmailPassword]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum LoginIdentity {
    EmailPassword(LIEmailPassword),
}

// Model representing the data sent from the client to create a new user.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LIEmailPassword {
    pub user_id: Uuid,

    pub email: String,
    pub password_hash: String,
    pub salt: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Enum representing all possible login identities that a user can use when first creating their account.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum NewLoginIdentity {
    EmailPassword(NewLIEmailPassword),
}

// Model representing the data sent from the client to create a new user.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewLIEmailPassword {
    pub email: String,
    pub password: String,
}
