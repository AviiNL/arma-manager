use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimpleResponse {
    pub response: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiToken {
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub roles: Vec<String>,
    pub verified: bool,
    pub tokens: Vec<FilteredUserToken>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilteredUserToken {
    pub token: String,
    pub ip: String,
    pub created_at: i64,
    pub last_used: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum State {
    Starting,
    Running,
    Stopping,
    #[default]
    Stopped,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Status {
    pub steamcmd: State,
    pub arma: State,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogResponse {
    pub log: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preset {
    pub id: i64,
    pub name: String,
    pub selected: bool,
    pub items: Vec<PresetItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PresetItem {
    pub id: i64,
    pub name: String,
    pub published_file_id: i64,
    pub position: i64,
    pub enabled: bool,
    pub blacklisted: bool,
    #[cfg_attr(feature = "ssr", sqlx(skip))]
    pub exists: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PresetUpdate {
    Created(Preset),
    Updated(PresetItem),
    Removed(i64),
    Selected(i64),
    Blacklisted(i64),
    Unblacklisted(i64),
    Delete(i64),
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArmaConfig {
    pub config: String,
    // pub profile: String, // for difficulty settings etc
}
