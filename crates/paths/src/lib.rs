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

pub fn get_log_path() -> PathBuf {
    let path = get_base_path().join("logs");

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    path
}
