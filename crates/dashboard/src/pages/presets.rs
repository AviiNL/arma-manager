use api_schema::response::{Preset, State};
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

    let selected_preset = create_rw_signal(cx, None::<Preset>);

    create_effect(cx, move |_| {
        let presets = presets.get();
        loading.set(Loading::Loading(Some("Loading Presets..."))); // doesnt show up?

        // find the preset with the selected flag
        if let Some(preset) = presets.iter().find(|preset| preset.selected) {
            let mut preset = preset.clone();
            // we need to sort the items inside preset by position
            preset.items.sort_by(|a, b| a.position.cmp(&b.position));
            selected_preset.set(Some(preset));
        };
        loading.set(Loading::Ready);
    });

    let select_preset = create_action(cx, move |id: &i64| {
        let id = id.clone();
        async move {
            if let Some(preset) = selected_preset.get_untracked() {
                if preset.id == id {
                    return;
                }
            }
            loading.set(Loading::Loading(None));

            let api = app_state.api.get_untracked().expect("there to be an Api");

            let schema = api_schema::request::SelectPresetSchema { id };

            api.activate_preset(&schema).await.unwrap();
        }
    });

    let download_missing_mods = create_action(cx, move |()| async move {
        let api = app_state.api.get_untracked().expect("there to be an Api");
        api.download_missing_mods().await.unwrap();
    });

    view! { cx,
        <div class="card w-full p-6 bg-base-100 shadow-xl mt-2 mb-4">
            <div class="flex justify-between text-sm font-semibold">
                <div class="dropdown" title="Change Preset">
                    <label class="btn gap-1 normal-case btn-ghost" tabindex="0">
                        <span class="truncate">{move || if let Some(active_preset) = selected_preset.get() { active_preset.name } else { "No Preset Selected".to_string() } }</span>
                        <i class="fa fa-caret-down"></i>
                    </label>
                    <ul tabindex="0" class="dropdown-content menu p-2 shadow-lg bg-base-100 rounded-box w-52">
                        <For each={move || presets.get()} key={|item| item.id} view={move |cx, item| view! { cx, <li><a onClick="document.activeElement.blur();" on:click=move |_| select_preset.dispatch(item.id)>{item.name}</a></li>}.into_view(cx)} />
                    </ul>
                </div>

                <div class="order-last">

                    <div class="dropdown dropdown-end">
                        <div class="btn-group">
                            <button
                                class="btn btn-ghost hover:glass"
                                onFocus="document.activeElement.blur();"
                                onClick="document.activeElement.blur();"
                                on:click=move |_| download_missing_mods.dispatch(())
                                title="Download missing">
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
                                    onFocus="document.activeElement.blur();"
                                    onClick="document.activeElement.blur();"
                                    // on:click=move |_| download_all_mods.dispatch(())
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
                        {move || if let Some(selected_preset) = selected_preset.get() {
                            view! { cx, <For each={move || selected_preset.items.clone()} key={|item| item.id} view={move |cx, item| view! { cx, <PresetItem item=item.clone() /> }.into_view(cx)} /> }.into_view(cx)
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
