use time::PlainDateTime;

pub struct Paste {
    pub id: String,
    pub title: String,
    pub content: String,
    pub language: String,
    pub views: i32,
    pub one_time: bool,
    pub expires_at: Option<PlainDateTime>,
}

pub struct CreatePasteDto {
    pub title: String,
    pub content: String,
    pub language: String,
    pub one_time: Option<bool>,
    pub expires_at: Option<PlainDateTime>,
}

pub struct CreatePasteRepoDto {
    pub id: String,
    pub title: String,
    pub content: String,
    pub language: String,
    pub one_time: Option<bool>,
    pub expires_at: Option<PlainDateTime>,
}

pub struct UpdatePasteDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub language: Option<String>,
    pub one_time: Option<bool>,
    pub expires_at: Option<PlainDateTime>,
}
