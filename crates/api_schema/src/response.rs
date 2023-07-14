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
