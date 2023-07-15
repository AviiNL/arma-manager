use api_schema::request::*;
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

    pub async fn create(&self, preset: &CreatePresetSchema) -> RepositoryResult<Preset> {
        let existing = sqlx::query_as!(
            CreatePresetSchema,
            r#"
            SELECT id, name, selected FROM presets WHERE name = ?
            "#,
            preset.name
        )
        .fetch_optional(&self.pool)
        .await?;

        let preset = if let Some(existing) = existing {
            // delete all items
            sqlx::query!(
                r#"
                DELETE FROM preset_items WHERE preset_id = ?
                "#,
                existing.id
            );

            existing
        } else {
            let preset = sqlx::query_as!(
                Preset,
                r#"
                INSERT INTO presets (name, selected)
                VALUES (?, 1)
                RETURNING id, name, selected
                "#,
                preset.name
            )
            .fetch_one(&self.pool)
            .await?;

            preset
        };

        // add items
        for item in &preset.items {
            sqlx::query_as!(
                PresetItemSchema
                r#"
                INSERT INTO preset_items (preset_id, name, published_file_id, position)
                VALUES (?, ?, ?, ?, ?)
                "#,
                preset.id,
                item.name,
                item.value
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(preset)
    }
}
