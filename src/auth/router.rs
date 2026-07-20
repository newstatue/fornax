use axum::{
    routing::post,
    Router,
};

use crate::auth::handler;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/send-code", post(handler::send_code))
}