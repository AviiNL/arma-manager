use leptos::*;
use leptos_router::*;

use crate::app_state::AppState;

#[component]
pub fn Missions(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let missions = app_state.missions.clone();

    view! { cx,
        <div class="card w-full flex-1 p-6 bg-base-100 shadow-xl mt-2 mb-4">
            <div class="text-xl font-semibold inline-block">
                "Missions"
            </div>
            <div class="divider mt-2"></div>
            <div class="h-full w-full grow">
                <table class="table table-zebra w-full">
                    <thead>
                        <tr>
                            <th>"Mission"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For each={move || missions.get()} key={move |mission| mission.clone()} view={move |cx, mission| view! { cx,
                            <tr>
                                <td><input class="input input-ghost w-full" readonly value={mission} /></td>
                            </tr>
                        }}/>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
