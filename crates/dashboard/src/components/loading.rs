use api_schema::response::{State, Status};
use leptos::{html::*, *};
use leptos_use::*;

use crate::{
    api::AuthorizedApi,
    app_state::{self, AppState},
    components::{LogView, Progress, ProgressBar},
};

#[component]
pub fn Loading(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");

    let loading = app_state.loading.clone();

    let checked = move || match loading.get() {
        app_state::Loading::Loading(_) => true,
        _ => false,
    };

    let text = move || match loading.get() {
        app_state::Loading::Loading(text) => text,
        _ => None,
    };

    view! { cx,
        <input type="checkbox" id="my-modal-5" class="loading-cover-toggle" checked=checked />
        <div class="loading-cover">
            <p class="loading loading-bars text-primary loading-lg"></p>
            { move || if text().is_some() {
                view! { cx, <p class="text-base-content">{text().unwrap()}</p> }.into_view(cx)
            } else {
                view! { cx, <p></p> }.into_view(cx)
            }}
        </div>
    }
    .into_view(cx)
}
