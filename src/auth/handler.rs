use crate::auth::dto::{LoginReq, LoginResp, SendCodeReq, SendCodeResp};
use crate::auth::repository::UserRepository;
use crate::auth::{key, message, repository};
use crate::common::{jwt, secret, var};
use crate::state::{AppEnv, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use rand::RngExt;
use uuid::Uuid;
use worker::{console_error, console_log, SendEmailBuilder};

#[utoipa::path(
    post,
    path = "/api/auth/send-code",
    tag = "Auth",
    request_body = SendCodeReq,
    responses(
        (status = 200, description = "验证码发送成功", body = SendCodeResp),
        (status = 400, description = "邮箱不能为空"),
        (status = 500, description = "服务器内部错误")
    )
)]
#[worker::send]
pub async fn send_code(
    State(state): State<AppState>,
    Json(request): Json<SendCodeReq>,
)->Result<Json<SendCodeResp>,StatusCode> {
    let code = rand::rng().random_range(100_000..1_000_000);
    let email_addr = request.email.trim().to_lowercase();
    if email_addr.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let kv = state.kv()
        .map_err(|e|{
            console_log!("get kv error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let code_key = key::email_code(&email_addr);

    let expire = state.env
        .var(var::EMAIL_CODE_EXPIRE)
        .map_err(|e|{
            console_log!("get EMAIL_CODE_EXPIRE error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .to_string()
        .parse::<u64>()
        .map_err(|e|{
            console_log!("parse EMAIL_CODE_EXPIRE to u64 error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    kv.put(&code_key, &code.to_string())
        .map_err(|e|{
            console_error!("create kv put builder error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .expiration_ttl(expire)
        .execute()
        .await
        .map_err(|e|{
            console_error!("put code kv error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if state.app_env() == AppEnv::Local {
        console_log!("Sending code {code} to {email_addr}");

        return  Ok(Json(SendCodeResp {
            msg: message::EMAIL_SUCCESS.to_string(),
        }))
    }

    let sender = state.sender()
        .map_err(|e|{
            console_error!("get email sender error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let text = message::email_text(&code.to_string());
    let html = message::email_html(&code.to_string());

    let email = SendEmailBuilder::builder(
        &var::EMAIL_FROM,
        &email_addr,
        message::EMAIL_SUBJECT,
    ).text(&text)
        .html(&html)
        .build();

    sender.send_with_builder(&email)
        .await
        .map_err(|e| {
        console_error!("send email error: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(SendCodeResp{
        msg: message::EMAIL_SUCCESS.to_string(),
    }))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Auth",
    request_body = LoginReq,
    responses(
        (status = 200, description = "登录成功", body = LoginResp),
        (status = 400, description = "邮箱或验证码不能为空"),
        (status = 401, description = "验证码错误或已过期"),
        (status = 500, description = "服务器内部错误")
    )
)]
#[worker::send]
pub async fn login(
    State(state): State<AppState>,
    Json(request) : Json<LoginReq>
) -> Result<Json<LoginResp>, StatusCode> {
    let email = request.email.trim().to_lowercase();
    let code = request.code.trim().to_string();

    if email.is_empty() || code.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let kv = state.kv().map_err(|e| {
        console_error!("get kv error {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let code_key = key::email_code(&email);

    let cached_code = kv
        .get(&code_key)
        .text()
        .await
        .map_err(|e|{
            console_error!("get key error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if cached_code != code {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user_repository = UserRepository::new(state.clone());

    let user =  user_repository.find_by_email(&email)
        .await
        .map_err(|e| {
            console_error!("get user error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let user_id = match user {
        Some(user) => user.id,
        None => {
            // 用户不存在, 新建用户
            let user = repository::UserEntity {
                id: Uuid::new_v4().to_string(),
                email: email.clone(),
                name: None,
                status: 0,
                created_at: Utc::now().timestamp(),
                updated_at: Utc::now().timestamp(),
            };

            user_repository.insert(&user)
                .await
                .map_err(|e|{
                    console_error!("insert user error {e}");
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            user.id
        }
    };

    kv.delete(&code_key)
        .await
        .map_err(|e|{
            console_error!("delete key error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let jwt_secret = state
        .env
        .secret(secret::JWT_SECRET)
        .map_err(|e| {
            console_error!("get jwt secret error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    .to_string();

    let token = jwt::generate_token(&user_id,&jwt_secret)
        .map_err(|e| {
            console_error!("generate token error {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(LoginResp{
        token,
    }))
}