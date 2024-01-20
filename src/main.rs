use axum::Router;
use axum::routing::get;

// TODO remove hardcode
struct Config {
    port: i32
}

impl Config {
    fn build() -> Config {
        // TODO remove hardcode
        Config {
            port: 3000
        }
    }
}


#[tokio::main]
async fn main() {
    let config = Config::build();

    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));

    println!("Running on http://localhost:{}", config.port);
    axum::Server::bind(&format!("0.0.0.0:{}", config.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}