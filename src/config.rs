
// TODO remove hardcode
pub struct Config {
    pub port: i32,
    pub database_uri: String
}

impl Config {
    pub fn build() -> Config {
        // TODO remove hardcode
        Config {
            port: 3000,
            database_uri: String::from("postgres://postgres:qwe@127.0.0.1/rust_notes")
        }
    }
}
