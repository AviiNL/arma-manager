use api_schema::{
    request::*,
    response::{Preset, PresetItem},
};
use sqlx::SqlitePool;

use super::RepositoryResult;

#[derive(Clone)]
pub struct PresetRepository {
    pool: SqlitePool,
}

impl PresetRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: &CreatePresetSchema) -> RepositoryResult<Preset> {
        let preset = sqlx::query_as!(
            SqlPreset,
            r#"
            INSERT INTO presets (name, selected)
            VALUES (?, ?)
            ON CONFLICT (name) DO UPDATE SET updated_at = CURRENT_TIMESTAMP
            RETURNING id, name, selected
            "#,
            input.name,
            false
        )
        .fetch_one(&self.pool)
        .await?;

        // set as selected
        self.select(preset.id).await?;

        // clear items if they exist
        sqlx::query!(
            r#"
            DELETE FROM preset_items WHERE preset_id = ?
            "#,
            preset.id
        )
        .execute(&self.pool)
        .await?;

        let mut items = vec![];
        // add items
        for item in &input.items {
            let item = sqlx::query_as!(
                PresetItem,
                r#"
                INSERT INTO preset_items (preset_id, name, published_file_id, position, enabled)
                VALUES (?, ?, ?, ?, ?)
                RETURNING id, name, published_file_id, position, enabled
                "#,
                preset.id,
                item.name,
                item.published_file_id,
                item.position,
                item.enabled
            )
            .fetch_one(&self.pool)
            .await?;

            items.push(item);
        }

        let preset = Preset {
            id: preset.id,
            name: preset.name,
            selected: preset.selected.is_some(),
            items: items,
        };

        Ok(preset)
    }

    async fn select(&self, id: i64) -> RepositoryResult<Preset> {
        // unselect all
        sqlx::query(
            r#"
            UPDATE presets SET selected = NULL WHERE selected = 1
            "#,
        )
        .execute(&self.pool)
        .await?;

        // select one
        let preset = sqlx::query_as!(
            SqlPreset,
            r#"
            UPDATE presets SET selected = 1 WHERE id = ?
            RETURNING id as "id!", name, selected
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        let items = vec![]; // self.get_items(preset.id).await?;

        Ok(Preset {
            id: preset.id,
            name: preset.name,
            selected: preset.selected.is_some(),
            items,
        })
    }

    // async fn get_items(&self, preset_id: i64) -> RepositoryResult<Vec<PresetItem>> {
    //     let items = sqlx::query_as!(
    //         PresetItem,
    //         r#"
    //         SELECT name, published_file_id, position, enabled
    //         FROM preset_items
    //         WHERE preset_id = ?
    //         ORDER BY position ASC
    //         "#,
    //         preset_id
    //     )
    //     .fetch_all(&self.pool)
    //     .await?;

    //     Ok(items)
    // }
}

struct SqlPreset {
    id: i64,
    name: String,
    selected: Option<bool>,
}
