use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    response::{sse::Event, sse::KeepAlive, IntoResponse, Sse},
    Extension,
};
use futures::Stream;
use tokio_stream::wrappers::WatchStream;

use crate::{
    response::{ApiResponse, ApiResult},
    status_service::StatusService,
};

pub async fn sse_status_handler(
    Extension(status): Extension<Arc<StatusService>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = WatchStream::new(status.subscribe());

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(1)).text("keep-alive"))
}

pub async fn api_status_handler(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    Ok(ApiResponse::new(status.get_last().await).with_root_key_name("data"))
}
