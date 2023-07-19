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

    let toggle_blacklist = create_action(cx, move |()| {
        let published_file_id = item.published_file_id.clone();
        let value = !item.blacklisted;
        async move {
            let api = app_state.api.get_untracked().expect("there to be an Api");
            let schema = BlacklistItemSchema { published_file_id };

            if value {
                api.blacklist_item(&schema).await.unwrap();
            } else {
                api.unblacklist_item(&schema).await.unwrap();
            }
        }
    });

    view! { cx,
        <tr class=("blacklist", move || item.blacklisted) class="">
            <td>
                <div class="flex items-center">
                    <div class="h-full text-center flex-0 mr-4">
                        <label>
                            <input type="checkbox" class="checkbox" disabled=item.blacklisted on:change={move |ev| toggle.dispatch(event_target_checked(&ev)) } checked={move || enabled.get()} />
                        </label>
                    </div>
                    <div class="h-full text-left flex-1 whitespace-normal">
                        {if item.exists {
                            view! { cx, <p class=("text-base-content", move || !item.blacklisted) >{item.name}</p> }.into_view(cx)
                        } else {
                            view! { cx, <s>{item.name}</s> }.into_view(cx)
                        }}
                    </div>
                    <div class="h-full text-center flex-0 ml-5">
                        <button class="btn btn-sm btn-ghost hover:glass" on:click=move |_| toggle_blacklist.dispatch(())>
                        {if item.blacklisted {
                            view! { cx, <i class="fa-regular fa-circle-xmark"></i> }
                        } else {
                            view! { cx, <i class="fa-regular fa-circle-check"></i> }
                        }}
                        </button>
                    </div>
                </div>
            </td>
        </tr>
    }
}
