use std::error::Error;

use axum::extract::Extension;

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

    let pool = build_pool(&config);

    let app = build_router()
        .layer(Extension(pool));

    println!("Running on http://localhost:{}", config.port);
    axum::Server::bind(&format!("0.0.0.0:{}", config.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
