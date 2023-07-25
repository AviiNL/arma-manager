use std::sync::Arc;

use api_schema::response::SimpleResponse;
use arma::*;
use axum::{response::IntoResponse, Extension};
use steam::AppUpdate;

use crate::{
    repository::PresetRepository,
    response::{ApiResponse, ApiResult, ErrorResponse},
    service::{State, StatusService},
};

pub async fn update_arma(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    if status.steam().await != State::Stopped {
        return Err(ErrorResponse::new("Steam is already running").into());
    }

    status.set_steam(State::Starting).await;

    let steam = steam::Steam::new_from_env().app_update(
        AppUpdate::new(ARMA_SERVER_APP_ID)
            .validate(true)
            .beta("creatordlc", None),
    );

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

pub async fn download_missing_mods(
    Extension(status): Extension<Arc<StatusService>>,
    Extension(repository): Extension<PresetRepository>,
) -> ApiResult<impl IntoResponse> {
    if status.steam().await != State::Stopped {
        return Err(ErrorResponse::new("Steam is already running").into());
    }

    status.set_steam(State::Starting).await;

    let mut steam = steam::Steam::new_from_env();

    let preset = repository
        .get_selected_preset()
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    let Some(preset) = preset else {
        return Err(ErrorResponse::new("No preset selected").into());
    };

    let mods = preset
        .items
        .iter()
        .filter(|m| m.enabled && !m.exists)
        .map(|m| m.published_file_id)
        .collect::<Vec<_>>();

    for published_file_id in mods {
        steam = steam.workshop_download_item(ARMA_CLIENT_APP_ID, published_file_id);
    }

    let c = steam
        .run()
        .map_err(|e| ErrorResponse::new(format!("Failed to update preset: {:?}", e)))?;

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

pub async fn force_check(
    Extension(status): Extension<Arc<StatusService>>,
    Extension(repository): Extension<PresetRepository>,
) -> ApiResult<impl IntoResponse> {
    if status.steam().await != State::Stopped {
        return Err(ErrorResponse::new("Steam is already running").into());
    }

    status.set_steam(State::Starting).await;

    let mut steam = steam::Steam::new_from_env();

    let preset = repository
        .get_selected_preset()
        .await
        .map_err(|e| ErrorResponse::new(format!("Database Error: {}", e)))?;

    let Some(preset) = preset else {
        return Err(ErrorResponse::new("No preset selected").into());
    };

    for published_file_id in preset.items.iter().filter(|m| m.enabled).map(|m| m.published_file_id) {
        steam = steam.workshop_download_item(ARMA_CLIENT_APP_ID, published_file_id);
    }

    let c = steam
        .run()
        .map_err(|e| ErrorResponse::new(format!("Failed to update preset: {:?}", e)))?;

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
