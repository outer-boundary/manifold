use serde::{Deserialize, Serialize};

// Model representing a message.
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub content: String,
}

// Model representing the value returned from querying a message from the database.
#[derive(Clone, Serialize)]
pub struct DbMessage {
    pub id: Option<String>,
    pub content: String,
}

// Convert DbMessage to Message, failing if the id is None. This should never be the case though as the id value in the
// database is set to NOT NULL.
impl From<DbMessage> for Message {
    fn from(db_message: DbMessage) -> Self {
        Message {
            id: db_message
                .id
                .expect("ID should never be NULL in the database"),
            content: db_message.content,
        }
    }
}

// Model representing the data sent from the frontend to the server.
#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub content: String,
}
