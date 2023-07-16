use std::collections::HashMap;

use api_schema::{request::*, response::*};
use derive_more::Display;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::{api::AuthorizedApi, app::API_TOKEN_STORAGE_KEY};

#[derive(Clone, Display, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    #[display(fmt = "default")]
    Default,
    #[display(fmt = "dark")]
    Dark,
    #[display(fmt = "light")]
    Light,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Loading {
    Loading(Option<&'static str>),
    Error(Option<&'static str>),
    Ready,
}

pub type LogData = HashMap<String, Vec<String>>;

#[derive(Clone)]
pub struct AppState {
    pub theme: RwSignal<Theme>,
    pub loading: RwSignal<Loading>,
    pub user: RwSignal<Option<FilteredUser>>,
    pub api: RwSignal<Option<AuthorizedApi>>,
    pub status: RwSignal<Option<Status>>,
    pub log: RwSignal<Option<LogData>>,
}

impl AppState {
    pub fn new(cx: Scope) -> Self {
        Self {
            theme: create_rw_signal(cx, Theme::Default),
            loading: create_rw_signal(cx, Loading::Loading(Some("Initializing, Please stand by..."))),
            user: create_rw_signal(cx, None),
            api: create_rw_signal(cx, None),
            status: create_rw_signal(cx, None),
            log: create_rw_signal(cx, None),
        }
    }

    pub fn listen_for_login(&self, cx: Scope) {
        let api = self.api.clone();
        let user = self.user.clone();
        create_effect(cx, move |_| async move {
            let api = api.get(); // track
            if api.is_some() {
                // let api = api.expect("api to no longer be none");
                // this should be tracked?
                tracing::info!("Api updated, fetching user info");
                LocalStorage::set(API_TOKEN_STORAGE_KEY, api.token()).expect("LocalStorage::set");

                // Load User Info
                // match api.user_info().await {
                //     Ok(user_info) => user.set(Some(user_info)),
                //     Err(_) => {
                //         // token is invalid
                //         // delete the key
                //         LocalStorage::delete(API_TOKEN_STORAGE_KEY);
                //         user.set(None);
                //         return;
                //     }
                // }

                let navigate = use_navigate(cx);
                navigate(crate::pages::Page::Home.path(), Default::default()).expect("Home route");
            }
        })
    }

    pub fn load_theme(&self, cx: Scope) -> RwSignal<AdditionalAttributes> {
        let theme = self.theme.clone();
        create_effect(cx, move |_| {
            let set_theme = match LocalStorage::get("theme") {
                Ok(theme) => theme,
                Err(e) => Theme::Default,
            };
            theme.set(set_theme);
        });

        create_rw_signal::<AdditionalAttributes>(cx, vec![("data-theme", move || theme.get().to_string())].into())
    }
}
