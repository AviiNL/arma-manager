use std::path::PathBuf;

use api_schema::response::Preset;

pub const ARMA_CLIENT_APP_ID: u64 = 107410;
pub const ARMA_SERVER_APP_ID: u64 = 233780;

pub fn mod_exists(published_file_id: i64) -> bool {
    get_mod_path(published_file_id).exists()
}

pub fn get_mod_path(published_file_id: i64) -> PathBuf {
    paths::get_steam_path()
        .join("steamapps")
        .join("workshop")
        .join("content")
        .join(ARMA_CLIENT_APP_ID.to_string())
        .join(published_file_id.to_string())
}

pub fn get_mod_str(preset: Preset) -> Result<String, Box<dyn std::error::Error>> {
    let mut items = preset.items;

    items.sort_by(|a, b| a.position.cmp(&b.position));

    let mut missing = Vec::new();

    for item in &items {
        if !mod_exists(item.published_file_id) {
            missing.push(item.name.clone());
        }
    }

    if !missing.is_empty() {
        return Err(format!("Missing mods: {}", missing.join(", ")).into());
    }

    let items = items
        .iter()
        .filter(|item| item.enabled && !item.blacklisted)
        .map(|item| get_mod_path(item.published_file_id).to_string_lossy().to_string())
        .collect::<Vec<_>>();

    Ok(format!(r#""-mods={}""#, items.join(";")))
}
