use leptos::*;
use leptos_router::*;

use crate::{app::Theme, components::Header};

#[component]
pub fn PageContent<F>(cx: Scope, on_logout: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    view! { cx,
        <div class="drawer-content flex flex-col">
            <Header on_logout=on_logout />
            <main class="flex flex-1 overflow-y-auto pt-6 px-6  bg-base-200">
                <Outlet/>
                <div class="h-16"></div>
            </main>
        </div>
    }
}
