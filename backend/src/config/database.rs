use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, PgConnectOptions};
use lazy_static::lazy_static;


lazy_static! {
    static ref POOL: PgPool = {
        let opt = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("test")
        .password("test")
        .database("test")
    };

    PgPoolOptions::new()
    .max_connections(5)
    .connect_with(opt)
    .await
    .unwrap()
}