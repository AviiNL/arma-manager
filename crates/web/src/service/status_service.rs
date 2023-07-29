use std::{convert::Infallible, sync::Arc};

use axum::response::sse::Event;
use tokio::sync::RwLock;

pub use api_schema::response::State;
pub use api_schema::response::Status;

pub struct StatusService {
    tx: tokio::sync::watch::Sender<Result<Event, Infallible>>,
    last_status: RwLock<Status>,
}

#[allow(unused)]
impl StatusService {
    pub fn new() -> Arc<Self> {
        let tx = tokio::sync::watch::channel(Ok(Event::default())).0;

        // use sysinfo to check if arma3server_x64.exe is running
        // if it is, set arma to State::Running

        // use sysinfo to check if steamcmd.exe is running
        // if it is, set steam to State::Running

        let mut status = Status::default();

        if arma::is_runnung() {
            status.arma = State::Running;
        }

        if steam::is_runnung() {
            status.steamcmd = State::Running;
        }

        Arc::new(Self {
            tx,
            last_status: RwLock::new(status),
        })
    }

    pub async fn get_last(&self) -> Status {
        let last_status = self.last_status.read().await;
        last_status.clone()
    }

    pub async fn steam(&self) -> State {
        let last_status = self.last_status.read().await;
        last_status.steamcmd.clone()
    }

    pub async fn arma(&self) -> State {
        let last_status = self.last_status.read().await;
        last_status.arma.clone()
    }

    pub fn subscribe(&self) -> tokio::sync::watch::Receiver<Result<Event, Infallible>> {
        self.tx.subscribe()
    }

    pub async fn set_steam(&self, state: State) {
        let mut last_status = self.last_status.write().await;
        last_status.steamcmd = state;

        let event: Result<Event, Infallible> = Ok(Event::default().json_data(&*last_status).unwrap());
        let _ = self.tx.send(event);
    }

    pub async fn set_arma(&self, state: State) {
        let mut last_status = self.last_status.write().await;
        last_status.arma = state;

        let event: Result<Event, Infallible> = Ok(Event::default().json_data(&*last_status).unwrap());
        let _ = self.tx.send(event);
    }
}
