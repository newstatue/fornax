use serde::Deserialize;
use worker::wasm_bindgen::JsValue;

use crate::state::AppState;

#[derive(Debug,Deserialize)]
pub struct UserEntity {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone)]
pub struct UserRepository {
    state: AppState,
}

impl UserRepository {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn insert(&self, user: &UserEntity) -> worker::Result<()> {
        let db = self.state.db()?;

        let name = match &user.name {
            Some(name) => JsValue::from_str(name),
            None => JsValue::NULL,
        };

        db.prepare(
            r#"
            insert into users (
                id,
                email,
                name,
                status,
                updated_at,
                created_at
            )
            values (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
            .bind(&[
                JsValue::from_str(&user.id),
                JsValue::from_str(&user.email),
                name,
                JsValue::from_f64(user.status as f64),
                JsValue::from_f64(user.updated_at as f64),
                JsValue::from_f64(user.created_at as f64),
            ])?
            .run()
            .await?;

        Ok(())
    }


    pub async fn find_by_email(&self, email: &str, ) -> worker::Result<Option<UserEntity>> {
        let db = self.state.db()?;

        db.prepare(
            r#"
        select
            id,
            email,
            name,
            status,
            created_at,
            updated_at
        from users
        where email = ?1
        "#,
        )
            .bind(&[
                JsValue::from_str(email),
            ])?
            .first::<UserEntity>(None)
            .await
    }
}