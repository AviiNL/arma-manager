/*

*/

use derive_more::Display;
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

#[derive(Clone, Display, Debug, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    #[display(fmt = "default")]
    Default,
    #[display(fmt = "dark")]
    Dark,
    #[display(fmt = "light")]
    Light,
}

impl Theme {
    pub fn next(self) -> Self {
        match self {
            Self::Default => Self::Dark,
            Self::Dark => Self::Light,
            Self::Light => Self::Default,
        }
    }
}

#[component]
pub fn ThemeSelect(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("there to be an AppState");
    let theme = app_state.theme;
    let checked = move || theme.get() == Theme::Light;

    let on_input = move |_| {
        use gloo_storage::{LocalStorage, Storage};
        let next = theme.get_untracked().next();
        theme.set(next.clone());
        LocalStorage::set("theme", next).expect("LocalStorage::set");
    };

    let class = Signal::derive(cx, move || {
        let theme = theme.get();
        match theme {
            Theme::Default => "fa-solid fa-circle-half-stroke",
            Theme::Dark => "fa-solid fa-circle",
            Theme::Light => "fa-regular fa-circle",
        }
        .to_owned()
    });

    view! { cx,
        <label on:click=on_input class="btn btn-ghost btn-circle swap swap-rotate text-2xl">
            <i class={move || class.get()} title={move || format!("{:?}", theme.get())}></i>
        </label>
    }
}
