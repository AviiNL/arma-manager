use api_schema::response::LogResponse;
use axum::{
    extract::Path,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    Extension,
};
use futures::Stream;
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

use crate::{
    log_service::LogService,
    response::{ApiResponse, ApiResult},
};

pub async fn api_logs(
    Extension(log): Extension<LogService>,
    Path(channel): Path<String>,
) -> ApiResult<impl IntoResponse> {
    let log = log.get_latest(channel);

    Ok(ApiResponse::new(LogResponse { log }).with_root_key_name("log"))
}

pub async fn sse_logs(
    Extension(log): Extension<LogService>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    let rx = log.subscribe();

    let mystream = BroadcastStream::new(rx);

    Sse::new(mystream).keep_alive(KeepAlive::default())
}
