use std::collections::HashMap;

use api_schema::{request::*, response::*};
use derive_more::Display;
use gloo_storage::{LocalStorage, Storage};
use http::status;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::{
    api::AuthorizedApi,
    app::{API_TOKEN_STORAGE_KEY, DEFAULT_API_URL},
    sse::create_sse,
};

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

#[derive(Clone, Copy)]
pub struct AppState {
    pub theme: RwSignal<Theme>,
    pub loading: RwSignal<Loading>,
    pub user: RwSignal<Option<FilteredUser>>,
    pub api: RwSignal<Option<AuthorizedApi>>,
    pub status: RwSignal<Option<Status>>,
    pub log: RwSignal<LogData>,
}

impl AppState {
    pub fn new(cx: Scope) -> Self {
        Self {
            theme: create_rw_signal(cx, Theme::Default),
            loading: create_rw_signal(cx, Loading::Loading(Some("Initializing, Please stand by..."))),
            user: create_rw_signal(cx, None),
            api: create_rw_signal(cx, None),
            status: create_rw_signal(cx, None),
            log: create_rw_signal(cx, Default::default()),
        }
    }

    pub fn cleanup(&self) {
        if let Some(api) = self.api.get_untracked() {
            api.run_abort_signals();
            self.api.set(None);
            self.user.set(None);
            self.status.set(None);
            self.log.set(Default::default());
        }
    }

    pub fn check_auth(&self, cx: Scope) {
        if self.api.get_untracked().is_some() {
            return;
        }

        let api_signal = self.api.clone();
        let loading_signal = self.loading.clone();

        let user_signal = self.user.clone();
        let status_signal = self.status.clone();
        let log_signal = self.log.clone();

        create_effect(cx, move |_| {
            if let Some(api) = api_signal.get() {
                let navigate = use_navigate(cx);
                let route = use_route(cx);

                LocalStorage::set(API_TOKEN_STORAGE_KEY, api.token());

                spawn_local(async move {
                    let api = api_signal.get_untracked().expect("api to exist");
                    // Fetch user
                    if !set_user(&api, &user_signal).await {
                        api_signal.set(None);
                        loading_signal.set(Loading::Ready);

                        if route.path() != crate::pages::Page::Login.path().trim_start_matches("/") {
                            navigate(crate::pages::Page::Login.path(), Default::default()).expect("Login route");
                        }

                        return;
                    }

                    // deffo confirmed signed in at this point, so we can load everything else in parallel
                    set_status(cx, &api, &status_signal).await;
                    setup_logs(cx, &api, &log_signal).await;

                    // only do this if we are on Login page
                    if route.path() == crate::pages::Page::Login.path().trim_start_matches("/") {
                        tracing::info!("Redirecting to /console");
                        navigate(crate::pages::Page::Home.path(), Default::default()).expect("Home route");
                    }
                });
            }
        });

        create_effect(cx, move |_| {
            if let Ok(token) = LocalStorage::get(API_TOKEN_STORAGE_KEY) {
                let api = AuthorizedApi::new(DEFAULT_API_URL, token);
                api_signal.set(Some(api));
            } else {
                // not logged in, stop loading
                loading_signal.update(|l| *l = Loading::Ready);
            }
        });
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

async fn set_user(api: &AuthorizedApi, user_signal: &RwSignal<Option<FilteredUser>>) -> bool {
    match api.user_info().await {
        Ok(user_info) => {
            user_signal.set(Some(user_info));
            true
        }
        Err(e) => {
            LocalStorage::delete(API_TOKEN_STORAGE_KEY);
            tracing::error!("Failed to fetch user info: {:?}", e);
            false
        }
    }
}

async fn set_status(cx: Scope, api: &AuthorizedApi, status_signal: &RwSignal<Option<Status>>) {
    let api = api.clone();
    let status_signal = status_signal.clone();
    match api.last_status().await {
        Ok(status) => {
            status_signal.set(Some(status));
        }
        Err(e) => {
            tracing::error!("Failed to fetch status: {:?}", e);
        }
    }

    // also start the sse for status here? still need to make an abstraction for it though
    let abort_signal = create_sse(cx, "status", vec!["message".to_string()], move |_, data| {
        status_signal.set(data);
    });

    api.add_abort_signal(abort_signal);
}

async fn setup_logs(cx: Scope, api: &AuthorizedApi, log_signal: &RwSignal<LogData>) {
    let api = api.clone();

    // grab steamcmd latest log
    if let Ok(new_data) = api.get_log("arma").await {
        log_signal.update(|l| {
            if !l.contains_key("arma") {
                l.insert("arma".into(), vec![]);
            }
            l.get_mut("arma").unwrap().extend(new_data.log.clone())
        });
    }

    let log_signal = log_signal.clone();
    let abort_signal = create_sse(
        cx,
        "logs",
        vec!["steamcmd".to_string(), "arma".to_string()],
        move |channel, data: Vec<String>| {
            log_signal.update(|l| {
                if !l.contains_key(&channel) {
                    l.insert(channel.clone(), vec![]);
                }
                for line in data.iter() {
                    l.get_mut(&channel).unwrap().push(line.clone());
                }
            });
        },
    );

    api.add_abort_signal(abort_signal);
}
