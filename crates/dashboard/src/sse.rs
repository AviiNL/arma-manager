use futures::{channel::oneshot, select, stream, FutureExt, StreamExt};
use gloo_net::eventsource::futures::EventSource;
use leptos::*;
use merge_streams::MergeStreams;
use serde::de::DeserializeOwned;

pub const DEFAULT_SSE_URL: &str = "/sse/v1";

use crate::{api::AuthorizedApi, app_state::AppState};

pub fn create_sse<T, C>(cx: Scope, uri: impl Into<String>, channels: Vec<String>, cb: C) -> oneshot::Sender<()>
where
    T: DeserializeOwned + 'static,
    C: Fn(String, T) + 'static,
{
    let uri = uri.into();

    let (tx, mut rx) = oneshot::channel();

    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let api = app_state.api.get_untracked().expect("api to exist");

    let url = format!("{}/{}?token={}", DEFAULT_SSE_URL, uri, api.token().token);

    // BIG FUCKING TODO: Proper error handling!
    // make T Result<T, err> in cb?

    let mut event_source = EventSource::new(&url).expect("EventSource::new");
    spawn_local(async move {
        let _ = event_source.state(); //keep-alive?
        let mut channels = channels;

        let mut streams = Vec::new();

        while let Some(channel) = channels.pop() {
            let stream = event_source.subscribe(channel).expect("this not to fail");
            streams.push(stream);
        }

        let mut all_streams = streams.merge().fuse();

        loop {
            select! {
                _ = rx => {
                    break;
                }
                data = all_streams.next() => {
                    if let Some(Ok((event_type, msg))) = data {
                        let data = msg.data().as_string().unwrap();
                        cb(event_type, serde_json::from_str::<T>(&data).expect("this not to fail"));
                    }
                }
            }
        }

        tracing::info!("SSE stopped");

        // while let Some(Ok((event_type, msg))) = all_streams.next().await {
        //     if rx.try_recv().is_ok() {
        //         // i was hoping i could merge this with the streams
        //         // and expect a None when to stop or something
        //         tracing::info!("{}/{} Stopped", uri, event_type);
        //         break;
        //     } else {
        //         tracing::error!("Sender dropped!");
        //     }

        //     let data = msg.data().as_string().unwrap();
        //     cb(event_type, serde_json::from_str::<T>(&data).expect("this not to fail"));
        // }
    });

    tx
}
