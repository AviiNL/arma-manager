use std::sync::{Arc, RwLock};

use a2s::{info::Info, players::Player};
use axum::response::sse::Event;
use serde::Serialize;
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize)]
pub enum PlayerOrInfo {
    Players(Vec<Player>),
    Info(Info),
}

#[derive(Clone)]
pub struct A2sService {
    info: Arc<RwLock<Option<Info>>>,
    players: Arc<RwLock<Vec<Player>>>,
    tx: broadcast::Sender<Event>,
}

impl A2sService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            info: Arc::new(RwLock::new(None)),
            players: Arc::new(RwLock::new(vec![])),
            tx: broadcast::channel(100).0,
        })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }

    pub fn get_latest_info(&self) -> Option<Info> {
        let info = self.info.read().unwrap();
        info.clone()
    }

    pub fn get_latest_players(&self) -> Vec<Player> {
        let players = self.players.read().unwrap();
        players.clone()
    }

    pub fn start(&self) {
        // spawn a thread to pull changes from a2s every 5 to 10 seconds
        let tx = self.tx.clone();

        // get the ip address of the first network interface that is not localhost

        let last_info = self.info.clone();
        let last_players = self.players.clone();
        tokio::spawn(async move {
            let ip = get_ip_address().expect("ip address");
            let ip_port = format!("{}:2303", ip);
            loop {
                let client = a2s::A2SClient::new().await.expect("socket stuff");
                if let Ok(info) = client.info(&ip_port).await {
                    let mut last_info = last_info.write().unwrap();
                    *last_info = Some(info.clone());
                    drop(last_info);
                    let info = PlayerOrInfo::Info(info);
                    let info_json = serde_json::to_string(&info).expect("serde to work");
                    let info_event = Event::default().event("info").data(info_json);
                    let _ = tx.send(info_event);
                };
                drop(client);

                tokio::time::sleep(std::time::Duration::from_secs(5)).await;

                let client = a2s::A2SClient::new().await.expect("socket stuff");
                match client.players(&ip_port).await {
                    Ok(players) => {
                        let mut last_players = last_players.write().unwrap();
                        *last_players = players.clone();
                        drop(last_players);
                        let players = PlayerOrInfo::Players(players);
                        let players_json = serde_json::to_string(&players).expect("serde to work");
                        let players_event = Event::default().event("players").data(players_json);
                        let _ = tx.send(players_event);
                    }
                    Err(e) => {
                        tracing::error!("{:?}", e);
                    }
                }
                drop(client);

                // sleep
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        });
    }
}

/// getting the players gives me a InvalidResponse when using 127.0.0.1
/// So an automated way to get the first network adapter that is not localhost
/// is needed, weird hack, i know, but it allows me to continue developing.
fn get_ip_address() -> Option<String> {
    use local_ip_address::list_afinet_netifas;
    let network_interfaces = list_afinet_netifas();
    if let Ok(network_interfaces) = network_interfaces {
        for (_, ip) in network_interfaces.iter() {
            if ip.is_ipv4() && !ip.is_loopback() {
                return Some(ip.to_string());
            }
        }
    }
    None
}
