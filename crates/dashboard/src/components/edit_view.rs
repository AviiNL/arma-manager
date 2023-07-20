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
pub fn EditView(cx: Scope, channel: MaybeSignal<String>) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let arma_config = app_state.config.clone();

    let content = create_rw_signal(cx, String::new());

    let element = create_node_ref::<Textarea>(cx);

    let achannel = channel.clone();
    create_effect(cx, move |_| {
        let channel = achannel.get();
        let arma_config = arma_config.get();
        let new_content = match arma_config.get(&channel) {
            Some(c) => c.join("\n"),
            None => String::default(),
        };
        content.set(new_content);
    });

    let UseScrollReturn { set_y, .. } = use_scroll_with_options(
        cx,
        element,
        UseScrollOptions::default().behavior(ScrollBehavior::Smooth),
    );

    let achannel = channel.clone();
    let save = create_action(cx, move |()| {
        let channel = achannel.get();
        async move {
            let api = app_state.api.get_untracked().expect("api to exist");
            let content = content.get_untracked();
            api.save_config(channel, content).await;
        }
    });

    view! { cx,
        <div class="relative h-full flex flex-col">
            <div class="bg-base-100 pb-2 flex-0">
                <div class="btn-group">
                    <button class="btn glass hover:bg-primary btn-warning btn-sm w-8 h-8" on:click=move |_| save.dispatch(())>
                        <i class="fa fa-save"/>
                    </button>
                </div>
            </div>
            <textarea class="textarea log flex-1 w-full resize-none focus:outline-none shadow-inner-xl bg-transparent mt-2" on:input=move |ev| content.set(event_target_value(&ev))>{move || content.get()}</textarea>
        </div>
    }
}
