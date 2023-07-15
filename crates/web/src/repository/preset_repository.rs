use sqlx::SqlitePool;

use crate::model::*;

use super::RepositoryResult;

#[derive(Clone)]
pub struct PresetRepository {
    pool: SqlitePool,
}

impl PresetRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // pub async fn create(&self, preset: &CreatePresetSchema) -> RepositoryResult<Preset> {
    //     let preset = sqlx::query_as!(
    //         Preset,
    //         r#"
    //         INSERT INTO presets (name, selected)
    //         VALUES (?, 1)
    //         RETURNING id, name, selected
    //         "#,
    //         preset.name
    //     )
    //     .fetch_one(&self.pool)
    //     .await?;

    //     Ok(preset)
    // }
}
