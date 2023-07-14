use api_schema::response::{State, Status};
use leptos::*;

use crate::api::AuthorizedApi;

#[component]
pub fn ServerButtons(cx: Scope) -> impl IntoView {
    let status = use_context::<RwSignal<Option<Status>>>(cx).expect("to have found the status provided");
    let api = use_context::<AuthorizedApi>(cx).expect("to have found the api provided");

    let update_arma = create_action(cx, move |_| {
        let api = api.clone();
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

    let on_start = move |_| {
        update_arma.dispatch(());
    };

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
                        <button class="btn glass hover:bg-error btn-error w-32" disabled=disabled on:click=move |_| ()>"Stop"</button>
                        <button class="btn glass hover:bg-warning btn-warning w-32" disabled=disabled on:click=move |_| ()>"Restart"</button>
                    </div>
                }.into_view(cx)
            }
            State::Stopped => {
                view ! {
                    cx,
                    <div class="btn-group">
                        <button class="btn glass hover:bg-success btn-success w-32" disabled=disabled on:click=on_start>"Start"</button>
                    </div>
                }.into_view(cx)
            }
            _ => {
                // working on _something_
                view! { cx,
                    <span class="loading loading-bars text-primary"></span>
                    {format!("{:?}", status.arma)}
                }.into_view(cx)
            }

        }
    }
}
