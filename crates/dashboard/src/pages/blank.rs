use api_schema::response::FilteredUser;
use leptos::*;
use leptos_router::*;

use crate::api::AuthorizedApi;

use super::Page;

#[component]
pub fn Blank(cx: Scope, title: &'static str) -> impl IntoView {
    view! { cx,
        <div class="hero h-4/5 bg-base-200">
            <div class="hero-content text-accent text-center">
                <div class="max-w-md">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" class="h-48 w-48 inline-block"><path d="M5.625 1.5c-1.036 0-1.875.84-1.875 1.875v17.25c0 1.035.84 1.875 1.875 1.875h12.75c1.035 0 1.875-.84 1.875-1.875V12.75A3.75 3.75 0 0016.5 9h-1.875a1.875 1.875 0 01-1.875-1.875V5.25A3.75 3.75 0 009 1.5H5.625z"></path><path d="M12.971 1.816A5.23 5.23 0 0114.25 5.25v1.875c0 .207.168.375.375.375H16.5a5.23 5.23 0 013.434 1.279 9.768 9.768 0 00-6.963-6.963z"></path></svg>
                    <h1 class="text-5xl mt-2 font-bold">{title}</h1>
                </div>
            </div>
        </div>
    }
}
