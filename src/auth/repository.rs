use std::sync::Arc;

use worker::{D1Database};
use worker::wasm_bindgen::JsValue;
use crate::auth::entity::UserEntity;

#[derive(Clone)]
pub struct UserRepository {
    db: Arc<D1Database>,
}

impl UserRepository {
    pub fn new(db: Arc<D1Database>) -> Self {
        Self { db }
    }

    pub async fn insert_ignore(
        &self,
        user: &UserEntity,
    ) -> worker::Result<()> {
        let name = user
            .name
            .as_deref()
            .map(JsValue::from_str)
            .unwrap_or(JsValue::NULL);

        self.db
            .prepare(
                r#"
                INSERT OR IGNORE INTO users (
                    id,
                    email,
                    name,
                    status,
                    created_at,
                    updated_at
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
            )
            .bind(&[
                JsValue::from_str(&user.id),
                JsValue::from_str(&user.email),
                name,
                JsValue::from_f64(user.status as f64),
                JsValue::from_f64(user.created_at as f64),
                JsValue::from_f64(user.updated_at as f64),
            ])?
            .run()
            .await?;

        Ok(())
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> worker::Result<Option<UserEntity>> {
        self.db
            .prepare(
                r#"
                SELECT
                    id,
                    email,
                    name,
                    status,
                    created_at,
                    updated_at
                FROM users
                WHERE email = ?1
                LIMIT 1
                "#,
            )
            .bind(&[JsValue::from_str(email)])?
            .first::<UserEntity>(None)
            .await
    }

    pub async fn find_or_create(
        &self,
        email: &str,
    ) -> worker::Result<UserEntity> {
        if let Some(user) = self.find_by_email(email).await? {
            return Ok(user);
        }

        let new_user = UserEntity::new(email.to_owned());

        self.insert_ignore(&new_user).await?;

        self.find_by_email(email)
            .await?
            .ok_or_else(|| {
                worker::Error::RustError(
                    "user not found after insert".to_string(),
                )
            })
    }
}