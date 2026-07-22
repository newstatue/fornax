use std::collections::HashMap;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResp<T> {
    pub code: u16,
    pub msg: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<String>>>,
}

impl<T> ApiResp<T> {
    // pub fn success(data: T) -> Self {
    //     Self {
    //         code: StatusCode::OK.as_u16(),
    //         msg: "success".to_string(),
    //         data: Some(data),
    //     }
    // }

    pub fn success_with_msg(
        msg: impl Into<String>,
        data: T,
    ) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            msg: msg.into(),
            data: Some(data),
            errors: None,
        }
    }

}

impl ApiResp<()> {
    // pub fn ok(msg: impl Into<String>) -> Self {
    //     Self {
    //         code: StatusCode::OK.as_u16(),
    //         msg: msg.into(),
    //         data: None,
    //     }
    // }

    // pub fn fail(
    //     status: StatusCode,
    //     msg: impl Into<String>,
    // ) -> Self {
    //     Self {
    //         code: status.as_u16(),
    //         msg: msg.into(),
    //         data: None,
    //     }
    // }

    pub fn from_error<E: ApiError>(error: &E) -> Self {
        Self {
            code: error.code().as_u16(),
            msg: error.msg().to_string(),
            data: None,
            errors: None,
        }
    }

    pub fn validation(
        errors: HashMap<String, Vec<String>>,
    ) -> Self {
        Self{
            code: StatusCode::BAD_REQUEST.as_u16(),
            msg: "请求参数校验失败".to_string(),
            data: None,
            errors: Some(errors),
        }
    }
}

pub trait ApiError {
    fn code(&self) -> StatusCode;

    fn msg(&self) -> &'static str;
}