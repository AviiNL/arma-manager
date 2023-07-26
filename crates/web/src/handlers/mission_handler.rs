// a single function to upload a mission file

use api_schema::response::{MissionResponse, SimpleResponse};
use axum::response::IntoResponse;
use axum_extra::extract::Multipart;

use crate::response::{ApiResponse, ApiResult, ErrorResponse};

pub async fn get_missions() -> ApiResult<impl IntoResponse> {
    let Some(arma_path) = paths::get_arma_path() else {
        return Err(ErrorResponse::new("Arma not installed").into());
    };

    let missions_path = arma_path.join("mpmissions");

    // if not exists, create
    if !missions_path.exists() {
        tokio::fs::create_dir_all(&missions_path)
            .await
            .map_err(|e| ErrorResponse::new(format!("Failed to create dir: {}", e)))?;
    }

    let mut missions = Vec::new();

    let mut dir_contents = tokio::fs::read_dir(missions_path)
        .await
        .map_err(|e| ErrorResponse::new(format!("Failed to read dir: {}", e)))?;

    while let Some(entry) = dir_contents
        .next_entry()
        .await
        .map_err(|e| ErrorResponse::new(format!("Failed to read entry: {}", e)))?
    {
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();

            if filename.ends_with(".pbo") {
                let filename = filename.strip_suffix(".pbo").unwrap().to_string();
                missions.push(filename);
            }
        }
    }

    Ok(ApiResponse::new(MissionResponse { missions }))
}

pub async fn upload_mission(mut multipart: Multipart) -> ApiResult<impl IntoResponse> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let Some(arma_path) = paths::get_arma_path() else {
            return Err(ErrorResponse::new("Arma not installed").into());
        };

        let missions_path = arma_path.join("mpmissions");

        // if not exists, create
        if !missions_path.exists() {
            tokio::fs::create_dir_all(&missions_path)
                .await
                .map_err(|e| ErrorResponse::new(format!("Failed to create dir: {}", e)))?;
        }

        let file_path = missions_path.join(name);

        tokio::fs::write(file_path, data)
            .await
            .map_err(|e| ErrorResponse::new(format!("Failed to write file: {}", e)))?;
    }

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}
