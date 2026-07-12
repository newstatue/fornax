mod auth;

use axum::extract::Path;
use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

fn router(env: Env) -> Router {
    let api_routes = Router::new()
        .route("/", get(root))
        .route("/hello/{name}", get(hello))
        .route("/get_users", get(auth::get_users))
        .with_state(env);
    Router::new().nest("/api", api_routes)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router(env).call(req).await?)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

pub async fn hello(Path(name): Path<String>) -> String {
    format!("Hello {}!", name)
}