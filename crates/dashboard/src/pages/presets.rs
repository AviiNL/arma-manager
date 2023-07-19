use api_schema::{request::*, response::*};
use gloo_storage::{LocalStorage, Storage};
use leptos::*;

use crate::{
    app_state::{AppState, Loading},
    components::PresetItem,
};

#[component]
pub fn Presets(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let loading = app_state.loading.clone();
    let presets = app_state.presets.clone();

    let status = app_state.status.clone();

    let hide_blacklisted = create_rw_signal(cx, false);
    create_effect(cx, move |_| {
        let value = LocalStorage::get("hide_blacklisted").unwrap_or_default();
        hide_blacklisted.set(value);
    });

    let selected_preset = create_rw_signal(cx, None::<Preset>);
    let search = create_rw_signal(cx, String::default());

    let filtered_preset = Signal::derive(cx, move || {
        let search = search.get();
        let Some(preset) = selected_preset.get() else {
            return None;
        };

        let mut preset = preset.clone();

        if hide_blacklisted.get() {
            preset.items = preset.items.into_iter().filter(|item| !item.blacklisted).collect();
        }

        if search.is_empty() {
            return Some(preset);
        }

        preset.items = preset
            .items
            .into_iter()
            .filter(|item| item.name.to_lowercase().contains(&search.to_lowercase()))
            .collect();

        Some(preset)
    });

    create_effect(cx, move |_| {
        let presets = presets.get();
        loading.set(Loading::Loading(Some("Loading Presets...")));

        if let Some(preset) = presets.iter().find(|preset| preset.selected) {
            let mut preset = preset.clone();
            preset.items.sort_by(|a, b| a.position.cmp(&b.position));
            selected_preset.set(Some(preset.clone()));
        };

        loading.set(Loading::Ready);
    });

    let select_preset = create_action(cx, move |id: &i64| {
        let id = id.clone();
        async move {
            if let Some(preset) = filtered_preset.get_untracked() {
                if preset.id == id {
                    return;
                }
            }
            search.set(String::default());
            loading.set(Loading::Loading(None));

            let api = app_state.api.get_untracked().expect("there to be an Api");

            let schema = SelectPresetSchema { id };

            api.activate_preset(&schema).await.unwrap();
        }
    });

    let toggle_hide_blacklisted = create_action(cx, move |()| async move {
        let value = hide_blacklisted.get_untracked();
        LocalStorage::set("hide_blacklisted", !value).unwrap();
        hide_blacklisted.set(!value);
    });

    let download_missing_mods = create_action(cx, move |()| async move {
        let api = app_state.api.get_untracked().expect("there to be an Api");
        api.download_missing_mods().await.unwrap();
    });

    let force_check = create_action(cx, move |()| async move {
        let api = app_state.api.get_untracked().expect("there to be an Api");
        api.force_check().await.unwrap();
    });

    let delete_preset = create_action(cx, move |id: &i64| {
        let id = id.clone();
        async move {
            let api = app_state.api.get_untracked().expect("there to be an Api");
            let schema = DeletePresetSchema { id };
            api.delete_preset(&schema).await.unwrap();
        }
    });

    view! { cx,
        <div class="card w-full p-6 bg-base-100 shadow-xl mt-2 mb-4">
            <div class="flex justify-between text-sm font-semibold">
                <div class="dropdown">
                    <label class="btn gap-1 normal-case btn-ghost" tabindex="0">
                        <span class="truncate">
                        {move || {
                            if let Some(active_preset) = selected_preset.get() {
                                let enabled_count = active_preset.items.iter().filter(|item| item.enabled && !item.blacklisted).count();
                                let item_count = active_preset.items.iter().filter(|item| !item.blacklisted).count();
                                format!("{} ({}/{})", active_preset.name, enabled_count, item_count)
                            } else {
                                "No Preset Selected".to_string()
                            }
                        }}</span>
                        <i class="fa fa-caret-down"></i>
                    </label>
                    <ul tabindex="0" class="dropdown-content menu p-2 shadow-lg bg-base-100 rounded-box w-fit">
                        <For
                            each={move || presets.get()}
                            key={|item| item.id}
                            view={move |cx, item| view! { cx,
                                <li>
                                    <div class="flex whitespace-nowrap items-stretch" onClick="document.activeElement.blur();" on:click=move |_| select_preset.dispatch(item.id)>
                                        <div class="flex flex-1 grow items-center">
                                            <a href="#">{item.name}</a>
                                        </div>
                                        <div class="flex-0">
                                            {move || {
                                                let mut disabled = false;
                                                if let Some(active_preset) = selected_preset.get() {
                                                    if active_preset.id == item.id {
                                                        disabled = true;
                                                    }
                                                }

                                                view!(cx,
                                                    <button
                                                        class="btn btn-ghost btn-sm hover:bg-error hover:text-error-content"
                                                        onClick="document.activeElement.blur();"
                                                        disabled=disabled
                                                        on:click=move |_| delete_preset.dispatch(item.id)
                                                        title="Delete Preset">
                                                        <i class="fa fa-trash"></i>
                                                    </button>
                                                )
                                            }}

                                        </div>
                                    </div>
                                </li>
                            }.into_view(cx)} />
                    </ul>
                </div>

                <div class="order-last flex flex-none gap-2">
                    <div class="form-control">
                        // show/hide blacklisted items button
                        <button class="btn btn-ghost" on:click={move |_| toggle_hide_blacklisted.dispatch(())}>
                            {move ||
                                if let Some(active_preset) = selected_preset.get() {
                                    let blacklist_count = active_preset.items.iter().filter(|item| item.blacklisted).count();
                                    if hide_blacklisted.get() {
                                        format!("Show Blacklisted ({})", blacklist_count)
                                    } else {
                                        format!("Hide Blacklisted ({})", blacklist_count)
                                    }
                                } else {
                                    if hide_blacklisted.get() {
                                        format!("Show Blacklisted")
                                    } else {
                                        format!("Hide Blacklisted")
                                    }
                                }
                            }
                        </button>

                    </div>
                    <div class="form-control">
                        <input type="text" placeholder="Search…" class="input input-bordered" prop:value={move || search.get()} on:input=move |ev| search.set(event_target_value(&ev)) />
                    </div>

                    <div class="dropdown dropdown-end">
                        <div class="btn-group">
                            <button
                                class="btn btn-ghost hover:glass"
                                onClick="document.activeElement.blur();"
                                on:click=move |_| download_missing_mods.dispatch(())
                                title="Start Download">
                                <i class="text-lg fa-brands fa-steam"></i>
                            </button>
                            <label tabindex="0" class="btn btn-ghost hover:glass w-4 rounded-r-lg">
                                <i class="fa fa-chevron-down"></i>
                            </label>
                        </div>

                        <ul tabindex="0" class="menu dropdown-content p-1 shadow bg-base-100 rounded-box w-fit mt-2">
                            <li>
                                <a
                                    class="p-2 rounded-box whitespace-nowrap hover:glass"
                                    href="#"
                                    onClick="document.activeElement.blur();"
                                    on:click=move |_| force_check.dispatch(())
                                    title="Force Check">
                                    "Force Check"
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
            <div class="divider mt-2"></div>
            <div class="max-w-full h-full overflow-y-auto">
                <table class="table table-fixed table-zebra w-full">
                    <tbody>
                        {move || if let Some(filtered_preset) = filtered_preset.get() {
                            view! { cx, <For each={move || filtered_preset.items.clone()} key={|item| item.id} view={move |cx, item| view! { cx, <PresetItem item=item.clone() /> }.into_view(cx)} /> }.into_view(cx)
                        } else {
                            view! { cx,
                                <tr>
                                    <td class="text-center" colspan="3">No Preset Selected</td>
                                </tr>
                            }.into_view(cx)
                        }}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
