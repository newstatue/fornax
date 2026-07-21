use worker::Env;
use crate::common::var;

#[derive(Clone)]
pub struct AppState {
    pub env: Env,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnv{
    Local,
    Remote
}

impl AppState {
    pub fn app_env(&self) -> AppEnv {
        let value = self
            .env
            .var(var::APP_ENV)
            .map(|value| value.to_string())
            .unwrap_or_else(|_| "remote".to_string());

        match value.to_lowercase().as_str() {
            "local" => AppEnv::Local,
            _ => AppEnv::Remote,
        }
    }
    pub fn db(&self) -> worker::Result<worker::D1Database> {
        self.env.d1("DB")
    }

    pub fn kv(&self) -> worker::Result<worker::KvStore> {
        self.env.kv("KV")
    }

    pub fn sender(&self) -> worker::Result<worker::SendEmail> {
        self.env.send_email("EMAIL")
    }
}