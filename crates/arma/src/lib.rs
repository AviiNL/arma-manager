pub const ARMA_CLIENT_APP_ID: u64 = 107410;
pub const ARMA_SERVER_APP_ID: u64 = 233780;

pub fn mod_exists(published_file_id: i64) -> bool {
    paths::get_steam_path()
        .join("steamapps")
        .join("workshop")
        .join("content")
        .join(ARMA_CLIENT_APP_ID.to_string())
        .join(published_file_id.to_string())
        .exists()
}
