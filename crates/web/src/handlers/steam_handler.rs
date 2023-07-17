use std::sync::Arc;

use api_schema::response::SimpleResponse;
use axum::{response::IntoResponse, Extension};
use steam::AppUpdate;

use crate::{
    response::{ApiResponse, ApiResult, ErrorResponse},
    service::{State, StatusService},
};

pub async fn update_arma(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    if status.steam().await != State::Stopped {
        return Err(ErrorResponse::new("Steam is already running").into());
    }

    status.set_steam(State::Starting).await;

    let steam = steam::Steam::new_from_env().app_update(AppUpdate::new(233780).validate(true));

    let c = steam
        .run()
        .map_err(|e| ErrorResponse::new(format!("Failed to update Arma 3: {:?}", e)))?;

    tokio::spawn(async move {
        status.set_steam(State::Running).await;

        loop {
            if status.steam().await == State::Stopping {
                c.kill();
            }

            match c.next().await {
                Ok(Some(_)) => {}
                Ok(None) => {
                    break;
                }
                Err(_) => {}
            }
        }

        status.set_steam(State::Stopped).await;
    });

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn cancel_update_arma(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    if status.steam().await != State::Running {
        return Err(ErrorResponse::new("Steam is not running").into());
    }

    status.set_steam(State::Stopping).await;

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}
