use api_schema::response::Preset;
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

    let active_preset = create_rw_signal(cx, None::<Preset>);

    create_effect(cx, move |_| {
        loading.set(Loading::Loading(Some("Loading Presets...")));
        let presets = presets.get();
        // find the preset with the selected flag
        if let Some(preset) = presets.iter().find(|preset| preset.selected) {
            let mut preset = preset.clone();
            // we need to sort the items inside preset by position
            preset.items.sort_by(|a, b| a.position.cmp(&b.position));
            active_preset.set(Some(preset));
        };
        loading.set(Loading::Ready);
    });

    let set_active = create_action(cx, move |id: &i64| {
        let id = id.clone();
        async move {
            // this will probably be done by SSE once we update the actual database
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
        <div class="card w-full p-6 bg-base-100 shadow-xl mt-2 mb-4">
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
            <div class="w-full h-full overflow-auto">
                <table class="table table-zebra w-full">
                    <tbody>
                        {move || if let Some(active_preset) = active_preset.get() {
                            view! { cx, <For each={move || active_preset.items.clone()} key={|item| item.id} view={move |cx, item| view! { cx, <PresetItem item=item.clone() /> }.into_view(cx)} /> }.into_view(cx)
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
