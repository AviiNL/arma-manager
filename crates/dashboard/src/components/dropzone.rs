use api_schema::response::{State, Status};
use leptos::{html::*, *};
use leptos_use::*;

use crate::{
    api::AuthorizedApi,
    components::{LogView, Progress, ProgressBar},
};

#[component]
pub fn Dropzone(cx: Scope) -> impl IntoView {
    let document = leptos::window();
    let drop_zone_el = create_node_ref::<Div>(cx);

    let on_drop = |event: UseDropZoneEvent| {
        // called when files are dropped on zone
        for file in event.files {
            tracing::info!("{}: {}", file.name(), file.size());
        }
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
