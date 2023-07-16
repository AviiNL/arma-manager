use api_schema::response::FilteredUser;
use leptos::*;
use leptos_router::*;

use crate::{api::AuthorizedApi, components::LogView};

use super::Page;

#[component]
pub fn Log(cx: Scope) -> impl IntoView {
    let visible = create_rw_signal(cx, true);

    view! { cx,
        <div class="card w-full flex-1 p-6 bg-base-100 shadow-xl mt-2 mb-4">
            <div class="text-xl font-semibold inline-block">
                "Arma Logs"
            </div>
            <div class="divider mt-2"></div>
            <div class="h-full w-full pb-6 bg-base-100 grow">
                <LogView channel="arma" visible=visible />
            </div>
        </div>
    }
}
