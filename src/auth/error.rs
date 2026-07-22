use std::collections::HashMap;

use axum::http::{
    header,
    HeaderValue,
    StatusCode,
};
use axum::response::{
    IntoResponse,
    Response,
};
use axum::Json;
use worker::console_error;

use crate::api::{
    ApiError,
    ApiResp,
};
use crate::common::util;

#[derive(Debug)]
pub enum AuthError {
    Validation {
        errors: HashMap<String, Vec<String>>,
    },

    InvalidCode,
    CodeExpired,
    UserDisabled,

    TooManyRequests {
        retry_after_secs: u64,
    },

    Worker(worker::Error),
    Kv(worker::KvError),
    Email(String),
    Jwt(String),
    Config(String),
}

impl ApiError for AuthError {
    fn code(&self) -> StatusCode {
        match self {
            Self::Validation { .. }
            | Self::InvalidCode => StatusCode::BAD_REQUEST,

            Self::CodeExpired => {
                StatusCode::UNAUTHORIZED
            }

            Self::UserDisabled => {
                StatusCode::FORBIDDEN
            }

            Self::TooManyRequests { .. } => {
                StatusCode::TOO_MANY_REQUESTS
            }

            _ => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn msg(&self) -> &'static str {
        match self {
            Self::Validation { .. } => {
                "请求参数校验失败"
            }

            Self::InvalidCode => {
                "验证码错误"
            }

            Self::CodeExpired => {
                "验证码已过期"
            }

            Self::UserDisabled => {
                "用户已禁用"
            }

            Self::TooManyRequests { .. } => {
                "验证码发送过于频繁"
            }

            Self::Worker(_) => {
                "服务器内部错误"
            }

            Self::Kv(_) => {
                "服务器内部错误"
            }

            Self::Email(_) => {
                "邮件发送失败"
            }

            Self::Jwt(_) => {
                "服务器内部错误"
            }

            Self::Config(_) => {
                "服务器配置错误"
            }
        }
    }
}

impl From<worker::Error> for AuthError {
    fn from(error: worker::Error) -> Self {
        Self::Worker(error)
    }
}

impl From<worker::KvError> for AuthError {
    fn from(error: worker::KvError) -> Self {
        Self::Kv(error)
    }
}
impl From<garde::Report> for AuthError {
    fn from(report: garde::Report) -> Self {
        Self::Validation {
            errors: util::report_to_map(report),
        }
    }
}
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = self.code();

        match &self {
            Self::Worker(error) => {
                console_error!(
                    "auth worker error: {error}"
                );
            }

            Self::Kv(error) => {
                console_error!(
                    "auth kv error: {error}"
                );
            }

            Self::Email(error) => {
                console_error!(
                    "auth email error: {error}"
                );
            }

            Self::Jwt(error) => {
                console_error!(
                    "auth jwt error: {error}"
                );
            }

            Self::Config(error) => {
                console_error!(
                    "auth config error: {error}"
                );
            }

            _ => {}
        }

        let retry_after_secs = match &self {
            Self::TooManyRequests {
                retry_after_secs,
            } => Some(*retry_after_secs),

            _ => None,
        };

        let body = match self {
            Self::Validation { errors } => {
                ApiResp::<()>::validation(errors)
            }

            error => {
                ApiResp::<()>::from_error(&error)
            }
        };

        let mut response =
            (status, Json(body)).into_response();

        if let Some(seconds) = retry_after_secs {
            if let Ok(value) =
                HeaderValue::from_str(
                    &seconds.to_string(),
                )
            {
                response
                    .headers_mut()
                    .insert(
                        header::RETRY_AFTER,
                        value,
                    );
            }
        }

        response
    }
}