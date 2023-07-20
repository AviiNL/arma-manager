use api_schema::{request::*, response::*};
use axum::{
    extract::Path,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    Extension, Json,
};
use tokio_stream::{
    wrappers::{errors::BroadcastStreamRecvError, BroadcastStream},
    Stream,
};

use crate::{
    response::{ApiResponse, ApiResult, ErrorResponse},
    ConfigService,
};

pub async fn get_config(Path(channel): Path<String>) -> ApiResult<impl IntoResponse> {
    let config_file = paths::get_config_path().join(channel);

    // check if file exists
    if !config_file.exists() {
        return Err(ErrorResponse::new(format!("Config file {} does not exist", config_file.display())).into());
    }

    let config = tokio::fs::read_to_string(config_file)
        .await
        .map_err(|e| ErrorResponse::new(format!("{}", e)))?;

    Ok(ApiResponse::new(ConfigResponse {
        config: config.lines().map(|l| l.to_string()).collect(),
    }))
}

pub async fn post_config(
    Extension(arma_config): Extension<ConfigService>,
    Path(channel): Path<String>,
    Json(body): Json<UpdateConfigSchema>,
) -> ApiResult<impl IntoResponse> {
    let body = body.config;

    arma_config
        .update_config(channel, body)
        .await
        .map_err(|e| ErrorResponse::new(format!("{}", e)))?;

    Ok(ApiResponse::new(SimpleResponse {
        response: "Config updated".to_string(),
    }))
}

pub async fn sse_config(
    Extension(arma_config): Extension<ConfigService>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    let rx = arma_config.subscribe();

    Sse::new(BroadcastStream::new(rx)).keep_alive(KeepAlive::default())
}
