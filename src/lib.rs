use tower_service::Service;
use worker::*;

mod auth;
mod common;
mod router;
mod state;
pub mod openapi;
mod api;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let mut router = router::create_router(env)?;

    router
        .call(req)
        .await
        .map_err(|e| Error::RustError(e.to_string()))
}