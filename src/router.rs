use axum::{
    routing::get,
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use worker::Env;

use crate::auth;
use crate::openapi::ApiDoc;
use crate::state::AppState;

pub fn create_router(env: Env) -> worker::Result<Router> {
    let state = AppState::new(env)?;

    let api_router = Router::new()
        .route("/", get(root))
        .nest("/auth", auth::router::routes())
        .with_state(state);

    let openapi = ApiDoc::openapi();

    let router = Router::new()
        .nest("/api", api_router)
        .merge(
            SwaggerUi::new("/api/docs/swagger-ui")
                .url("/api/docs/openapi.json", openapi),
        );

    Ok(router)
}

#[utoipa::path(
    get,
    path = "/api/",
    responses(
        (status = 200, description = "服务正常")
    ),
    tag = "General"
)]
async fn root() -> &'static str {
    "Hello!"
}