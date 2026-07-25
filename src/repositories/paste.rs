use crate::{
    models::paste::{CreatePasteRepoDto, Paste, PasteResponse, UpdatePasteRepoDto},
    types::{Database, DatabaseResult, Id},
};

pub struct PasteRepository {
    pub pool: Database,
}

impl PasteRepository {
    pub fn new(pool: Database) -> Self {
        Self { pool }
    }

    pub async fn create_paste(&self, dto: CreatePasteRepoDto) -> DatabaseResult<PasteResponse> {
        sqlx::query_as!(
            PasteResponse,
            "
            Insert into paste (id, title, content, language, one_time, expires_at)
            Values ($1, $2, $3, $4, $5, $6)
            Returning id, views, expires_at;
            ",
            dto.id,
            dto.title,
            dto.content,
            dto.language,
            dto.one_time.unwrap_or(false),
            dto.expires_at
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_pastes(&self, limit: i64, offset: i64) -> DatabaseResult<Vec<PasteResponse>> {
        sqlx::query_as!(
            PasteResponse,
            "
            Select id, views, expires_at from paste
            Order By created_at desc
            Limit $1 Offset $2;
            ",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn view_paste(&self, id: Id) -> DatabaseResult<Option<Paste>> {
        sqlx::query_as!(
            Paste,
            "
            Update paste
            Set views = views + 1
            Where id = $1 And expires_at < Current_timestamp
            Returning id, title, content, language, views, one_time, expires_at
            ",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_paste(
        &self,
        id: Id,
        data: UpdatePasteRepoDto,
    ) -> DatabaseResult<Option<Paste>> {
        sqlx::query_as!(
            Paste,
            "
            Update paste
            Set
                title = Coalesce($2, title),
                content = Coalesce($3, content),
                language = Coalesce($4, language),
                one_time = Coalesce($5, one_time),
                expires_at = Coalesce($6, expires_at)
            Where id = $1
            Returning id, title, content, language, views, one_time, expires_at
            ",
            id,
            data.title,
            data.content,
            data.language,
            data.one_time,
            data.expires_at
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete_paste(&self, id: Id) -> DatabaseResult<Option<Paste>> {
        sqlx::query_as!(
            Paste,
            "
            Delete from paste
            Where id = $1
            Returning id, title, content, language, views, one_time, expires_at
            ",
            id,
        )
        .fetch_optional(&self.pool)
        .await
    }
}
