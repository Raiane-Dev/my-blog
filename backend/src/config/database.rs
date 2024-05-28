use std::env;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres, ConnectOptions};
use lazy_static::lazy_static;


lazy_static! {
    pub static ref INSTANCE: PgPool = {
        let pool = async {
            PgPoolOptions::new()
                .max_connections(5)
                .connect("postgresql://test:test@localhost/test")
                .await
                .expect("Failed to create pool")
        };
        
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(pool)
    };
}