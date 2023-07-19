use directories::BaseDirs;
use std::path::PathBuf;

pub fn get_base_path() -> PathBuf {
    let mut path: PathBuf = BaseDirs::new().unwrap().config_dir().into();
    path.push("Arma Server Manager");

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    path
}

pub fn get_steam_path() -> PathBuf {
    let path = get_base_path().join("steamcmd");

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    path
}

pub fn get_config_path() -> PathBuf {
    let path = get_base_path().join("config");

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    path
}

pub fn get_arma_path() -> Option<PathBuf> {
    let path = get_steam_path().join("steamapps").join("common").join("Arma 3 Server");

    if !path.exists() {
        return None;
    }

    Some(path)
}

pub fn get_log_path() -> PathBuf {
    let path = get_base_path().join("logs");

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    path
}

pub fn get_arma_log_path() -> PathBuf {
    let path = directories::BaseDirs::new().unwrap();
    path.cache_dir().join("Arma 3")
}
