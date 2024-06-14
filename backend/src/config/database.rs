use lazy_static::lazy_static;
use sqlx::{postgres::PgPoolOptions, ConnectOptions, PgPool, Pool, Postgres};
use std::env;
use tokio::sync::OnceCell;


lazy_static! {
    pub static ref INSTANCE: OnceCell<PgPool> = OnceCell::const_new();
}

async fn initialize_db_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn get_db_pool() -> &'static PgPool {
    INSTANCE.get_or_init(initialize_db_pool).await
}