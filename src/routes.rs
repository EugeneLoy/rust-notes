use aide::{
    axum::{
        ApiRouter,
        IntoApiResponse, routing::{delete, get, post},
    },
    openapi::{Info, OpenApi},
};
use axum::{Extension, Json, Router};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum_swagger_ui::swagger_ui;

use crate::repository::Pool;
use crate::rest::{notebooks, notes};

pub fn build_router() -> Router<Pool> {
    let mut open_api = OpenApi {
        info: Info {
            description: Some(String::from("Rust Notes API")),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    ApiRouter::new()
        .api_route("/api/notebooks", post(notebooks::create_notebook))
        .api_route("/api/notebooks/:id", get(notebooks::get_notebook))
        .api_route("/api/notebooks/:id", post(notebooks::update_notebook))
        .api_route("/api/notebooks/:id", delete(notebooks::delete_notebook))
        .api_route("/api/notebooks", get(notebooks::list_notebooks))
        .api_route("/api/notes", post(notes::create_note))
        .api_route("/api/notes/:id", get(notes::get_note))
        .api_route("/api/notes/:id", post(notes::update_note))
        .api_route("/api/notes/:id", delete(notes::delete_note))
        .fallback(fallback_handler)
        .route("/api.json", get(serve_open_api))
        .route("/swagger", get(|| async { Html(swagger_ui("/api.json")) }))
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