use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use leptos::{html::Textarea, *};
use leptos_use::*;

use crate::{api::AuthorizedApi, app::LogData, app_state::AppState, components::Progress};

#[component]
pub fn EditView<F>(cx: Scope, channel: F) -> impl IntoView
where
    F: Fn() -> String + 'static + Clone,
{
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let arma_config = app_state.config.clone();

    let element = create_node_ref::<Textarea>(cx);

    let initial_load = create_rw_signal(cx, true);

    let c = channel.clone();
    create_effect(cx, move |_| {
        let Some(element) = element.get() else {
            return;
        };
        let channel = c();
        let arma_config = arma_config.get();
        let new_content = match arma_config.get(&*channel) {
            Some(c) => c.join("\n"),
            None => String::default(),
        };
        element.set_value(&new_content);
    });

    let save = create_action(cx, move |channel: &String| {
        let channel = channel.clone();
        let content = element.get().expect("textarea to exist").value();
        async move {
            let api = app_state.api.get_untracked().expect("api to exist");
            api.save_config(channel, content).await;
        }
    });

    view! { cx,
        <div class="relative h-full flex flex-col">
            <div class="bg-base-100 pb-2 flex-0">
                <div class="btn-group">
                    <button class="btn glass hover:bg-primary btn-warning btn-sm w-8 h-8" on:click=move |_| save.dispatch(channel())>
                        <i class="fa fa-save"/>
                    </button>
                </div>
            </div>
            <textarea
                class="textarea log flex-1 w-full resize-none focus:outline-none shadow-inner-xl bg-transparent mt-2"
                node_ref=element></textarea>
        </div>
    }
}
