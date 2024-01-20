use diesel_async::{AsyncPgConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use crate::config::Config;

pub type Pool = diesel_async::pooled_connection::deadpool::Pool<AsyncPgConnection>;

pub fn build_pool(config: &Config) -> Pool {
    let connection_manager: AsyncDieselConnectionManager<AsyncPgConnection> = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&config.database_uri);
    diesel_async::pooled_connection::deadpool::Pool::builder(connection_manager).build().unwrap()
}