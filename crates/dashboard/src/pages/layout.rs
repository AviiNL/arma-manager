use api_schema::response::FilteredUser;
use leptos::{svg::Filter, *};
use leptos_router::*;

use crate::{api::AuthorizedApi, app::Theme, components::*};

use super::Page;

#[component]
pub fn Layout<F>(cx: Scope, on_logout: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    let Some(user_info) = use_context::<FilteredUser>(cx) else {
        unreachable!()
    };

    let Some(api) = use_context::<AuthorizedApi>(cx) else {
        unreachable!()
    };

    view! { cx,
        <>
            <div class="drawer drawer-mobile">
                <input id="left-sidebar-drawer" type="checkbox" class="drawer-toggle" />
                <PageContent on_logout />
                <LeftSidebar />
            </div>
        </>
    }
    .into_view(cx)
}
