use crate::types::Database;

#[derive(Debug)]
pub struct CleanUpService(Database);

impl CleanUpService {
    pub fn new(pool: Database) -> Self {
        Self(pool)
    }

    pub async fn cleanup_expired(&self) -> Result<(), sqlx::Error> {
        let result = sqlx::query!("DELETE FROM paste WHERE expires_at < NOW()")
            .execute(&self.0)
            .await?;

        println!("[cleanup]: Deleted: {}", result.rows_affected());

        Ok(())
    }
}
