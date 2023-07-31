use api_schema::response::{State, Status};
use leptos::*;

use crate::{api::AuthorizedApi, app_state::AppState, components::*};

#[component]
pub fn SteamCmdDialog(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let status = app_state.status;
    let api = app_state.api;

    let progress = create_rw_signal(cx, Progress::default());

    let cancel_update_arma = create_action(cx, move |_| {
        let api = api.clone().get_untracked().expect("to have found the api provided");
        async move {
            match api.cancel_update_arma().await {
                Ok(_) => {
                    tracing::info!("Cancelling arma update!");
                }
                Err(err) => {
                    tracing::error!("Unable to cancel arma update: {err}");
                    app_state.toast(
                        cx,
                        format!("Unable to cancel arma update: {err}"),
                        Some(ToastStyle::Error),
                    );
                }
            }
        }
    });

    let checked = Signal::derive(cx, move || {
        if let Some(status) = status.get() {
            status.steamcmd != State::Stopped
        } else {
            false
        }
    });

    view! { cx,
        <input type="checkbox" id="my-modal-5" class="modal-toggle" checked=move || checked.get() />
        <div class="modal">
            <div class="modal-box w-11/12 max-w-5xl h-4/6 flex flex-col">
                <h3 class="font-bold text-lg">"steamcmd.log"</h3>
                <div class="grow shrink bg-base-200 shadow-inner">
                <ClientOnly>
                    <LogView channel="steamcmd" visible=checked.into() progress=progress />
                </ClientOnly>
            </div>
            <ProgressBar values=progress />
            <div class="text-right mt-2">
                <button class="btn btn-error" on:click=move |_| cancel_update_arma.dispatch(())>"Cancel"</button>
            </div>
            </div>
        </div>
    }
    .into_view(cx)
}
