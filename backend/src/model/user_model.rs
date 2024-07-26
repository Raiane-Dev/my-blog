use sqlx::postgres::PgRow;

use crate::entities::{
    io::input::user_entity::UserInput, 
    schemas::user_schema::{self, UserSchema}
};
use std::error::Error;
use crate::config::database::INSTANCE;
use crate::config::database::get_db_pool;

pub async fn create(data: UserInput) -> Result<PgRow, sqlx::Error> {
    let pool = get_db_pool().await;

    let created_id = sqlx::query(
        r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
    )
    .bind(data.username)
    .bind(data.email)
    .bind(data.password)
    .fetch_one(pool)
    .await?;

    Ok(created_id)
}

pub async fn select_one(condition: String, args: Vec<&str>) -> Result<UserSchema, sqlx::Error> {

    let pool = get_db_pool().await;

    let query_str = format!("SELECT * FROM users WHERE {}", condition);

    let mut query = sqlx::query_as::<_, UserSchema>(&query_str);

    for arg in args {
        query = query.bind(arg);
    }

    let result = query.fetch_one(pool).await;

    result
}
