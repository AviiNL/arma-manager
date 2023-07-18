use std::convert::Infallible;
use std::sync::Arc;

use api_schema::request::*;
use api_schema::response::SimpleResponse;
use axum::response::sse::{Event, KeepAlive};
use axum::response::Sse;
use axum::Json;
use axum::{response::IntoResponse, Extension};
use futures::Stream;
use tokio_stream::wrappers::WatchStream;

use crate::response::{ApiResponse, ApiResult, ErrorResponse};
use crate::service::PresetService;

pub async fn get_presets(Extension(preset_service): Extension<Arc<PresetService>>) -> ApiResult<impl IntoResponse> {
    let presets = preset_service
        .get_all()
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(presets))
}

pub async fn create_preset(
    Extension(preset_service): Extension<Arc<PresetService>>,
    Json(input): Json<CreatePresetSchema>,
) -> ApiResult<impl IntoResponse> {
    let preset = preset_service
        .create(input)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(preset))
}

pub async fn select_preset(
    Extension(preset_service): Extension<Arc<PresetService>>,
    Json(input): Json<SelectPresetSchema>,
) -> ApiResult<impl IntoResponse> {
    preset_service
        .select(input)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn update_preset_item(
    Extension(preset_service): Extension<Arc<PresetService>>,
    Json(input): Json<UpdatePresetItemSchema>,
) -> ApiResult<impl IntoResponse> {
    let item = preset_service
        .update_item(input)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(item))
}

pub async fn blacklist_item(
    Extension(preset_service): Extension<Arc<PresetService>>,
    Json(input): Json<BlacklistItemSchema>,
) -> ApiResult<impl IntoResponse> {
    preset_service
        .blacklist_item(input)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn unblacklist_item(
    Extension(preset_service): Extension<Arc<PresetService>>,
    Json(input): Json<BlacklistItemSchema>,
) -> ApiResult<impl IntoResponse> {
    preset_service
        .unblacklist_item(input)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn delete_preset(
    Extension(preset_service): Extension<Arc<PresetService>>,
    Json(input): Json<DeletePresetSchema>,
) -> ApiResult<impl IntoResponse> {
    preset_service
        .delete_preset(input)
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn sse_preset_handler(
    Extension(presets): Extension<Arc<PresetService>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = WatchStream::new(presets.subscribe());

    Sse::new(stream).keep_alive(KeepAlive::default())
}
