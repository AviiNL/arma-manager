use std::path::PathBuf;

use api_schema::response::Preset;
use process::{Process, ProcessControls};

pub const ARMA_CLIENT_APP_ID: u64 = 107410;
pub const ARMA_SERVER_APP_ID: u64 = 233780;

const DEFAULT_CONFIG: &str = include_str!("../server.cfg");
const DEFAULT_PROFILE: &str = include_str!("../profile.cfg");

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

pub fn get_mod_str(preset: &Preset) -> Result<String, Box<dyn std::error::Error>> {
    let mut items = preset.items.clone();
    let mut dlcs = preset.dlcs.clone();

    items.sort_by(|a, b| a.position.cmp(&b.position));
    dlcs.sort_by(|a, b| a.position.cmp(&b.position));

    let mods = items
        .iter()
        .filter(|item| item.enabled && !item.blacklisted && !item.server_mod)
        .collect::<Vec<_>>();

    let server_mods = items
        .iter()
        .filter(|item| item.enabled && !item.blacklisted && item.server_mod)
        .collect::<Vec<_>>();

    let dlcs = dlcs.iter().filter(|dlc| dlc.enabled).collect::<Vec<_>>();

    let mut missing = Vec::new();

    for item in mods.iter().clone().chain(&server_mods) {
        if !mod_exists(item.published_file_id) {
            missing.push(item.name.clone());
        }
    }

    if !missing.is_empty() {
        return Err(format!("Missing mods: {}", missing.join(", ")).into());
    }

    let mods = mods
        .iter()
        .map(|item| get_mod_path(item.published_file_id).to_string_lossy().to_string())
        .collect::<Vec<_>>();

    let server_mods = server_mods
        .iter()
        .map(|item| get_mod_path(item.published_file_id).to_string_lossy().to_string())
        .collect::<Vec<_>>();

    // PREPEND the dlcs to items
    let mods = dlcs.iter().map(|dlc| dlc.key.clone()).chain(mods).collect::<Vec<_>>();

    let mut mods_str = String::new();

    // if there are mods, add the -mod flag
    if !mods.is_empty() {
        mods_str.push_str(&format!(r#" "-mod={}""#, mods.join(";")));
    }

    // if there are servermods, append the -serverMod flag
    if !server_mods.is_empty() {
        mods_str.push_str(&format!(r#" "-serverMod={}""#, server_mods.join(";")));
    }

    Ok(mods_str)
}

pub fn install_keys(preset: &Preset) -> Result<(), std::io::Error> {
    let Some(arma_path) = paths::get_arma_path() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Arma 3 Server is not installed",
        ));
    };

    let items = preset.items.iter().filter(|item| item.enabled).collect::<Vec<_>>();

    let arma_keys_path = arma_path.join("keys");

    if !arma_keys_path.exists() {
        std::fs::create_dir_all(&arma_keys_path)?;
    }

    // delete all files except "a3.bikey" in the keys dir
    for entry in std::fs::read_dir(&arma_keys_path)? {
        let entry = entry?;

        let file_name = entry.file_name();

        if file_name != "a3.bikey" {
            std::fs::remove_file(entry.path())?;
        }
    }

    for item in &items {
        let mod_path = get_mod_path(item.published_file_id);

        let keys_path = mod_path.join("keys");

        if keys_path.exists() {
            for entry in std::fs::read_dir(&keys_path)? {
                let entry = entry?;

                let file_name = entry.file_name();

                let arma_key_path = arma_keys_path.join(file_name);

                if !arma_key_path.exists() {
                    std::fs::copy(entry.path(), arma_key_path)?;
                }
            }
        }
    }

    Ok(())
}

pub fn install_dlc_keys(preset: &Preset) -> Result<(), std::io::Error> {
    let Some(arma_path) = paths::get_arma_path() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Arma 3 Server is not installed",
        ));
    };

    let arma_keys_path = arma_path.join("keys");

    if !arma_keys_path.exists() {
        std::fs::create_dir_all(&arma_keys_path)?;
    }

    let items = preset.dlcs.iter().filter(|item| item.enabled).collect::<Vec<_>>();

    for item in &items {
        // key path is arma_path/{item.key}/keys, copy to arma_path/keys
        let key_path = arma_path.join(item.key.clone()).join("keys");

        if key_path.exists() {
            for entry in std::fs::read_dir(&key_path)? {
                let entry = entry?;

                let file_name = entry.file_name();

                let arma_key_path = arma_keys_path.join(file_name);

                if !arma_key_path.exists() {
                    std::fs::copy(entry.path(), arma_key_path)?;
                }
            }
        }
    }

    Ok(())
}

pub fn get_default_parameters() -> Vec<String> {
    let mut params = Vec::new();

    let cpus = num_cpus::get();

    // all of this needs to be moved to a config page
    params.push("-noSplash".to_string());
    params.push("-world=empty".to_string());
    params.push("-noFilePatching".to_string());
    params.push("-noPause".to_string());
    params.push("-skipIntro".to_string());
    params.push("-enableHT".to_string());
    params.push("-hugePages".to_string());
    params.push("-limitFPS=80".to_string());
    params.push(format!("-cpuCount={}", cpus));

    params
}

pub fn prepare_config() -> Result<(), std::io::Error> {
    let config_path = paths::get_config_path();
    let config_file = config_path.join("server.cfg");

    if !config_file.exists() {
        std::fs::write(config_file, DEFAULT_CONFIG)?;
    }

    Ok(())
}

pub fn prepare_profile() -> Result<(), std::io::Error> {
    let profile_path = paths::get_config_path();
    let profile_file = profile_path.join("profile.cfg");

    if !profile_file.exists() {
        std::fs::write(profile_file, DEFAULT_PROFILE)?;
    }

    Ok(())
}

pub struct Arma3 {
    mods: Option<String>,
    parameters: Option<Vec<String>>,
    name: String,
}

impl Arma3 {
    pub fn new() -> Self {
        Self {
            mods: None,
            parameters: None,
            name: "server".to_string(),
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

        let config_path = paths::get_config_path();

        let config_file = config_path.join("server.cfg");
        let config_lock = config_path.join("server.cfg.lock");

        let profile_file = config_path.join("profile.cfg");
        let Some(mut profile_lock) = paths::get_profile_path(&self.name) else {
            unreachable!();
        };
        profile_lock = profile_lock.join(format!("{}.Arma3Profile", self.name)); // this is different

        if !config_file.exists() {
            std::fs::write(&config_file, DEFAULT_CONFIG)?;
        }

        if !profile_file.exists() {
            std::fs::write(&profile_file, DEFAULT_PROFILE)?;
        }
        // make a copy of the config file, overwrite if exists "server.cfg.lock"
        // then pass the path to the config file as a parameter
        std::fs::copy(&config_file, &config_lock)?;

        // make a copy of the profile file, overwrite if exists "format!("{}.Arma3Profile", self.name)"
        std::fs::copy(&profile_file, &profile_lock)?;

        let mut cmd = Process::new(arma_path.join("arma3server_x64.exe"));

        cmd.arg(format!("-name={}", self.name));

        if let Some(mods) = self.mods {
            cmd.arg(mods);
        }

        cmd.arg(format!(r#""-config={}""#, config_lock.to_string_lossy()));

        if let Some(parameters) = self.parameters {
            for parameter in parameters {
                cmd.arg(parameter);
            }
        }

        Ok(cmd.start()?)
    }
}
