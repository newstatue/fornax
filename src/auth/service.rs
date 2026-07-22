use std::sync::Arc;

use chrono::Utc;
use rand::RngExt;
use worker::{
    console_log,
    KvStore,
    SendEmail,
    SendEmailBuilder,
};

use crate::auth::dto::{
    LoginReq,
    LoginResp,
    SendCodeReq,
    SendCodeResp,
};
use crate::auth::error::AuthError;
use crate::auth::repository::UserRepository;
use crate::auth::{key, message};
use crate::common::jwt;
use crate::state::AppEnv;

#[derive(Clone)]
pub struct AuthService {
    user_repository: UserRepository,
    kv: Arc<KvStore>,
    sender: Arc<SendEmail>,

    app_env: AppEnv,
    email_from: String,
    code_cd: u64,
    code_expire: u64,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(
        user_repository: UserRepository,
        kv: Arc<KvStore>,
        sender: Arc<SendEmail>,
        app_env: AppEnv,
        email_from: String,
        code_cd: u64,
        code_expire: u64,
        jwt_secret: String,
    ) -> Self {
        Self {
            user_repository,
            kv,
            sender,
            app_env,
            email_from,
            code_cd,
            code_expire,
            jwt_secret,
        }
    }

    pub async fn send_code(
        &self,
        request: SendCodeReq,
    ) -> Result<SendCodeResp, AuthError> {
        // DTO 层已经使用 garde 完成格式校验，
        // Service 层只对邮箱进行统一化处理。
        let email = normalize_email(&request.email);

        // 发送前检查冷却时间。
        self.check_code_cd(&email).await?;

        // 生成五位数字验证码：10000～99999。
        let code = rand::rng()
            .random_range(10_000..100_000)
            .to_string();

        if self.app_env == AppEnv::Local {
            console_log!(
                "local email code: {code}, email: {email}"
            );
        } else {
            // 生产环境先发送邮件。
            self.send_code_email(&email, &code).await?;
        }

        // 邮件发送成功后再保存验证码。
        self.save_code(&email, &code).await?;

        // 设置发送冷却时间。
        self.save_code_cd(&email).await?;

        Ok(SendCodeResp {
            cd: self.code_cd,
        })
    }

    pub async fn login(
        &self,
        request: LoginReq,
    ) -> Result<LoginResp, AuthError> {
        // DTO 层负责字段格式校验。
        let email = normalize_email(&request.email);

        // 判断验证码是否存在、过期或不正确。
        self.verify_code(&email, &request.code).await?;

        let user = self
            .user_repository
            .find_or_create(&email)
            .await?;

        if !user.is_active() {
            return Err(AuthError::UserDisabled);
        }

        let token = jwt::generate_token(
            &user.id,
            &self.jwt_secret,
        )
            .map_err(|error| {
                AuthError::Jwt(error.to_string())
            })?;

        // 只有全部登录流程成功后才删除验证码。
        self.delete_code(&email).await?;

        Ok(LoginResp { token })
    }

    async fn verify_code(
        &self,
        email: &str,
        input_code: &str,
    ) -> Result<(), AuthError> {
        let code_key = key::email_code(email);

        let saved_code = self
            .kv
            .get(&code_key)
            .text()
            .await?;

        let Some(saved_code) = saved_code else {
            return Err(AuthError::CodeExpired);
        };

        if saved_code != input_code {
            return Err(AuthError::InvalidCode);
        }

        Ok(())
    }

    async fn save_code_cd(
        &self,
        email: &str,
    ) -> Result<(), AuthError> {
        let code_cd_key = key::email_code_cd(email);

        let next_send_at =
            Utc::now().timestamp() + self.code_cd as i64;

        self.kv
            .put(
                &code_cd_key,
                next_send_at.to_string(),
            )?
            .expiration_ttl(self.code_cd)
            .execute()
            .await?;

        Ok(())
    }

    async fn check_code_cd(
        &self,
        email: &str,
    ) -> Result<(), AuthError> {
        let code_cd_key = key::email_code_cd(email);

        let next_send_at = self
            .kv
            .get(&code_cd_key)
            .text()
            .await?;

        let Some(next_send_at) = next_send_at else {
            return Ok(());
        };

        let next_send_at = next_send_at
            .parse::<i64>()
            .map_err(|error| {
                AuthError::Config(format!(
                    "invalid cooldown timestamp: {error}"
                ))
            })?;

        let now = Utc::now().timestamp();

        let remaining =
            (next_send_at - now).max(0) as u64;

        if remaining > 0 {
            return Err(AuthError::TooManyRequests {
                retry_after_secs: remaining,
            });
        }

        Ok(())
    }

    async fn save_code(
        &self,
        email: &str,
        code: &str,
    ) -> Result<(), AuthError> {
        let code_key = key::email_code(email);

        self.kv
            .put(&code_key, code)?
            .expiration_ttl(self.code_expire)
            .execute()
            .await?;

        Ok(())
    }

    async fn delete_code(
        &self,
        email: &str,
    ) -> Result<(), AuthError> {
        let code_key = key::email_code(email);

        self.kv
            .delete(&code_key)
            .await?;

        Ok(())
    }

    async fn send_code_email(
        &self,
        email_address: &str,
        code: &str,
    ) -> Result<(), AuthError> {
        let text = message::email_text(code);
        let html = message::email_html(code);

        let email = SendEmailBuilder::builder(
            &self.email_from,
            email_address,
            message::EMAIL_SUBJECT,
        )
            .text(&text)
            .html(&html)
            .build();

        self.sender
            .send_with_builder(&email)
            .await
            .map_err(|error| {
                AuthError::Email(error.to_string().into())
            })?;

        Ok(())
    }
}

/// 统一邮箱格式
fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}