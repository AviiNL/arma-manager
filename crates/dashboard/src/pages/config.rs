use api_schema::response::FilteredUser;
use leptos::*;
use leptos_router::*;

use crate::{api::AuthorizedApi, components::*};

use super::Page;

#[component]
pub fn Config(cx: Scope) -> impl IntoView {
    let profile = create_rw_signal(cx, "server.cfg".to_string()); // default

    view! { cx,
        <div class="card w-full flex-1 p-6 bg-base-100 shadow-xl mt-2 mb-4">
            <div class="text-xl font-semibold inline-block">
                <div class="dropdown">
                    <label class="btn gap-1 normal-case btn-ghost" tabindex="0">
                        {move || profile.get()}
                        <i class="fa fa-caret-down"></i>
                    </label>
                    <ul tabindex="0" class="dropdown-content menu p-2 shadow-lg bg-base-100 rounded-box w-fit text-sm">
                        <li>
                            <div class="flex flex-1 grow items-center" onClick="document.activeElement.blur();" on:click=move |_| profile.set("server.cfg".to_string())>
                                <a href="#">"server.cfg"</a>
                            </div>
                        </li>
                        <li>
                        <div class="flex flex-1 grow items-center" onClick="document.activeElement.blur();" on:click=move |_| profile.set("profile.cfg".to_string())>
                            <a href="#">"profile.cfg"</a>
                        </div>
                        </li>
                    </ul>
                </div>
            </div>
            <div class="divider my-2"></div>
            <div class="h-full w-full grow bg-base-200 shadow-inner">
                <ClientOnly>
                <EditView channel=profile.into() />
                </ClientOnly>
            </div>
        </div>
    }
}
