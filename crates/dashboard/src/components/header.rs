use api_schema::response::{FilteredUser, Status};
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;

use crate::{
    app::API_TOKEN_STORAGE_KEY,
    app_state::AppState,
    components::{ServerButtons, ThemeSelect},
    pages::Page,
};

trait Gravatar {
    fn gravatar(&self, size: u32) -> String;
}

impl Gravatar for FilteredUser {
    fn gravatar(&self, size: u32) -> String {
        let hash = md5::compute(self.email.as_bytes());
        format!("https://www.gravatar.com/avatar/{:?}?s={}", hash, size)
    }
}

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("to have found the app_state provided");
    let theme = app_state.theme;
    let user = app_state.user;
    let internal_user = create_rw_signal::<Option<FilteredUser>>(cx, None);

    create_effect(cx, move |_| {
        if let Some(user) = user.get() {
            internal_user.set(Some(user.clone()));
        } else {
            internal_user.set(None);
        }
    });

    let logout = create_action(cx, move |_| async move {
        let api = app_state.api.clone().get_untracked().expect("api to exist");
        match api.logout().await {
            Ok(_) => {
                app_state.cleanup();
                LocalStorage::delete(API_TOKEN_STORAGE_KEY);
                use_navigate(cx)(Page::Login.path(), Default::default()).expect("Login route");
            }
            Err(e) => {
                tracing::error!("Failed to logout: {:?}", e);
            }
        }

        // this is the only place we _can_ logout, so might as well keep this here.
    });

    view! { cx,
        <>
            <div class="navbar flex justify-between bg-base-100  z-10 shadow-md">
                <div class="gap-2">
                    <label for="left-sidebar-drawer" class="btn btn-primary drawer-button lg:hidden">
                        <i class="fa fa-bars" /> // hamburger menu icon
                    </label>
                    <ServerButtons />
                </div>

                <div class="order-last">

                    <ThemeSelect />

                    <div class="dropdown dropdown-end ml-4">
                        <label tabindex="0" class="btn btn-ghost btn-circle avatar">
                            <div class="w-10 rounded-full">
                                {move || if let Some(user) = internal_user.get() {
                                    view! { cx, <img src=move || user.gravatar(80) alt="profile" />}.into_view(cx)
                                } else {
                                    view! { cx, <img />}.into_view(cx)
                                }}
                            </div>
                        </label>
                        <ul tabindex="0" class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
                            <li class="justify-between">
                                <A href={Page::Profile.path()} exact=true>
                                    "Profile Settings"
                                </A>
                            </li>
                            <div class="divider mt-0 mb-0"></div>
                            <li>
                                <a href="#" on:click=move |_| logout.dispatch(())>"Logout"</a>
                            </li>
                        </ul>
                    </div>

                </div>
            </div>
        </>
    }
}
