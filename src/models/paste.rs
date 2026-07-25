pub struct Paste {
    id: String,
    title: String,
    content: String,
    language: String,
    views: i32,
    one_time: bool,
}

pub struct CreatePasteDto {
    title: String,
    content: String,
    language: String,
    one_time: Option<bool>,
}


pub struct UpdatePasteDto {
    title: Option<String>,
    content: Option<String>,
    language: Option<String>,
    one_time: Option<bool>,
}

