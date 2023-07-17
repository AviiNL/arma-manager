use crate::app_state::AppState;
use api_schema::{request::*, response::*};
use leptos::*;

#[component]
pub fn PresetItem(cx: Scope, item: PresetItem) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("there to be an AppState");
    let enabled = create_rw_signal(cx, item.enabled);

    let name = item.name.clone();
    let toggle = create_action(cx, move |value: &bool| {
        let id = item.id.clone();
        let value = value.clone();
        let name = name.clone();
        async move {
            let api = app_state.api.get_untracked().expect("there to be an Api");
            let schema = UpdatePresetItemSchema {
                id,
                enabled: Some(value),
                position: None,
            };

            // send new value to backend
            api.set_preset_item_enabled(&schema).await.unwrap();

            enabled.set(value);
        }
    });

    view! { cx,
        <tr>
            <td class="text-center align-middle w-0">
            <label>
                <input type="checkbox" class="checkbox" on:change={move |ev| toggle.dispatch(event_target_checked(&ev)) } checked={move || enabled.get()} />
            </label>
            </td>
            <td class="align-middle whitespace-normal">{ item.name }</td>
        </tr>
    }
}
