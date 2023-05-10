use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Model representing a user.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,

    pub username: String,
    pub display_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Model representing the value returned from querying a user from the database.
#[derive(Clone, Serialize)]
pub struct DbUser {
    pub id: Option<String>,

    pub username: String,
    pub display_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Convert DbIser to User, failing if the 'id' is None. This should never be the case though as the 'id' value in the
// database is set to NOT NULL.
impl From<DbUser> for User {
    fn from(db_user: DbUser) -> Self {
        User {
            id: db_user.id.expect("ID should never be NULL in the database"),

            username: db_user.username,
            display_name: db_user.display_name,
            first_name: db_user.first_name,
            last_name: db_user.last_name,

            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
        }
    }
}

// Model representing the data sent from the frontend to the server to create a new user.
#[derive(Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub username: String,
}
