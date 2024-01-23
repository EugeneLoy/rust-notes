use std::env;

pub struct Config {
    pub rest_port: i32,
    pub grpc_port: i32,
    pub database_uri: String
}

impl Config {
    pub fn build() -> Config {
        // TODO remove hardcode, handle errors
        Config {
            rest_port: 3000,
            grpc_port: 3001,
            database_uri: env::var("DATABASE_URL").unwrap_or(String::from("postgres://postgres:qwe@127.0.0.1/rust_notes"))
        }
    }
}
