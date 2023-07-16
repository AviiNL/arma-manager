use api_schema::response::PresetItem;
use leptos::*;

#[component]
pub fn PresetItem(cx: Scope, item: PresetItem) -> impl IntoView {
    let enabled = create_rw_signal(cx, item.enabled);

    let name = item.name.clone();
    let toggle = create_action(cx, move |value: &bool| {
        let id = item.id.clone();
        let value = value.clone();
        let name = name.clone();
        async move {
            tracing::info!("Toggling [{}] {}: {:?}", id, name, value);

            // send new value to backend

            enabled.set(value);
        }
    });

    view! { cx,
        <tr>
            <td class="text-center m-none p-none">
            <label>
                <input type="checkbox" class="checkbox" on:change={move |ev| toggle.dispatch(event_target_checked(&ev)) } checked={move || enabled.get()} />
            </label>
            </td>
            <td class="w-full align-middle">{ item.name }</td>
        </tr>
    }
}
