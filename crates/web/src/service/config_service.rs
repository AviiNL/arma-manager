use axum::response::sse::Event;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct ConfigService {
    tx: broadcast::Sender<Event>,
}

impl ConfigService {
    pub fn new() -> Self {
        Self {
            tx: broadcast::channel(100).0,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }

    pub async fn update_config(&self, channel: String, body: String) -> Result<(), std::io::Error> {
        let config_file = paths::get_config_path().join(channel.clone());

        if !config_file.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Config file {} does not exist", config_file.display()),
            ));
        }

        tokio::fs::write(config_file, body.clone()).await?;

        let mut data = Vec::new();
        for line in body.lines() {
            data.push(line);
        }

        let body = serde_json::to_string(&data)?;

        let _ = self.tx.send(Event::default().event(channel).data(body));

        Ok(())
    }
}
