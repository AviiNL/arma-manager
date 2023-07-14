use std::thread::sleep;

use api_schema::response::FilteredUser;
use leptos::*;
use leptos_router::*;

use crate::{api::AuthorizedApi, app::Theme, components::SteamCmdDialog, pages::layout::Layout};

use super::Page;

#[component]
pub fn Home<F>(cx: Scope, on_logout: F, user_info: Signal<Option<FilteredUser>>) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    // This is probably not the right way to do this, but it works.

    create_effect(cx, move |_| {
        let _ = user_info.get(); // subscribe?
        let timeout_handle = set_timeout_with_handle(
            move || {
                if user_info.get().is_none() {
                    let navigate = use_navigate(cx);
                    navigate(Page::Login.path(), Default::default());
                }
            },
            std::time::Duration::from_millis(1500),
        )
        .expect("timeouts to work");
    });

    view! { cx,
        {move || match user_info.get() {
            Some(info) => {
                provide_context(cx, info);

                view! { cx, <div id="root"><Layout on_logout=on_logout.clone() /></div> }.into_view(cx)
            }
            None => {
                view! { cx,
                    <p></p>
                }
                    .into_view(cx)
            }
        }}
    }
}
