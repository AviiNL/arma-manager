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
pub struct CreatePresetSchema {
    pub name: String,
    pub items: Vec<PresetItemSchema>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetActivePresetSchema {
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetItemSchema {
    pub name: String,
    pub published_file_id: i64,
    pub enabled: bool,
    pub position: i64,
}
