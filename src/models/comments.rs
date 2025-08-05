use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Comments {
    text: String,
    id_user: i32,
    id_post: i32,
    created_at: DateTime<Utc>,
    update_at: DateTime<Utc>,
    is_update: bool,
}

impl Comments {
    pub fn new(id_user: i32, id_post: i32, text: String) -> Self {
        let time = Utc::now();
        Comments {
            text,
            id_post,
            id_user,
            created_at: time,
            update_at: time,
            is_update: false,
        }
    }
    pub fn update_comment(&mut self, text: String) {
        self.text = text;
        self.is_update = true;
        self.update_at = Utc::now();
    }
}