use std::str::FromStr;
use std::sync::Arc;

use worker::Env;

use crate::auth::repository::UserRepository;
use crate::auth::service::AuthService;
use crate::common::{secret, var};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnv {
    Local,
    Remote,
}

#[derive(Clone)]
pub struct AppState {
    auth_service: AuthService,
}

impl AppState {
    pub fn new(env: Env) -> worker::Result<Self> {
        let db = Arc::new(env.d1("DB")?);
        let kv = Arc::new(env.kv("KV")?);
        let sender = Arc::new(env.send_email("EMAIL")?);

        let app_env = read_app_env(&env);

        let email_from = env
            .var(var::EMAIL_FROM)?
            .to_string();

        let code_expire = read_env_parse(&env, var::EMAIL_CODE_EXPIRE)?;

        let code_cd = read_env_parse(&env, var::AUTH_CODE_CD)?;

        if code_expire == 0 {
            return Err(worker::Error::RustError(format!(
                "{} must be greater than 0",
                var::EMAIL_CODE_EXPIRE,
            )));
        }

        if code_cd == 0 {
            return Err(worker::Error::RustError(format!(
                "{} must be greater than 0",
                var::AUTH_CODE_CD,
            )));
        }

        let jwt_secret = env
            .secret(secret::JWT_SECRET)?
            .to_string();

        let user_repository = UserRepository::new(db);

        let auth_service = AuthService::new(
            user_repository,
            kv,
            sender,
            app_env,
            email_from,
            code_cd,
            code_expire,
            jwt_secret,
        );

        Ok(Self { auth_service })
    }

    pub fn auth_service(&self) -> &AuthService {
        &self.auth_service
    }
}

fn read_app_env(env: &Env) -> AppEnv {
    let value = read_env(env, var::APP_ENV, "remote");

    match value.trim().to_lowercase().as_str() {
        "local" => AppEnv::Local,
        _ => AppEnv::Remote,
    }
}

fn read_env(
    env: &Env,
    key: &str,
    default: &str,
) -> String {
    env.var(key)
        .map(|value| value.to_string())
        .unwrap_or_else(|_| default.to_string())
}


fn read_env_parse<T>(
    env: &Env,
    key: &str,
) -> worker::Result<T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    env.var(key)?
        .to_string()
        .parse::<T>()
        .map_err(|error| {
            worker::Error::RustError(format!(
                "invalid {}: {}",
                key, error
            ))
        })
}