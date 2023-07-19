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

    pub async fn get_selected_preset(&self) -> RepositoryResult<Option<Preset>> {
        let preset = sqlx::query_as!(
            SqlPreset,
            r#"
            SELECT id, name, selected
            FROM presets
            WHERE selected = ?
            "#,
            true
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(preset) = preset {
            let items = self.get_items(preset.id).await?;
            Ok(Some(Preset {
                id: preset.id,
                name: preset.name,
                selected: true,
                items,
            }))
        } else {
            Ok(None)
        }
    }

    /*
    SELECT id, name, published_file_id, position, enabled
    FROM preset_items
    FULL OUTER JOIN blacklist ON blacklist.blacklisted = preset_items.published_file_id
    WHERE preset_id = ?
    ORDER BY position ASC
        */

    pub async fn get_items(&self, preset_id: i64) -> RepositoryResult<Vec<PresetItem>> {
        let mut items: Vec<PresetItem> = sqlx::query_as(
            r#"
            SELECT i.id, i.name, i.published_file_id, i.position, i.enabled,
                CASE WHEN b.published_file_id IS NULL THEN 0 ELSE 1 END AS blacklisted
            FROM preset_items i
            LEFT JOIN blacklist b ON i.published_file_id = b.published_file_id
            WHERE i.preset_id = ?
            ORDER BY i.position ASC
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
                RETURNING id, name, published_file_id, position, enabled,
                    (SELECT EXISTS(SELECT 1 FROM blacklist b
                    WHERE b.published_file_id = preset_items.published_file_id)) AS blacklisted
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
        let mut query = QueryBuilder::new(
            r#"
            UPDATE preset_items
            SET "#,
        );

        if let Some(enabled) = schema.enabled {
            query.push(" enabled = ").push_bind(enabled);
        }

        if let Some(position) = schema.position {
            query.push(" position = ").push_bind(position);
        }

        query.push(" WHERE id = ").push_bind(schema.id);

        // returning
        query.push(
            "RETURNING id, name, published_file_id, position, enabled,
            (SELECT EXISTS(SELECT 1 FROM blacklist b
             WHERE b.published_file_id = preset_items.published_file_id)) AS blacklisted",
        );

        let mut preset_item = query.build_query_as::<PresetItem>().fetch_one(&self.pool).await?;
        preset_item.exists = arma::mod_exists(preset_item.published_file_id);

        Ok(preset_item)
    }

    pub async fn delete_preset(&self, id: i64) -> RepositoryResult<()> {
        // abort if the preset is the currently selected one
        let selected = sqlx::query!(
            r#"
            SELECT selected FROM presets WHERE id = ?
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        if selected.selected.is_some() {
            return Err(anyhow::anyhow!("Cannot delete the currently selected preset").into());
        }

        sqlx::query("DELETE FROM preset_items WHERE preset_id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        sqlx::query("DELETE FROM presets WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

impl PresetRepository {
    pub async fn is_item_used(&self, published_file_id: i64) -> RepositoryResult<bool> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM preset_items WHERE published_file_id = ?) AS "exists!: bool"
            "#,
            published_file_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists)
    }
}

impl PresetRepository {
    // blacklist
    pub async fn blacklist_item(&self, published_file_id: i64) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO blacklist (published_file_id)
            VALUES (?)
            "#,
            published_file_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // unblacklist
    pub async fn unblacklist_item(&self, published_file_id: i64) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM blacklist WHERE published_file_id = ?
            "#,
            published_file_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

struct SqlPreset {
    id: i64,
    name: String,
    selected: Option<bool>,
}
