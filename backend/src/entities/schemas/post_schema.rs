use sqlx::{FromRow, Error, Row};

pub struct PostSchema {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub image_path: String,
    pub description: String
}


impl<'r> FromRow<'r, sqlx::postgres::PgRow> for PostSchema {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            body: row.try_get("body")?,
            image_path: row.try_get("image_path")?,
            description: row.try_get("description")?,
        })
    }
}