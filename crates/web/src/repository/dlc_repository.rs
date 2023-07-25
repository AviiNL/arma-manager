use api_schema::{request::*, response::Dlc};
use sqlx::{QueryBuilder, SqlitePool};

use super::RepositoryResult;

#[derive(Clone)]
pub struct DlcRepository {
    pool: SqlitePool,
}

impl DlcRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl DlcRepository {
    pub async fn get_dlcs(&self) -> RepositoryResult<Vec<Dlc>> {
        let result = sqlx::query_as!(
            Dlc,
            r#"
            SELECT *
            FROM dlc
            ORDER BY id ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
}
