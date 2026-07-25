use crate::error::AppError;

use sqlx::{Pool, Postgres};

pub type Database = Pool<Postgres>;
pub type Id = String;

pub type AppResult<T> = Result<T, AppError>;
pub type DatabaseResult<T> = Result<T, sqlx::Error>;
