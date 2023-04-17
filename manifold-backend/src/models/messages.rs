use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub content: String,
}

#[derive(Clone, Serialize)]
pub struct DbMessage {
    pub id: Option<String>,
    pub content: String,
}

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

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub content: String,
}
