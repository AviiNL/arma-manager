use api_schema::request::*;
use axum::Json;
use axum::{response::IntoResponse, Extension};

use crate::repository::PresetRepository;
use crate::response::{ApiResponse, ApiResult, ErrorResponse};

pub async fn create_preset(
    Extension(preset_repository): Extension<PresetRepository>,
    Json(input): Json<CreatePresetSchema>,
) -> ApiResult<impl IntoResponse> {
    let preset = preset_repository
        .create(&input)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(preset))
}
