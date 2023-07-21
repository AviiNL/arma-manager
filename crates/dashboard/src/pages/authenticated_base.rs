use api_schema::response::FilteredUser;
use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use leptos::*;
use leptos::{svg::Filter, *};
use leptos_router::*;

use crate::{
    app_state::{AppState, Loading},
    components::*,
};

use super::Page;

#[component]
pub fn AuthenticatedBase(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let loading = app_state.loading.clone();

    app_state.check_auth(cx);

    create_effect(cx, move |_| {
        request_animation_frame(move || {
            loading.set(Loading::Ready);
        })
    });

    view! { cx,
        <div class="drawer drawer-mobile">
            <input id="left-sidebar-drawer" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content flex flex-col">
                <Header />
                <main class="flex flex-1 overflow-y-auto pt-6 px-6 bg-base-200 flex-col md:flex-row">
                    <Outlet/>
                </main>
            </div>
            <LeftSidebar />
        </div>
        <SteamCmdDialog />
        <ClientOnly>
            <Dropzone />
        </ClientOnly>
    }
    .into_view(cx)
}
