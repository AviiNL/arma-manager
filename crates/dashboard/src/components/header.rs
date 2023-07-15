use api_schema::response::{FilteredUser, Status};
use leptos::*;
use leptos_router::*;

use crate::{app::Theme, components::ServerButtons};

trait Gravatar {
    fn gravatar(&self, size: u32) -> String;
}

impl Gravatar for FilteredUser {
    fn gravatar(&self, size: u32) -> String {
        let hash = md5::compute(self.email.as_bytes());
        format!("https://www.gravatar.com/avatar/{:?}?s={}", hash, size.to_string())
    }
}

#[component]
pub fn Header<F>(cx: Scope, on_logout: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    let user = use_context::<FilteredUser>(cx).expect("to have found the user provided");
    let theme = use_context::<WriteSignal<Theme>>(cx).expect("to have found the theme provided");
    let current_theme = use_context::<ReadSignal<Theme>>(cx).expect("to have found the theme provided");

    let checked = move || current_theme.get() == Theme::Light;

    let on_input = move |ev| {
        use gloo_storage::Storage;
        if event_target_checked(&ev) {
            theme.set(Theme::Light);
            gloo_storage::LocalStorage::set("theme", Theme::Light).expect("LocalStorage::set");
        } else {
            theme.set(Theme::Dark);
            gloo_storage::LocalStorage::set("theme", Theme::Dark).expect("LocalStorage::set");
        }
    };

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

                    <div class="dropdown dropdown-end">
                        <label class="btn btn-ghost btn-circle swap swap-rotate text-2xl">
                            <input type="checkbox" prop:checked=checked on:input=on_input/>
                            <i class="fa-regular fa-sun swap-on"></i>
                            <i class="fa-regular fa-moon swap-off"></i>
                        </label>
                    </div>

                    <div class="dropdown dropdown-end ml-4">
                        <label tabindex="0" class="btn btn-ghost btn-circle avatar">
                            <div class="w-10 rounded-full">
                            <img src=move || user.gravatar(80) alt="profile" />
                            </div>
                        </label>
                        <ul tabindex="0" class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
                            <li class="justify-between">
                                <A href="/profile" exact=true>
                                    "Profile Settings"
                                </A>
                            </li>
                            <div class="divider mt-0 mb-0"></div>
                            <li>
                                <a href="#" on:click={
                                    let on_logout = on_logout.clone();
                                    move |_| on_logout()
                                }>"Logout"</a>
                            </li>
                        </ul>
                    </div>

                </div>
            </div>
        </>
    }
}
