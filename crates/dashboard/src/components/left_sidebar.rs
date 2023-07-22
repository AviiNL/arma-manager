use leptos::*;
use leptos_router::*;

use crate::{components::*, pages::Page};

#[component]
pub fn LeftSidebar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="drawer-side">
            <label for="left-sidebar-drawer" class="drawer-overlay"></label>
            <ul class="menu w-80 bg-base-100 text-base-content">
                <label for="left-sidebar-drawer" class="btn btn-ghost bg-base-300  btn-circle z-50 top-0 right-0 mt-4 mr-2 absolute lg:hidden">
                    <i class="fa fa-times"/>
                </label>

                <li class="mb-2 font-semibold text-xl">
                    <a href={Page::Home.path()} class="gap-1">
                        <img class="mask mask-squircle w-12" src="/logo.png" alt="Logo"/>
                        "Arma"<div class="visible">"Server"</div><div class="lg:visible invisible">"Manager"</div>
                    </a>
                </li>

                <li>
                    <NavLink href={Page::Dashboard.path()} exact=true class="font-normal">
                        <i class="fa fa-home"/>
                        <s>
                            "Dashboard"
                        </s>
                    </NavLink>
                </li>
                <li>
                    <NavLink href={Page::Profile.path()} exact=true class="font-normal">
                        <i class="fa fa-user"/>
                        "Profile"
                    </NavLink>
                </li>
                <li>
                    <NavLink href={Page::Users.path()} exact=true class="font-normal">
                        <i class="fa fa-users"/>
                        "Users"
                    </NavLink>
                </li>

                <div class="divider mt-0 mb-0"></div>

                <li>
                    <NavLink href={Page::Logs.path()} exact=true class="font-normal">
                        <i class="fa fa-file-lines"/>
                        "Arma Logs"
                    </NavLink>
                </li>

                <div class="divider mt-0 mb-0"></div>

                <li>
                    <NavLink href={Page::Config.path()} exact=true class="font-normal">
                        <i class="fa fa-cogs"/>
                        "Configuration"
                    </NavLink>
                </li>

                <li>
                    <NavLink href={Page::Presets.path()} exact=true class="font-normal">
                        <i class="fa fa-cubes"/>
                        "Mod Presets"
                    </NavLink>
                </li>

                <li>
                    <NavLink href={Page::Missions.path()} exact=true class="font-normal">
                        <i class="fa-solid fa-tents"/>
                        "Missions"
                    </NavLink>
                </li>
            </ul>
        </div>
    }
}
