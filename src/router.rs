use crate::auth;
use crate::state::AppState;
use axum::{
    routing::get,
    Router,
};
use utoipa_swagger_ui::SwaggerUi;
use worker::Env;
use utoipa::OpenApi;
use crate::openapi::ApiDoc;

pub fn create_router(env: Env) -> Router {
    let state = AppState { env };

    let api_router = Router::new()
        .route("/", get(root))
        .nest("/auth", auth::router::routes())
        .with_state(state);

    let openapi = ApiDoc::openapi();

    Router::new()
        .nest("/api", api_router)
        .merge(
            SwaggerUi::new("/api/docs/swagger-ui")
                .url("/api/docs/openapi.json", openapi),
        )
}

#[utoipa::path(
    get,
    path="/api/",
    responses(
    (status = 200, description="服务正常")
    ),
    tag = "General"
)]
async fn root() -> String {
    "Hello!".to_string()
}