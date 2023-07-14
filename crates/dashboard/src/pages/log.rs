use api_schema::response::FilteredUser;
use leptos::*;
use leptos_router::*;

use crate::{api::AuthorizedApi, components::LogView, pages::layout::Layout};

use super::Page;

#[component]
pub fn Log(cx: Scope) -> impl IntoView {
    let visible = create_rw_signal(cx, true);

    view! { cx,
        <h1>"Arma Logs"</h1>
        <div class="hero h-5/6 bg-base-200">
            <LogView channel="arma" visible=visible />
        </div>
    }
}
