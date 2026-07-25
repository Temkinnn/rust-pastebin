use chrono::{Duration, NaiveDateTime, Utc};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::paste::{
        CreatePasteDto, CreatePasteRepoDto, Paste, UpdatePasteDto, UpdatePasteRepoDto,
    },
    repositories::paste::PasteRepository,
    types::{AppResult, Id},
};

pub struct PasteService(PasteRepository);

impl PasteService {
    pub fn new(repo: PasteRepository) -> Self {
        Self(repo)
    }

    pub async fn create_paste(&self, data: CreatePasteDto) -> AppResult<Paste> {
        let id = Uuid::now_v7().to_string();

        let CreatePasteDto {
            title,
            content,
            language,
            one_time,
            expires_in_hours,
        } = data;

        let expires_at: Option<NaiveDateTime> = match expires_in_hours {
            None => None,
            Some(hours) => Some(Utc::now().naive_utc() + Duration::hours(hours)),
        };

        Ok(self
            .0
            .create_paste(CreatePasteRepoDto {
                id,
                title,
                content,
                language,
                one_time,
                expires_at,
            })
            .await?)
    }

    pub async fn get_pastes(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> AppResult<Vec<Paste>> {
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(0);
        Ok(self.0.get_pastes(limit, offset).await?)
    }

    pub async fn get_paste_by_id(&self, id: Id) -> AppResult<Paste> {
        self.0.view_paste(id).await?.ok_or(AppError::NotFound)
    }

    pub async fn update_paste_data(&self, id: Id, data: UpdatePasteDto) -> AppResult<Paste> {
        let UpdatePasteDto {
            title,
            content,
            language,
            one_time,
            expires_in_hours,
        } = data;

        let expires_at: Option<NaiveDateTime> = match expires_in_hours {
            None => None,
            Some(hours) => Some(Utc::now().naive_utc() + Duration::hours(hours)),
        };

        self.0
            .update_paste(
                id,
                UpdatePasteRepoDto {
                    title,
                    content,
                    language,
                    one_time,
                    expires_at,
                },
            )
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn delete_paste_by_id(&self, id: Id) -> AppResult<Paste> {
        self.0.delete_paste(id).await?.ok_or(AppError::NotFound)
    }
}
