use aide::{
    axum::{
        ApiRouter,
        IntoApiResponse, routing::{delete, get, post},
    },
    openapi::{Info, OpenApi},
};
use aide::redoc::Redoc;
use axum::{Extension, Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::rest::*;

pub fn build_router() -> Router {
    let mut open_api = OpenApi {
        info: Info {
            description: Some(String::from("Rust Notes API")),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    ApiRouter::new()
        .api_route("/api/notebooks", post(create_notebook))
        .api_route("/api/notebooks/:id", get(get_notebook))
        .api_route("/api/notebooks/:id", post(update_notebook))
        .api_route("/api/notebooks/:id", delete(delete_notebook))
        .api_route("/api/notebooks", get(list_notebooks))
        .fallback(fallback_handler)
        .route("/api.json", get(serve_open_api))
        .route("/redoc", Redoc::new("/api.json").axum_route())
        .finish_api(&mut open_api)
        .layer(Extension(open_api))
}

async fn fallback_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

async fn serve_open_api(Extension(open_api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(open_api)
}