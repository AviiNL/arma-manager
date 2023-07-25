use std::{convert::Infallible, sync::Arc};

use api_schema::{request::*, response::*};
use axum::response::sse::Event;
use tokio::sync::watch::{channel, Receiver, Sender};

use crate::repository::DlcRepository;

pub struct DlcService {
    tx: Sender<Result<Event, Infallible>>,
    repository: DlcRepository,
}
