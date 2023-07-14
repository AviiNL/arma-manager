use api_schema::response::FilteredUser;
use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use leptos::{svg::Filter, *};
use leptos_router::*;

use crate::{
    api::AuthorizedApi,
    app::{Theme, DEFAULT_SSE_URL},
    components::*,
};

use super::Page;

#[component]
pub fn Layout<F>(cx: Scope, on_logout: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    view! { cx,
        <>
            <div class="drawer drawer-mobile">
                <input id="left-sidebar-drawer" type="checkbox" class="drawer-toggle" />
                <PageContent on_logout />
                <LeftSidebar />
            </div>
            <SteamCmdDialog />
        </>
    }
    .into_view(cx)
}
