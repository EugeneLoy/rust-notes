use axum::Router;
use axum::routing::{get, post, delete};
use crate::rest::*;


pub fn build_router() -> Router {
    Router::new()
        .route("/notebooks", post(create_notebook))
        .route("/notebooks/:id", get(get_notebook))
        .route("/notebooks/:id", post(update_notebook))
        .route("/notebooks/:id", delete(delete_notebook))
        .route("/notebooks", get(list_notebooks))
}
