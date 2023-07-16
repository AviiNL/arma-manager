use api_schema::response::{State, Status};
use leptos::{*, ev};

use crate::{api::AuthorizedApi, app_state::AppState};

#[component]
pub fn ServerButtons(cx: Scope) -> impl IntoView {
    //let status = use_context::<RwSignal<Option<Status>>>(cx).expect("to have found the status provided");
    //let authorized_api = use_context::<AuthorizedApi>(cx).expect("to have found the api provided");

    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let status = app_state.status.clone();
    let api = app_state.api.clone();

    let update_arma = create_action(cx, move |_| {
        let api = api.clone().get_untracked().expect("to have found the api provided");
        async move {
            match api.update_arma().await {
                Ok(_) => {
                    tracing::info!("Updated arma!");
                }
                Err(err) => {
                    tracing::error!("Unable to update arma: {err}");
                }
            }
        }
    });

    let start_arma = create_action(cx, move |_| {
        let api = api.clone().get_untracked().expect("to have found the api provided");
        async move {
            match api.start_arma().await {
                Ok(_) => {
                    tracing::info!("Started arma!");
                }
                Err(err) => {
                    tracing::error!("Unable to start arma: {err}");
                }
            }
        }
    });

    let stop_arma = create_action(cx, move |_| {
        let api = api.clone().get_untracked().expect("to have found the api provided");
        async move {
            match api.stop_arma().await {
                Ok(_) => {
                    tracing::info!("Stopped arma!");
                }
                Err(err) => {
                    tracing::error!("Unable to stop arma: {err}");
                }
            }
        }
    });

    let restart_arma = create_action(cx, move |_| {
        let api = api.clone().get_untracked().expect("to have found the api provided");
        async move {
            match api.restart_arma().await {
                Ok(_) => {
                    tracing::info!("Restarted arma!");
                }
                Err(err) => {
                    tracing::error!("Unable to restart arma: {err}");
                }
            }
        }
    });

    move || {
        let Some(status) = status.get() else {
            // If we don't have a status, we can't do anything
            return view! { cx, <span class="loading loading-bars text-primary"></span> }.into_view(cx)
        };

        // Disable the buttons if steamcmd is running
        let disabled = status.steamcmd != State::Stopped;

        match status.arma {
            State::Running => {
                view ! {
                    cx,
                    <div class="btn-group">
                        <button class="btn glass hover:bg-error btn-error w-32" disabled=disabled on:click=move |_| stop_arma.dispatch(())>"Stop"</button>
                        <button class="btn glass hover:bg-warning btn-warning w-32" disabled=disabled on:click=move |_| restart_arma.dispatch(())>"Restart"</button>
                    </div>
                }.into_view(cx)
            }
            State::Stopped => {
                view ! {
                    cx,
                        <div class="dropdown">
                            <div class="btn-group">
                                <button class="btn glass hover:bg-success btn-success w-24" disabled=disabled onFocus="document.activeElement.blur();" onClick="document.activeElement.blur();" on:click=move |_| start_arma.dispatch(())>"Start"</button>
                                <label for="start_dropdown" tabindex="0" disabled=disabled class="btn btn-ghost glass focus:bg-accent-focus hover:bg-accent btn-accent w-8 rounded-r-lg"><i class="fa fa-chevron-down"></i></label>
                            </div>
                        
                            <ul id="start_dropdown" tabindex="0" class="menu dropdown-content p-1 shadow bg-base-100 rounded-box w-52 mt-2">
                                <li><a class="p-2 rounded-box" href="#" on:click=move |_| update_arma.dispatch(())>"Check for updates"</a></li> 
                            </ul>
                        </div>
                }.into_view(cx)
            }
            _ => {
                view! { cx,
                    <span class="loading loading-bars text-primary"></span>
                    {format!("{:?}", status.arma)}
                }.into_view(cx)
            }

        }
    }
}
