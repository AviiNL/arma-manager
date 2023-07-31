use crate::app_state::AppState;
use api_schema::{request::*, response::*};
use leptos::*;

#[component]
pub fn PresetDlc(cx: Scope, item: DlcItem) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("there to be an AppState");
    let enabled = create_rw_signal(cx, item.enabled);

    let name = item.name.clone();
    let toggle = create_action(cx, move |value: &bool| {
        let id = item.id;
        let value = *value;
        let name = name.clone();
        async move {
            let api = app_state.api.get_untracked().expect("there to be an Api");
            let schema = UpdatePresetDlcSchema {
                id,
                enabled: Some(value),
                position: None,
            };

            // send new value to backend
            api.set_preset_dlc_enabled(&schema).await.unwrap();

            enabled.set(value);
        }
    });

    view! { cx,
        <tr>
            <td>
                <div class="flex items-center">
                    <div class="h-full text-center flex-0 mr-4">
                        <label>
                            <input type="checkbox" class="checkbox" on:change={move |ev| toggle.dispatch(event_target_checked(&ev)) } checked={move || enabled.get()} />
                        </label>
                    </div>
                    <div class="h-full text-left flex-1 whitespace-normal">
                        <p class="text-base-content" >{item.name}</p>
                    </div>
                </div>
            </td>
        </tr>
    }
}
