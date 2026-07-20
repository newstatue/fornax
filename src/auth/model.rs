/*use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum UserStatus{
    Active = 0,
    Inactive = 1,
}

impl User {
    pub fn new(email: impl Into<String>, name: Option<String>, status: UserStatus) -> Self {
        let now = Utc::now().timestamp();

        Self{
            id: Uuid::new_v4().to_string(),
            email: email.into(),
            name,
            status: status as i32,
            created_at: now,
            updated_at: now,
        }
    }
}*/