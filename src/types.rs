use sqlx::{Pool, Postgres};

pub type Database = Pool<Postgres>;
pub type Id = String;