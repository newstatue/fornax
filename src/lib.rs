mod auth;

use axum::{routing::get, Router};
use axum::extract::Path;
use tower_service::Service;
use worker::*;

fn router() -> Router {
    let api_routes = Router::new()
        .route("/", get(root))
        .route("/hello/{name}", get(hello));
    Router::new().nest("/api", api_routes)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

pub async fn hello(Path(name):Path<String>) -> String {
    format!("Hello {}!", name)
}
