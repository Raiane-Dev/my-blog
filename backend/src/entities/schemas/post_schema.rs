use chrono::{DateTime, TimeZone, Utc};

pub struct PostSchema {
    pub id: i32,
    pub title: String,
    pub body: String,
}