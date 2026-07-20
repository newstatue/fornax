use crate::auth::map::{SendCodeReq, SendCodeResp};
use crate::auth::{key, message};
use crate::common::{binding, config};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use rand::RngExt;
use worker::{console_error, SendEmailBuilder};

#[worker::send]
pub async fn send_code(
    State(state): State<AppState>,
    Json(request): Json<SendCodeReq>,
)->Result<Json<SendCodeResp>,StatusCode> {
    let code = format!("{:06}",rand::rng().random_range(0..1_000_000));

    let email = request.email.trim().to_lowercase();

    let kv = state.env.kv(binding::KV_BINDING)
        .map_err(|e| {
            console_error!("get kv error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let code_key = format!("{}:{}",key::EMAIL_CODE,email);

    kv.put(&code_key,&code)
        .map_err(|e| {
            console_error!("put kv error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .expiration_ttl(config::EMAIL_CODE_EXPIRE)
        .execute()
        .await
        .map_err(|e| {
            console_error!("save email code error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let sender = state.env.send_email(binding::EMAIL_BINDING)
        .map_err(|e| {
            console_error!("send email error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let text = message::email_text(&code);
    let html = message::email_html(&code);

    let email = SendEmailBuilder::builder(
        config::EMAIL_FROM,
        &email,
        message::EMAIL_SUBJECT
    ).text(&text)
        .html(&html)
        .build();

    sender.send_with_builder(&email)
        .await
        .map_err(|e|{
            console_error!("send email error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(SendCodeResp{
        message: message::EMAIL_SUCCESS.to_string(),
    }))
}