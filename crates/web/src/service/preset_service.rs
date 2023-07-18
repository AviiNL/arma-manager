use std::{convert::Infallible, sync::Arc};

use api_schema::{request::*, response::*};
use axum::response::sse::Event;
use tokio::sync::watch::{channel, Receiver, Sender};

use crate::repository::PresetRepository;

pub struct PresetService {
    tx: Sender<Result<Event, Infallible>>,
    repository: PresetRepository,
}

impl PresetService {
    pub fn new(repository: PresetRepository) -> Arc<Self> {
        let tx = channel(Ok(Event::default())).0;
        Arc::new(Self { tx, repository })
    }

    pub fn subscribe(&self) -> Receiver<Result<Event, Infallible>> {
        self.tx.subscribe()
    }

    pub async fn get_all(&self) -> Result<Vec<Preset>, Box<dyn std::error::Error>> {
        self.repository.get_all().await
    }

    pub async fn create(&self, schema: CreatePresetSchema) -> Result<Preset, Box<dyn std::error::Error>> {
        let preset = self.repository.create(schema).await?;

        let _ = self.tx.send(Ok(Event::default()
            .event("create")
            .data(serde_json::to_string(&PresetUpdate::Created(preset.clone()))?)));

        // set active
        self.select(SelectPresetSchema { id: preset.id }).await?;

        Ok(preset)
    }

    pub async fn select(&self, schema: SelectPresetSchema) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.select(schema.id).await?;

        let _ = self.tx.send(Ok(Event::default()
            .event("select")
            .data(serde_json::to_string(&PresetUpdate::Selected(schema.id))?)));

        Ok(())
    }

    pub async fn update_item(&self, schema: UpdatePresetItemSchema) -> Result<PresetItem, Box<dyn std::error::Error>> {
        let item = self.repository.update_item(schema).await?;

        let _ = self.tx.send(Ok(Event::default()
            .event("update")
            .data(serde_json::to_string(&PresetUpdate::Updated(item.clone()))?)));

        Ok(item)
    }

    // blacklist_item
    pub async fn blacklist_item(&self, schema: BlacklistItemSchema) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.blacklist_item(schema.published_file_id).await?;

        let _ = self
            .tx
            .send(Ok(Event::default().event("blacklist").data(serde_json::to_string(
                &PresetUpdate::Blacklisted(schema.published_file_id),
            )?)));

        Ok(())
    }

    // unblacklist_item
    pub async fn unblacklist_item(&self, schema: BlacklistItemSchema) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.unblacklist_item(schema.published_file_id).await?;

        let _ = self
            .tx
            .send(Ok(Event::default().event("unblacklist").data(serde_json::to_string(
                &PresetUpdate::Unblacklisted(schema.published_file_id),
            )?)));

        Ok(())
    }
}
