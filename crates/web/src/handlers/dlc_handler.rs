use std::sync::Arc;

use axum::{response::IntoResponse, Extension};

use crate::response::{ApiResponse, ApiResult, ErrorResponse};

pub async fn get_dlc(Extension(dlc_service): Extension<Arc<DlcService>>) -> ApiResult<impl IntoResponse> {
    let dlc = dlc_service
        .get_all()
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    Ok(ApiResponse::new(dlc))
}
