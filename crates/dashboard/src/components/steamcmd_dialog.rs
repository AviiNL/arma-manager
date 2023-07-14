use api_schema::response::{State, Status};
use leptos::*;

use crate::{
    api::AuthorizedApi,
    components::{LogView, Progress, ProgressBar},
};

#[component]
pub fn SteamCmdDialog(cx: Scope) -> impl IntoView {
    let status = use_context::<RwSignal<Option<Status>>>(cx).expect("to have found the status provided");

    let checked = create_rw_signal(cx, false);

    let progress = create_rw_signal(cx, Progress::default());

    let api = use_context::<AuthorizedApi>(cx).expect("to have found the api provided");

    let cancel_update_arma = create_action(cx, move |_| {
        let api = api.clone();
        async move {
            match api.cancel_update_arma().await {
                Ok(_) => {
                    tracing::info!("Cancelling arma update!");
                }
                Err(err) => {
                    tracing::error!("Unable to cancel arma update: {err}");
                }
            }
        }
    });

    create_effect(cx, move |_| {
        if let Some(status) = status.get() {
            checked.set(status.steamcmd != State::Stopped);
        }
    });

    view! { cx,
        <input type="checkbox" id="my-modal-5" class="modal-toggle" checked=move || checked.get() />
        <div class="modal">
            <div class="modal-box w-11/12 max-w-5xl h-4/6 flex flex-col">
                <h3 class="font-bold text-lg">"steamcmd.log"</h3>
                <div class="grow shrink">
                <LogView channel="steamcmd" visible=checked progress=progress />
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
