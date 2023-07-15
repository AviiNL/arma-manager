use std::time::Duration;

use api_schema::response::{State, Status};
use leptos::{html::*, *};
use leptos_use::*;

use crate::{
    api::AuthorizedApi,
    app::{sleep, LoadingState},
    components::{LogView, Progress, ProgressBar},
};

#[component]
pub fn Dropzone(cx: Scope) -> impl IntoView {
    let loading = use_context::<RwSignal<LoadingState>>(cx).expect("loading context to exist");

    let document = leptos::window();
    let drop_zone_el = create_node_ref::<Div>(cx);

    let on_drop = move |event: UseDropZoneEvent| {
        spawn_local(async move {
            let api = use_context::<AuthorizedApi>(cx).expect("api context to exist");

            loading.set(LoadingState::Loading);

            // called when files are dropped on zone
            for file in event.files {
                let js_future = wasm_bindgen_futures::JsFuture::from(file.text());
                let data = js_future.await.unwrap().as_string().unwrap();

                let data = crate::preset_parser::parse(&data).unwrap();

                // all this shit needs to get split up
                let preset = api.post_preset(&data).await.unwrap();

                tracing::info!("{:?}", preset);
            }

            loading.set(LoadingState::Ready);
        });
    };

    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(cx, document, UseDropZoneOptions::default().on_drop(on_drop));

    view! { cx,
        <input type="checkbox" id="my-modal-5" class="dropzone-toggle" checked=move || is_over_drop_zone.get() />
        <div class="dropzone" node_ref=drop_zone_el>
            "Drop the mods, the presets, or missions here."
        </div>
    }
    .into_view(cx)
}
