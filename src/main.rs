use std::error::Error;

use tonic::transport::Server;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use rust_notes::config::Config;
use rust_notes::grpc::notes_service::NotesService;
use rust_notes::grpc::proto::notes_service_server::NotesServiceServer;
use rust_notes::repository::build_pool;
use rust_notes::routes::build_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let config = Config::build();

    let pool = build_pool(&config);


    // setup tonic server (handles grpc api)
    let notes_service = NotesService::new(pool.clone());
    let notes_service_address = format!("0.0.0.0:{}",config.grpc_port).parse()?;
    let notes_service_server = Server::builder()
        .add_service(NotesServiceServer::new(notes_service));

    // run grpc in separate task
    let notes_service_server_handle = tokio::spawn(async move {
        println!("Running grpc api on: http://localhost:{}", config.grpc_port);
        notes_service_server.serve(notes_service_address).await
    });


    // setup axum (handles rest api)
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let app = build_router()
        .layer(trace_layer)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", config.rest_port)).await?;
    println!("Running rest api on: http://localhost:{}", config.rest_port);
    println!("See rest api documentation at: http://localhost:{}/swagger", config.rest_port);
    axum::serve(listener, app).await?;


    notes_service_server_handle.await??;

    Ok(())
}
