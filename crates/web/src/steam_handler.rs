use std::{convert::Infallible, sync::Arc, time::Duration};

use api_schema::response::SimpleResponse;
use axum::{
    response::{sse::Event, IntoResponse, Sse},
    Extension,
};
use futures::{stream, Stream, StreamExt};
use steam::AppUpdate;

use crate::{
    response::{ApiResponse, ApiResult, ErrorResponse},
    status::StatusService,
};

pub async fn update_arma(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    status.set_steam(crate::status::State::Starting).await;

    let steam = steam::Steam::new_from_env().app_update(AppUpdate::new(233780).validate(true));

    let c = steam
        .run()
        .map_err(|e| ErrorResponse::new(format!("Failed to update Arma 3: {:?}", e)))?;

    tokio::spawn(async move {
        status.set_steam(crate::status::State::Running).await;

        loop {
            if status.steam().await == crate::status::State::Stopping {
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

        status.set_steam(crate::status::State::Stopped).await;
    });

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn cancel_update_arma(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    if status.steam().await != crate::status::State::Running {
        return Err(ErrorResponse::new("Steam is not running").into());
    }

    status.set_steam(crate::status::State::Stopping).await;

    Ok(ApiResponse::new(SimpleResponse {
        response: "OK".to_string(),
    }))
}

pub async fn sse_steamcmd_log() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut rx = steam::log_watcher::watch().unwrap();

    let stream = stream::poll_fn(move |cx| {
        let mut events = Vec::new();

        while let futures::task::Poll::Ready(Some(data)) = rx.poll_next_unpin(cx) {
            events.push(data);
        }

        if events.is_empty() {
            futures::task::Poll::Pending
        } else {
            let events = serde_json::to_string(&events).unwrap();

            futures::task::Poll::Ready(Some(Ok(Event::default().data(events))))
        }
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
