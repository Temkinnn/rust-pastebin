use sqlx::PgPool;

use crate::types::Database;

pub struct DatabasePool;

impl DatabasePool {
    pub async fn init(url: String) -> Database {
        dotenvy::dotenv().expect("Failed to load enviroment variables");

        let pool = PgPool::connect(&url)
            .await
            .expect("Failed to connect to database");
        // let _ = sqlx::migrate!().run(&pool).await;
        pool
    }
}
