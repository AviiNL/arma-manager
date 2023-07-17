use std::time::Duration;

use api_schema::response::{Preset, State, Status};
use js_sys::Uint8Array;
use leptos::{html::*, *};
use leptos_router::*;
use leptos_use::*;

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

            // I guess we could just always do this, if we're already there, this hopefully just does nothing
            use_navigate(cx)("/console/presets", Default::default()).expect("Presets route");

            loading.set(Loading::Loading(Some("Processing files...")));

            // called when files are dropped on zone
            for file in event.files {
                let js_future = wasm_bindgen_futures::JsFuture::from(file.array_buffer());

                // at this point in time we need to figure out what kind of file it is...
                // if it's a preset, we need to parse it and then post it to the server
                // but it can also be a mission or a standalone mod, so we need to figure out what to do with those
                let jsval = js_future.await.unwrap();
                let arr: Uint8Array = Uint8Array::new(&jsval);
                let data: Vec<u8> = arr.to_vec();

                validate_and_upload(&api, &file.name(), data).await.unwrap(); // TODO: handle errors
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

async fn validate_and_upload(
    api: &AuthorizedApi,
    name: &str,
    buffer: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    // do this based off of the uploaded filename? that'll catch like 99% of the cases

    tracing::info!("Processing {}", name);

    // If it parses as a string, we'll treat it as a preset (for now)
    // We'll add a new function to actually validate and return some Enum with possible types,
    // eg Preset, Mission, Mod, etc
    if let Ok(data) = String::from_utf8(buffer) {
        if crate::preset_parser::is_preset(&data) {
            let data = crate::preset_parser::parse(&data)?;
            api.post_preset(&data).await?;
            return Ok(());
        }
    }

    tracing::error!("Unknown file type");

    Ok(())
}

// enum UploadFileTypes {
//     Preset(String),             // the contents of the html file
//     Mission((String, Vec<u8>)), // Name of the mission and the contents of the pbo
//     Mod,                        // zip file? this might actually also be .pbo
// }
