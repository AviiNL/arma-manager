use std::sync::Arc;

use axum::{
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    Extension,
};
use futures::Stream;
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

use crate::{
    response::{ApiResponse, ApiResult},
    A2sService,
};

pub async fn api_a2s_info(Extension(a2s): Extension<Arc<A2sService>>) -> ApiResult<impl IntoResponse> {
    let info = a2s.get_latest_info(); // info or players

    Ok(ApiResponse::new(info.unwrap()).with_root_key_name("info"))
}

pub async fn api_a2s_players(Extension(a2s): Extension<Arc<A2sService>>) -> ApiResult<impl IntoResponse> {
    let players = a2s.get_latest_players(); // info or players

    Ok(ApiResponse::new(players).with_root_key_name("players"))
}

pub async fn sse_a2s(
    Extension(a2s): Extension<Arc<A2sService>>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    let rx = a2s.subscribe();

    let mystream = BroadcastStream::new(rx);

    Sse::new(mystream).keep_alive(KeepAlive::default())
}
