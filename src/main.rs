use std::error::Error;

use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::config::Config;
use crate::repository::build_pool;
use crate::routes::build_router;

mod schema;
mod routes;
mod rest;
mod repository;
mod model;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let config = Config::build();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let pool = build_pool(&config);

    let app = build_router()
        .layer(trace_layer)
        .with_state(pool)
    ;

    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", config.port)).await?;
    println!("Running on http://localhost:{}", config.port);
    axum::serve(listener, app).await?;

    Ok(())
}
