use lazy_static::lazy_static;
use sqlx::{postgres::PgPoolOptions, postgres::PgConnectOptions, ConnectOptions, PgPool, Pool, Postgres};
use std::time::Duration;
use std::env;
use tokio::sync::OnceCell;

lazy_static! {
    pub static ref INSTANCE: OnceCell<PgPool> = OnceCell::const_new();
}

async fn initialize_db_pool() -> PgPool {
    let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let pass = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

    let options = PgConnectOptions::new()
        .host(&host)
        .port(5432)
        .username(&user)
        .password(&pass)
        .database(&db);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await;

    return pool.unwrap();
}

pub async fn get_db_pool() -> &'static PgPool {
    INSTANCE.get_or_init(initialize_db_pool).await
}
