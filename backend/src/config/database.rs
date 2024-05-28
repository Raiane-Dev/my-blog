use lazy_static::lazy_static;
use sqlx::{postgres::PgPoolOptions, ConnectOptions, PgPool, Pool, Postgres};
use std::env;
use tokio::sync::OnceCell;


lazy_static! {
    pub static ref INSTANCE: OnceCell<PgPool> = OnceCell::const_new();
}

async fn initialize_db_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://test:test@localhost/test")
        .await
        .expect("Failed to create pool")
}

pub async fn get_db_pool() -> &'static PgPool {
    INSTANCE.get_or_init(initialize_db_pool).await
}