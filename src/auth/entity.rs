use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct UserEntity {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl UserEntity {
    pub fn new(email: String) -> Self {
        let now = Utc::now().timestamp();

        Self {
            id: Uuid::new_v4().to_string(),
            email,
            name: None,
            status: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == 0
    }
}