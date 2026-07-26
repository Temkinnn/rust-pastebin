use tracing::{info, instrument};

use crate::types::Database;

#[derive(Debug)]
pub struct CleanUpService(Database);

impl CleanUpService {
    pub fn new(pool: Database) -> Self {
        Self(pool)
    }

    #[instrument(name = "Cleanup", skip(self))]
    pub async fn cleanup_expired(&self) -> Result<(), sqlx::Error> {
        let result = sqlx::query!("DELETE FROM paste WHERE expires_at < NOW()")
            .execute(&self.0)
            .await?;

        let rows = result.rows_affected();

        info!("Deleted: {}", rows);

        Ok(())
    }
}
