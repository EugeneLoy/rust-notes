use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post, delete};
use crate::rest::*;


pub fn build_router() -> Router {
    Router::new()
        .route("/api/notebooks", post(create_notebook))
        .route("/api/notebooks/:id", get(get_notebook))
        .route("/api/notebooks/:id", post(update_notebook))
        .route("/api/notebooks/:id", delete(delete_notebook))
        .route("/api/notebooks", get(list_notebooks))
        .fallback(fallback_handler)
}

async fn fallback_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}