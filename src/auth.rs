use axum::extract::State;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use worker::{console_error, Env};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub phone: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[worker::send]
pub async fn get_users(
    State(env): State<Env>,
) -> Result<axum::Json<Vec<User>>, axum::http::StatusCode> {
    let d1 = env.d1("DB").map_err(|e| {
        console_error!("d1 binding error: {:?}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let stmt = d1.prepare("SELECT id, phone, name, created_at FROM users");
    let result = stmt.all().await.map_err(|e| {
        console_error!("d1 query error: {:?}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let users: Vec<User> = result.results::<User>().map_err(|e| {
        console_error!("d1 deserialize error: {:?}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(axum::Json(users))
}