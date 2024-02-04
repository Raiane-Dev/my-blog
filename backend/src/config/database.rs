use std::env;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[tokio::main]
pub async fn connect() -> Pool<Postgres> {

    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    return pool;
}