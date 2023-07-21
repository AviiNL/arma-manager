use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserSchema {
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RevokeTokenSchema {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePresetSchema {
    pub name: String,
    pub items: Vec<PresetItemSchema>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectPresetSchema {
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePresetItemSchema {
    pub id: i64,
    pub enabled: Option<bool>,
    pub position: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetItemSchema {
    pub name: String,
    pub published_file_id: i64,
    pub enabled: bool,
    pub position: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlacklistItemSchema {
    pub published_file_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeletePresetSchema {
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateConfigSchema {
    pub config: String,
}
