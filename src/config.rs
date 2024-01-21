use std::env;

pub struct Config {
    pub port: i32,
    pub database_uri: String
}

impl Config {
    pub fn build() -> Config {
        // TODO remove hardcode, handle errors
        Config {
            port: 3000,
            database_uri: env::var("DATABASE_URL").expect("DATABASE_URL is not set")
        }
    }
}
