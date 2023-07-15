use api_schema::response::{State, Status};
use leptos::{html::*, *};
use leptos_use::*;

use crate::{
    api::AuthorizedApi,
    app::LoadingState,
    components::{LogView, Progress, ProgressBar},
};

#[component]
pub fn Loading(cx: Scope) -> impl IntoView {
    let loading = use_context::<RwSignal<LoadingState>>(cx).expect("loading state context to exist");

    view! { cx,
        <input type="checkbox" id="my-modal-5" class="loading-cover-toggle" checked=move || loading.get() == LoadingState::Loading />
        <div class="loading-cover">
            <span class="loading loading-bars text-primary loading-lg"></span>
        </div>
    }
    .into_view(cx)
}
