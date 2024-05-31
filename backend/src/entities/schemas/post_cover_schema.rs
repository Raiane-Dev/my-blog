use sqlx::{FromRow, Error, Row};

pub struct PostCoverSchema {
    pub id: i32,
    pub post_id: i32,
    pub headline: String,
    pub description: String,
    pub image_path: String,
}


impl<'r> FromRow<'r, sqlx::postgres::PgRow> for PostCoverSchema {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            post_id: row.try_get("post_id")?,
            headline: row.try_get("headline")?,
            description: row.try_get("description")?,
            image_path: row.try_get("image_path")?,
        })
    }
}