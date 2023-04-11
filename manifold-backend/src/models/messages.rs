use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub content: String,
}
