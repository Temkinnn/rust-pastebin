use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct Paste {
    pub id: String,
    pub title: String,
    pub content: String,
    pub language: String,
    pub views: i32,
    pub one_time: bool,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, ToSchema)]
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

#[derive(Serialize, Deserialize, ToSchema)]
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

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetPastesQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
