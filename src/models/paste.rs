use chrono::{NaiveDateTime};

pub struct Paste {
    pub id: String,
    pub title: String,
    pub content: String,
    pub language: String,
    pub views: i32,
    pub one_time: bool,
    pub expires_at: Option<NaiveDateTime>,
}

pub struct CreatePasteDto {
    pub title: String,
    pub content: String,
    pub language: String,
    pub one_time: Option<bool>,
    pub expires_in_hours: Option<i64>,
}

pub struct CreatePasteRepoDto {
    pub id: String,
    pub title: String,
    pub content: String,
    pub language: String,
    pub one_time: Option<bool>,
    pub expires_at: Option<NaiveDateTime>,
}

pub struct UpdatePasteDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub language: Option<String>,
    pub one_time: Option<bool>,
    pub expires_in_hours: Option<i64>,
}

pub struct UpdatePasteRepoDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub language: Option<String>,
    pub one_time: Option<bool>,
    pub expires_at: Option<NaiveDateTime>,
}
