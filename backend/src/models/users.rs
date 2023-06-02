use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Model representing a user.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,

    pub username: String,
    pub display_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Model representing the data sent from the frontend to the server to create a new user.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    pub username: String,
}
