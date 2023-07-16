use api_schema::response::Preset;
use leptos::*;

use crate::app_state::{AppState, Loading};

#[component]
pub fn Presets(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let loading = app_state.loading.clone();
    let presets = app_state.presets.clone();

    let active_preset = create_rw_signal(cx, None::<Preset>);

    create_effect(cx, move |_| {
        loading.set(Loading::Loading(Some("Loading Presets...")));
        let presets = presets.get();
        // find the preset with the selected flag
        if let Some(preset) = presets.iter().find(|preset| preset.selected) {
            active_preset.set(Some(preset.clone()));
        };
        loading.set(Loading::Ready);
    });

    let set_active = create_action(cx, move |id: &i64| {
        let id = id.clone();
        async move {
            // unset the active flag on all presets
            presets.update(|list| {
                list.iter_mut().for_each(|preset| {
                    preset.selected = false;
                    if preset.id == id {
                        preset.selected = true;
                    }
                });
            });
        }
    });

    view! { cx,
        <div class="card w-full flex-1 p-6 bg-base-100 shadow-xl mt-2 mb-4">
            <div class="text-sm font-semibold inline-block">
                <div class="dropdown" title="Change Preset">
                    <label class="btn gap-1 normal-case btn-ghost" tabindex="0">
                        <span class="truncate">{move || if let Some(active_preset) = active_preset.get() { active_preset.name } else { "No Preset Selected".to_string() } }</span>
                        <i class="fa fa-caret-down"></i>
                    </label>
                    <ul tabindex="0" class="dropdown-content menu p-2 shadow-lg bg-base-100 rounded-box w-52">
                        <For each={move || presets.get()} key={|item| item.id} view={move |cx, item| view! { cx, <li><a onClick="document.activeElement.blur();" on:click=move |_| set_active.dispatch(item.id)>{item.name}</a></li>}.into_view(cx)} />
                    </ul>
                </div>
            </div>
            <div class="divider mt-2"></div>
            <div class="h-full w-full grow">
                <table class="table w-full">
                    <tr>
                        <th></th>
                        <th>"Mod"</th>
                    </tr>
                </table>
            </div>
        </div>
    }
}
