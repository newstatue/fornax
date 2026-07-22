use crate::api::ApiResp;
use crate::auth::dto::{
    LoginReq,
    LoginResp,
    SendCodeReq,
    SendCodeResp,
};
use crate::auth::error::AuthError;
use crate::auth::message;
use crate::common::util;
use crate::state::AppState;
use axum::extract::State;
use axum::Json;

#[utoipa::path(
    post,
    path = "/api/auth/send-code",
    tag = "Auth",
    request_body = SendCodeReq,
    responses(
        (status = 200, description = "验证码发送成功", body = ApiResp<SendCodeResp>),
        (status = 400, description = "邮箱格式错误"),
        (status = 429, description = "验证码发送过于频繁"),
        (status = 500, description = "服务器内部错误")
    )
)]
#[worker::send]
pub async fn send_code(
    State(state): State<AppState>,
    Json(request): Json<SendCodeReq>,
) -> Result<Json<ApiResp<SendCodeResp>>, AuthError> {
    util::validate(&request)?;

    let response = state
        .auth_service()
        .send_code(request)
        .await?;

    Ok(Json(ApiResp::success_with_msg(
        message::EMAIL_SUCCESS,
        response,
    )))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Auth",
    request_body = LoginReq,
    responses(
        (status = 200, description = "登录成功", body = ApiResp<LoginResp>),
        (status = 400, description = "邮箱或验证码格式错误"),
        (status = 401, description = "验证码错误或已过期"),
        (status = 403, description = "用户已被停用"),
        (status = 500, description = "服务器内部错误")
    )
)]
#[worker::send]
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginReq>,
) -> Result<Json<ApiResp<LoginResp>>, AuthError> {
    util::validate(&request)?;

    let response = state
        .auth_service()
        .login(request)
        .await?;

    Ok(Json(ApiResp::success_with_msg(
        message::LOGIN_SUCCESS,
        response,
    )))
}