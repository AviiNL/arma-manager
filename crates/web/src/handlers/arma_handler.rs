use std::{sync::Arc, time::Duration};

use api_schema::response::SimpleResponse;
use axum::{response::IntoResponse, Extension};

use crate::{
    response::{ApiResponse, ApiResult, ErrorResponse},
    service::{State, StatusService},
};

pub async fn start_arma(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    if status.arma().await != State::Stopped {
        return Err(ErrorResponse::new("Arma is already running").into());
    }

    tokio::spawn(async move {
        status.set_arma(State::Starting).await;
        // sleep for 5 seconds
        tokio::time::sleep(Duration::from_secs(5)).await;
        // set state to running
        status.set_arma(State::Running).await;
    });

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn stop_arma(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    if status.arma().await == State::Stopped {
        return Err(ErrorResponse::new("Arma is not running").into());
    }

    tokio::spawn(async move {
        status.set_arma(State::Stopping).await;
        // sleep for 3 seconds
        tokio::time::sleep(Duration::from_secs(3)).await;
        // set state to stopped
        status.set_arma(State::Stopped).await;
    });

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn restart_arma(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    tokio::spawn(async move {
        // change state to stopping
        status.set_arma(State::Stopping).await;
        // sleep for 3 seconds
        tokio::time::sleep(Duration::from_secs(3)).await;
        // change state to starting
        status.set_arma(State::Starting).await;
        // sleep for 5 seconds
        tokio::time::sleep(Duration::from_secs(5)).await;
        // set state to running
        status.set_arma(State::Running).await;
    });

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}
