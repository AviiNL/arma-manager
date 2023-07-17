use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use leptos::{html::Textarea, *};
use leptos_use::*;

use crate::{api::AuthorizedApi, app::LogData, app_state::AppState, components::Progress};

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
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let log_data = app_state.log.clone();

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
            // log_content.set(String::new());
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

        if let Some(progress) = progress {
            if let Some(line) = lines.last() {
                if line.contains("progress:") {
                    progress.update(|p| p.update_from_line(line));
                } else {
                    // but only if it's not already 0,0
                    if progress.get_untracked().value != 0 || progress.get_untracked().max != 0 {
                        progress.update(|p| p.update(0, 0));
                    }
                }
            }
        }
    });

    let clear_log = create_action(cx, move |()| {
        let log_data = log_data.clone();
        async move {
            log_data.update(move |log| {
                log.insert(channel.to_string(), vec![]);
            });
        }
    });

    view! { cx,
        <div class="relative h-full">
        <textarea class="textarea log h-full w-full resize-none focus:outline-none shadow-inner-xl bg-transparent" readonly node_ref=element>{move || log_content.get()}</textarea>
        <button class="btn btn-circle btn-xs btn-ghost hover:glass absolute top-0 right-4 m-2" on:click=move |_| clear_log.dispatch(()) title="Clear log">
            <i class="fa fa-eraser" />
        </button>

        </div>
    }
}
