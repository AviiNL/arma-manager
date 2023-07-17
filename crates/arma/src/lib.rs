pub fn mod_exists(published_file_id: i64) -> bool {
    paths::get_steam_path()
        .join("steamapps")
        .join("workshop")
        .join("content")
        .join("107410")
        .join(published_file_id.to_string())
        .exists()
}
