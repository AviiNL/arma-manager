use std::path::PathBuf;

use api_schema::response::Preset;
use process::{Process, ProcessControls};

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

    let items = items
        .iter()
        .filter(|item| item.enabled && !item.blacklisted)
        .collect::<Vec<_>>();

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
        .map(|item| get_mod_path(item.published_file_id).to_string_lossy().to_string())
        .collect::<Vec<_>>();

    Ok(format!(r#""-mods={}""#, items.join(";")))
}

pub fn get_default_parameters() -> Vec<String> {
    let mut params = Vec::new();

    // all of this needs to be moved to a config page
    params.push("-noSplash".to_string());
    params.push("-world=empty".to_string());
    params.push("-noFilePatching".to_string());
    params.push("-noPause".to_string());
    params.push("-skipIntro".to_string());
    params.push("-enableHT".to_string());
    params.push("-hugePages".to_string());
    params.push("-limitFPS=80".to_string());

    params
}

pub struct Arma3 {
    mods: Option<String>,
    parameters: Option<Vec<String>>,
    // config: Option<ArmaConfig>,
}

impl Arma3 {
    pub fn new() -> Self {
        Self {
            mods: None,
            parameters: None,
        }
    }

    pub fn mods(mut self, mods: String) -> Self {
        self.mods = Some(mods);
        self
    }

    pub fn parameters(mut self, parameters: Vec<String>) -> Self {
        self.parameters = Some(parameters);
        self
    }

    pub fn run(self) -> Result<ProcessControls, Box<dyn std::error::Error>> {
        let Some(arma_path) = paths::get_arma_path() else {
            return Err("Arma 3 Server is not installed".into());
        };

        let mut cmd = Process::new(arma_path.join("arma3server_x64.exe"));

        if let Some(mods) = self.mods {
            cmd.arg(mods);
        }

        if let Some(parameters) = self.parameters {
            for parameter in parameters {
                cmd.arg(parameter);
            }
        }

        Ok(cmd.start()?)
    }
}
