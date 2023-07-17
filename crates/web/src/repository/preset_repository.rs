use api_schema::{
    request::*,
    response::{Preset, PresetItem},
};
use sqlx::{QueryBuilder, SqlitePool};

use super::RepositoryResult;

#[derive(Clone)]
pub struct PresetRepository {
    pool: SqlitePool,
}

impl PresetRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl PresetRepository {
    pub async fn get_all(&self) -> RepositoryResult<Vec<Preset>> {
        let presets = sqlx::query_as!(
            SqlPreset,
            r#"
            SELECT id, name, selected
            FROM presets
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = vec![];
        for preset in presets {
            let items = self.get_items(preset.id).await?;
            result.push(Preset {
                id: preset.id,
                name: preset.name,
                selected: preset.selected.is_some(),
                items,
            });
        }

        Ok(result)
    }

    async fn get_items(&self, preset_id: i64) -> RepositoryResult<Vec<PresetItem>> {
        let mut items: Vec<PresetItem> = sqlx::query_as(
            r#"
            SELECT id, name, published_file_id, position, enabled
            FROM preset_items
            WHERE preset_id = ?
            ORDER BY position ASC
            "#,
        )
        .bind(preset_id)
        .fetch_all(&self.pool)
        .await?;

        for item in &mut items {
            item.exists = arma::mod_exists(item.published_file_id);
        }

        Ok(items)
    }
}

impl PresetRepository {
    pub async fn create(&self, input: CreatePresetSchema) -> RepositoryResult<Preset> {
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
            let mut item: PresetItem = sqlx::query_as(
                r#"
                INSERT INTO preset_items (preset_id, name, published_file_id, position, enabled)
                VALUES (?, ?, ?, ?, ?)
                RETURNING id, name, published_file_id, position, enabled
                "#,
            )
            .bind(preset.id)
            .bind(item.name.clone())
            .bind(item.published_file_id)
            .bind(item.position)
            .bind(item.enabled)
            .fetch_one(&self.pool)
            .await?;

            item.exists = arma::mod_exists(item.published_file_id);

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

    pub async fn select(&self, id: i64) -> RepositoryResult<Preset> {
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

    pub async fn update_item(&self, schema: UpdatePresetItemSchema) -> RepositoryResult<PresetItem> {
        let mut query = QueryBuilder::new("UPDATE preset_items SET ");

        if let Some(enabled) = schema.enabled {
            query.push(" enabled = ").push_bind(enabled);
        }

        if let Some(position) = schema.position {
            query.push(" position = ").push_bind(position);
        }

        query.push(" WHERE id = ").push_bind(schema.id);

        // returning
        query.push(" RETURNING id, name, published_file_id, position, enabled");

        let mut preset_item = query.build_query_as::<PresetItem>().fetch_one(&self.pool).await?;
        preset_item.exists = arma::mod_exists(preset_item.published_file_id);

        Ok(preset_item)
    }
}

struct SqlPreset {
    id: i64,
    name: String,
    selected: Option<bool>,
}
