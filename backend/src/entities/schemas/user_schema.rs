use sqlx::{FromRow, Error, Row};

pub struct UserSchema {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String
}


impl<'r> FromRow<'r, sqlx::postgres::PgRow> for UserSchema {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            role: row.try_get("role")?,
        })
    }
}