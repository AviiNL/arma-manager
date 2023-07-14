use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use leptos::{html::Textarea, *};
use leptos_use::*;

use crate::{
    api::AuthorizedApi,
    app::{LogData, DEFAULT_SSE_URL},
    components::Progress,
};

pub fn is_mounted(cx: Scope) -> impl Fn() -> bool {
    let (mounted, _) = create_signal(cx, ());
    move || -> bool { mounted.try_get_untracked().is_some() }
}

#[component]
pub fn LogView(
    cx: Scope,
    channel: &'static str,
    visible: RwSignal<bool>,
    #[prop(optional)] progress: Option<RwSignal<Progress>>,
) -> impl IntoView {
    let log_data = use_context::<RwSignal<LogData>>(cx).expect("to have found the log data provided");

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
        let log_data = log_data.get();
        let Some(lines) = log_data.get(channel) else {
            return; // no logs for this channel
        };

        log_content.set(lines.join("\n"));

        if let Some(progress) = &progress {
            let Some(line_to_parse) = lines.last() else {
                return; // no logs for this channel
            };

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
    });

    view! { cx,
        <textarea class="textarea log h-full w-full resize-none" readonly node_ref=element>{move || log_content.get()}</textarea>
    }
}
