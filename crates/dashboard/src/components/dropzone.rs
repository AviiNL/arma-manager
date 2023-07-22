use std::time::Duration;

use api_schema::response::{Preset, State, Status};
use js_sys::Uint8Array;
use leptos::{html::*, *};
use leptos_router::*;
use leptos_use::*;
use web_sys::File;

use crate::{
    api::AuthorizedApi,
    app::sleep,
    app_state::{self, AppState, Loading},
    components::{LogView, Progress, ProgressBar},
    pages::Page,
};

#[component]
pub fn Dropzone(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let api = app_state.api.clone();
    let loading = app_state.loading.clone();

    let document = leptos::window();

    let on_drop = move |event: UseDropZoneEvent| {
        spawn_local(async move {
            let api = api.get_untracked().expect("api to exist");

            loading.set(Loading::Loading(Some("Processing files...")));

            // called when files are dropped on zone
            for file in event.files {
                if let Err(e) = validate_and_upload(cx, &api, &file).await {
                    app_state.toast(
                        cx,
                        &format!("Failed to upload file: {}", e),
                        Some(super::ToastStyle::Error),
                    );
                }
            }

            loading.set(Loading::Ready);
        });
    };

    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(cx, document, UseDropZoneOptions::default().on_drop(on_drop));

    view! { cx,
        <input type="checkbox" id="my-modal-5" class="dropzone-toggle" checked=move || is_over_drop_zone.get() />
        <div class="dropzone">
            "Drop the mods, the presets, or missions here."
        </div>
    }
    .into_view(cx)
}

async fn validate_and_upload(cx: Scope, api: &AuthorizedApi, file: &File) -> Result<(), Box<dyn std::error::Error>> {
    let name = file.name();

    let js_future = wasm_bindgen_futures::JsFuture::from(file.array_buffer());

    let jsval = js_future.await.unwrap();
    let arr: Uint8Array = Uint8Array::new(&jsval);
    let data: Vec<u8> = arr.to_vec();

    if let Ok(data) = String::from_utf8(data.clone()) {
        if crate::preset_parser::is_preset(&data) {
            let data = crate::preset_parser::parse(&data)?;
            api.post_preset(&data).await?;

            use_navigate(cx)("/console/presets", Default::default()).expect("Presets route");

            return Ok(());
        }
    }

    if name.ends_with(".pbo") {
        api.upload_mission(file).await.unwrap();

        use_navigate(cx)("/console/missions", Default::default()).expect("Presets route");

        return Ok(());
    }

    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    app_state.toast(cx, "Invalid file type", Some(super::ToastStyle::Error));

    Ok(())
}
