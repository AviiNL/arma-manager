use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use leptos::{html::Textarea, *};
use leptos_use::*;

use crate::{api::AuthorizedApi, app::DEFAULT_SSE_URL, components::Progress};

pub fn is_mounted(cx: Scope) -> impl Fn() -> bool {
    let (mounted, _) = create_signal(cx, ());
    move || -> bool { mounted.try_get_untracked().is_some() }
}

#[component]
pub fn LogView(
    cx: Scope,
    uri: &'static str,
    visible: RwSignal<bool>,
    progress: Option<RwSignal<Progress>>,
) -> impl IntoView {
    let log_content = create_rw_signal(cx, String::new());

    let element = create_node_ref::<Textarea>(cx);

    let UseScrollReturn { set_y, .. } = use_scroll_with_options(
        cx,
        element,
        UseScrollOptions::default().behavior(ScrollBehavior::Smooth),
    );

    // Scroll down on change
    create_effect(cx, move |_| {
        let _ = log_content.get();
        request_animation_frame(move || {
            // if log_content changes, scroll to bottom
            let UseScrollReturn { set_y, .. } = use_scroll_with_options(
                cx,
                element,
                UseScrollOptions::default().behavior(ScrollBehavior::Smooth),
            );
            set_y(element.get_untracked().unwrap().scroll_height() as f64);
        });
    });

    // clear log content when not visible
    create_effect(cx, move |_| {
        if !visible.get() {
            log_content.set(String::new());
            if let Some(progress) = &progress {
                progress.update(|p| p.update(0, 0));
            }
        }
    });

    create_effect(cx, move |_| {
        let api = use_context::<AuthorizedApi>(cx).expect("to have found the api provided");

        let uri = format!("{}/logs{}?token={}", DEFAULT_SSE_URL, uri, api.token().token);
        let mut event_source = EventSource::new(&uri).expect("EventSource::new");
        let mut stream = event_source.subscribe("message").unwrap();

        spawn_local(async move {
            let _ = event_source.state(); // this blocks until connected?
            while let Some(Ok((event_type, msg))) = stream.next().await {
                let new_data = msg.data().as_string().unwrap();
                // new data is a json array ["","",""] etc
                let new_data = serde_json::from_str::<Vec<String>>(&new_data).unwrap().join("\n");

                // if new_data contains progress: {float} ({usize} / {usize}})

                if let Some(progress) = &progress {
                    let mut line_to_parse = new_data.clone();
                    if line_to_parse.contains("\n") {
                        line_to_parse = line_to_parse.split("\n").last().unwrap().to_string();
                    }

                    if line_to_parse.contains("progress: ") {
                        let value = line_to_parse
                            .split("progress: ")
                            .nth(1)
                            .unwrap()
                            .split(" (")
                            .nth(1)
                            .unwrap()
                            .split(" / ")
                            .nth(0)
                            .unwrap()
                            .parse::<i64>()
                            .unwrap();

                        let max = line_to_parse
                            .split("progress: ")
                            .nth(1)
                            .unwrap()
                            .split(" / ")
                            .nth(1)
                            .unwrap()
                            .split(")")
                            .nth(0)
                            .unwrap()
                            .parse::<i64>()
                            .unwrap();

                        progress.update(|p| p.update(value, max));
                    }
                }

                let old_data = log_content.get_untracked();

                // append new_data to old_data and store in log_content
                log_content.set(format!("{}{}\n", old_data, new_data));
            }
        });
    });

    view! { cx,
        <textarea class="textarea log h-full w-full resize-none" readonly node_ref=element>{move || log_content.get()}</textarea>
    }
}
