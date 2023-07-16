use std::{convert::Infallible, sync::Arc};

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

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn api_status_handler(Extension(status): Extension<Arc<StatusService>>) -> ApiResult<impl IntoResponse> {
    Ok(ApiResponse::new(status.get_last().await).with_root_key_name("data"))
}
