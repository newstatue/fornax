use axum::{
    extract::Path,
    routing::get,
    Router,
};
use worker::Env;
use crate::auth;
use crate::state::AppState;

pub fn create_router(env: Env) -> Router {
    let state = AppState { env };

    let api_router = Router::new()
        .route("/", get(root))
        .route("/hello/{name}", get(hello))
        .nest("/auth",auth::router::routes())
        .with_state(state);

    Router::new().nest("/api", api_router)
}

async fn root() -> &'static str {
    "Hello Axum!"
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello {}!", name)
}