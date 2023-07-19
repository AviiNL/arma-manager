use std::{sync::Arc, time::Duration};

use api_schema::response::SimpleResponse;
use axum::{response::IntoResponse, Extension};

use crate::{
    repository::PresetRepository,
    response::{ApiResponse, ApiResult, ErrorResponse},
    service::{State, StatusService},
};

pub async fn start_arma(
    Extension(status): Extension<Arc<StatusService>>,
    Extension(preset_repository): Extension<PresetRepository>,
) -> ApiResult<impl IntoResponse> {
    if status.arma().await != State::Stopped {
        return Err(ErrorResponse::new("Arma is already running").into());
    }

    status.set_arma(State::Starting).await;

    let Ok(Some(preset)) = preset_repository.get_selected_preset().await else {
        status.set_arma(State::Stopped).await;
        return Err(ErrorResponse::new("No preset selected").into());
    };

    let mod_str = match arma::get_mod_str(&preset).map_err(|e| ErrorResponse::new(format!("{}", e))) {
        Ok(mod_str) => mod_str,
        Err(e) => {
            tokio::spawn(async move {
                status.set_arma(State::Stopped).await;
            });
            return Err(e.into());
        }
    };
    let params = arma::get_default_parameters();

    arma::install_keys(&preset).map_err(|e| ErrorResponse::new(format!("{}", e)))?;

    let c = match arma::Arma3::new()
        .mods(mod_str)
        .parameters(params)
        .run()
        .map_err(|e| ErrorResponse::new(format!("{}", e)))
    {
        Ok(c) => c,
        Err(e) => {
            tokio::spawn(async move {
                status.set_arma(State::Stopped).await;
            });
            return Err(e.into());
        }
    };

    let a_status = status.clone();
    tokio::spawn(async move {
        a_status.set_arma(State::Running).await;

        loop {
            if a_status.arma().await == State::Stopping {
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

        a_status.set_arma(State::Stopped).await;
    });

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn stop_arma(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    if status.arma().await == State::Stopped {
        return Err(ErrorResponse::new("Arma is not running").into());
    }

    status.set_arma(State::Stopping).await;

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
