use api_schema::{a2s::Player, response::FilteredUser};
use leptos::*;
use leptos_router::*;

use crate::{api::AuthorizedApi, app_state::AppState};

use super::Page;

#[component]
pub fn Dashboard(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    // if None show "Loading..."
    let server_name = Signal::derive(cx, move || app_state.server_info.get().map(|info| info.name.clone()));
    let mission = Signal::derive(cx, move || app_state.server_info.get().map(|info| info.game.clone()));
    let map = Signal::derive(cx, move || app_state.server_info.get().map(|info| info.map.clone()));
    let online_players = Signal::derive(cx, move || app_state.server_info.get().map(|info| info.players));
    let max_players = Signal::derive(cx, move || app_state.server_info.get().map(|info| info.max_players));
    let port = Signal::derive(cx, move || {
        app_state.server_info.get().map(|info| info.extended_server_info.port)
    });

    let players = app_state.players.clone();

    let server = Signal::derive(cx, move || {
        app_state
            .server_info
            .get()
            .map(|info| format!("{} {}", info.server_type.clone(), info.server_os.clone()))
    });

    view! { cx,
        <div class="h-full w-full bg-base-200">
            <div class="stats shadow mb-8">
                <div class="stat">
                    <div class="stat-title">"Server Name"</div>
                    <div class="stat-desc">
                        {move || server_name.get().unwrap_or_else(|| "Loading...".to_string())}
                    </div>
                </div>

                <div class="stat">
                    <div class="stat-title">"Map"</div>
                    <div class="stat-desc">
                        {move || map.get().unwrap_or_else(|| "Loading...".to_string())}
                    </div>
                </div>

                <div class="stat">
                    <div class="stat-title">"Mission"</div>
                    <div class="stat-desc">
                        {move || mission.get().unwrap_or_else(|| "Loading...".to_string())}
                    </div>
                </div>

                <div class="stat">
                    <div class="stat-title">"Players"</div>
                    <div class="stat-desc">
                        {move || online_players.get().unwrap_or_else(|| 0)}/{move || max_players.get().unwrap_or_else(|| 0)}
                    </div>
                </div>
            </div>

            <div class="card w-96 bg-base-100 shadow-xl">
                <div class="card-body">
                    <h2 class="card-title">"Online Players"</h2>
                    <div class="overflow-y-auto">
                        <table class="table w-full">
                            <thead>
                                <tr>
                                    <th>"Name"</th>
                                    <th>"Score"</th>
                                    <th>"Time"</th>
                                </tr>
                            </thead>
                            <tbody>

                                { move || {
                                    let players = players.get();
                                    let mut v = Vec::with_capacity(players.len());
                                    for player in players {
                                        v.push(view! { cx, <Player player=player /> });
                                    }
                                    v
                                }}

                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Player(cx: Scope, player: Player) -> impl IntoView {
    use chrono::{Duration, TimeZone, Utc};

    let timestamp = std::time::Duration::from_secs(player.duration as u64);
    let time = indicatif::HumanDuration(timestamp);

    view! { cx, <tr>
        <td>{player.name.clone()}</td>
        <td>{player.score}</td>
        <td>{time.to_string()}</td>
    </tr> }
}
