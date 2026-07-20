use tower_service::Service;
use worker::*;

mod auth;
mod common;
mod router;
mod state;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router::create_router(env).call(req).await?)
}