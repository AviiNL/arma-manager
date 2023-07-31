use std::collections::HashMap;

use api_schema::a2s::{Info, Player, PlayerOrInfo};
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
    components::{Theme, Toast, ToastStyle},
    sse::create_sse,
};

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Loading {
    Loading(Option<&'static str>),
    Error(Option<&'static str>),
    Ready,
}

pub type LogData = HashMap<String, Vec<String>>;
pub type ConfigData = HashMap<String, Vec<String>>;
pub type MissionData = Vec<String>;

pub type PresetList = Vec<Preset>;

#[derive(Clone, Copy)]
pub struct AppState {
    pub theme: RwSignal<Theme>,
    pub loading: RwSignal<Loading>,
    pub user: RwSignal<Option<FilteredUser>>,
    pub api: RwSignal<Option<AuthorizedApi>>,
    pub status: RwSignal<Option<Status>>,
    pub log: RwSignal<LogData>,
    pub players: RwSignal<Vec<Player>>,
    pub server_info: RwSignal<Option<Info>>,
    pub presets: RwSignal<PresetList>,
    pub config: RwSignal<ConfigData>,
    pub missions: RwSignal<MissionData>,
    pub toasts: RwSignal<Vec<Toast>>,
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
            players: create_rw_signal(cx, Default::default()),
            server_info: create_rw_signal(cx, Default::default()),
            presets: create_rw_signal(cx, Default::default()),
            config: create_rw_signal(cx, Default::default()),
            missions: create_rw_signal(cx, Default::default()),
            toasts: create_rw_signal(cx, Default::default()),
        }
    }

    pub fn cleanup(&self) {
        if let Some(api) = self.api.get_untracked() {
            api.run_abort_signals();
            self.api.set(None);
            self.user.set(None);
            self.status.set(None);
            self.log.set(Default::default());
            self.players.set(Default::default());
            self.server_info.set(Default::default());
            self.presets.set(Default::default());
            self.config.set(Default::default());
            self.missions.set(Default::default());
        }
    }

    pub fn toast(&self, cx: Scope, message: impl Into<String>, style: Option<ToastStyle>) {
        let message = message.into();
        let toast = Toast::new(cx, message, style);
        self.toasts.update(|t| {
            // push the new toast to the front of the vec
            t.insert(0, toast);
        });
    }

    pub fn check_auth(&self, cx: Scope) {
        if self.api.get_untracked().is_some() {
            return;
        }

        let api_signal = self.api;
        let loading_signal = self.loading;

        let user_signal = self.user;
        let status_signal = self.status;
        let log_signal = self.log;
        let info_signal = self.server_info;
        let players_signal = self.players;
        let preset_signal = self.presets;
        let config_signal = self.config;
        let mission_signal = self.missions;

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

                        if route.path() != crate::pages::Page::Login.path().trim_start_matches('/') {
                            navigate(crate::pages::Page::Login.path(), Default::default()).expect("Login route");
                        }

                        return;
                    }

                    // deffo confirmed signed in at this point, so we can load everything else in parallel
                    set_status(cx, &api, &status_signal).await;
                    setup_logs(cx, &api, &log_signal).await;
                    setup_a2s(cx, &api, &info_signal, &players_signal).await;
                    setup_presets(cx, &api, &preset_signal, &status_signal, &loading_signal).await;
                    setup_config(cx, &api, &config_signal).await;
                    setup_missions(cx, &api, &mission_signal).await;

                    // only do this if we are on Login page
                    if route.path() == crate::pages::Page::Login.path().trim_start_matches('/') {
                        tracing::info!("Redirecting to {}", crate::pages::Page::Home.path());
                        navigate(crate::pages::Page::Home.path(), Default::default()).expect("Home route");
                    }
                });
            }
        });

        create_effect(cx, move |_| {
            if let Ok(token) = LocalStorage::get(API_TOKEN_STORAGE_KEY) {
                let api = AuthorizedApi::new(DEFAULT_API_URL, token, loading_signal);
                api_signal.set(Some(api));
            } else {
                // not logged in, stop loading
                loading_signal.update(|l| *l = Loading::Ready);
            }
        });
    }

    pub fn load_theme(&self, cx: Scope) -> RwSignal<AdditionalAttributes> {
        let theme = self.theme;
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
    let status_signal = *status_signal;

    if let Ok(status) = api.last_status().await {
        status_signal.set(Some(status));
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

    if let Ok(new_data) = api.get_log("steamcmd").await {
        log_signal.update(|l| {
            if !l.contains_key("steamcmd") {
                l.insert("steamcmd".into(), vec![]);
            }
            l.get_mut("steamcmd").unwrap().extend(new_data.log.clone())
        });
    }

    let log_signal = *log_signal;
    let abort_signal = create_sse(
        cx,
        "logs",
        vec!["steamcmd".to_string(), "arma".to_string()],
        move |channel, data: Vec<String>| {
            log_signal.update(|l| {
                if !l.contains_key(&channel) {
                    tracing::error!("Log for {} not found!", channel);
                    return;
                }
                for line in data.iter() {
                    l.get_mut(&channel).unwrap().push(line.clone());
                }
            });
        },
    );

    api.add_abort_signal(abort_signal);
}

async fn setup_a2s(
    cx: Scope,
    api: &AuthorizedApi,
    info_signal: &RwSignal<Option<Info>>,
    players_signal: &RwSignal<Vec<Player>>,
) {
    let aapi = api.clone();

    let players_signal = *players_signal;
    let info_signal = *info_signal;

    if let Ok(new_data) = api.get_a2s_info().await {
        info_signal.set(Some(new_data));
    }

    if let Ok(new_data) = api.get_a2s_players().await {
        players_signal.set(new_data);
    }

    let abort_signal = create_sse(
        cx,
        "a2s",
        vec!["info".to_string(), "players".to_string()],
        move |channel, data| match channel.as_str() {
            "info" => {
                let PlayerOrInfo::Info(info) = data else {
                    // invalid type, ignore
                    return;
                };

                info_signal.set(Some(*info));
            }
            "players" => {
                let PlayerOrInfo::Players(players) = data else {
                    // invalid type, ignore
                    return;
                };

                players_signal.set(players);
            }
            _ => {}
        },
    );
    api.add_abort_signal(abort_signal);
}

async fn setup_presets(
    cx: Scope,
    api: &AuthorizedApi,
    preset_signal: &RwSignal<PresetList>,
    status: &RwSignal<Option<Status>>,
    loading: &RwSignal<Loading>,
) {
    let aapi = api.clone();
    let status = *status;

    let presets = *preset_signal;
    create_resource(
        cx,
        move || status.get(),
        move |status| {
            let api = aapi.clone();
            async move {
                if let Some(state) = status {
                    if state.steamcmd != api_schema::response::State::Stopped {
                        return;
                    }
                }
                let new_presets = api.get_presets().await.unwrap();
                presets.set(new_presets);
            }
        },
    );

    let presets = *preset_signal;
    let loading = *loading;
    let abort_signal = create_sse(
        cx,
        "presets",
        vec![
            "create".to_string(),
            "select".to_string(),
            "update".to_string(),
            "update_dlc".to_string(),
            "blacklist".to_string(),
            "unblacklist".to_string(),
            "delete".to_string(),
        ],
        move |event, data: PresetUpdate| match event.as_str() {
            "create" => {
                let PresetUpdate::Created(preset) = data else {
                    tracing::error!("Incompatible data for event: {}", event);
                    return;
                };

                presets.update(|list| {
                    let mut found = false;
                    list.iter_mut().for_each(|p| {
                        if p.id == preset.id {
                            *p = preset.clone();
                            found = true;
                        }
                    });

                    if !found {
                        list.push(preset);
                    }
                });
            }
            "select" => {
                let PresetUpdate::Selected(id) = data else {
                    tracing::error!("Incompatible data for event: {}", event);
                    return;
                };

                presets.update(|list| {
                    list.iter_mut().for_each(|preset| {
                        preset.selected = false;
                        if preset.id == id {
                            preset.selected = true;
                        }
                    });
                });
            }
            "update" => {
                let PresetUpdate::Updated(item) = data else {
                    tracing::error!("Incompatible data for event: {}", event);
                    return;
                };

                // ew?
                loading.set(Loading::Loading(Some("Updating Presets...")));
                presets.update(|list| {
                    list.iter_mut().for_each(|p| {
                        p.items.iter_mut().for_each(|i| {
                            if i.id == item.id {
                                *i = item.clone();
                            }
                        });
                    });
                });
                loading.set(Loading::Ready);
            }
            "update_dlc" => {
                let PresetUpdate::Dlc(item) = data else {
                    tracing::error!("Incompatible data for event: {}", event);
                    return;
                };

                // ew?
                loading.set(Loading::Loading(Some("Updating Presets...")));
                presets.update(|list| {
                    list.iter_mut().for_each(|p| {
                        p.dlcs.iter_mut().for_each(|i| {
                            if i.id == item.id {
                                *i = item.clone();
                            }
                        });
                    });
                });
                loading.set(Loading::Ready);
            }
            "blacklist" => {
                let PresetUpdate::Blacklisted(published_file_id) = data else {
                    tracing::error!("Incompatible data for event: {}", event);
                    return;
                };

                presets.update(|list| {
                    list.iter_mut().for_each(|p| {
                        p.items.iter_mut().for_each(|i| {
                            if i.published_file_id == published_file_id {
                                i.blacklisted = true;
                            }
                        });
                    });
                });
            }
            "unblacklist" => {
                let PresetUpdate::Unblacklisted(published_file_id) = data else {
                    tracing::error!("Incompatible data for event: {}", event);
                    return;
                };

                presets.update(|list| {
                    list.iter_mut().for_each(|p| {
                        p.items.iter_mut().for_each(|i| {
                            if i.published_file_id == published_file_id {
                                i.blacklisted = false;
                            }
                        });
                    });
                });
            }
            "delete" => {
                let PresetUpdate::Delete(id) = data else {
                    tracing::error!("Incompatible data for event: {}", event);
                    return;
                };

                presets.update(|list| {
                    list.retain(|p| p.id != id);
                });
            }
            _ => tracing::error!("Unknown preset event: {}", event),
        },
    );
    api.add_abort_signal(abort_signal);
}

async fn setup_config(cx: Scope, api: &AuthorizedApi, config_signal: &RwSignal<ConfigData>) {
    let api = api.clone();
    let config_signal = *config_signal;

    if let Ok(new_data) = api.get_config("server.cfg").await {
        config_signal.update(|l| {
            l.insert("server.cfg".into(), vec![]);
            l.get_mut("server.cfg").unwrap().extend(new_data.config.clone())
        });
    }

    if let Ok(new_data) = api.get_config("profile.cfg").await {
        config_signal.update(|l| {
            l.insert("profile.cfg".into(), vec![]);
            l.get_mut("profile.cfg").unwrap().extend(new_data.config.clone())
        });
    }

    let abort_signal = create_sse(
        cx,
        "arma/config",
        vec!["server.cfg".to_string(), "profile.cfg".to_string()],
        move |channel, data: Vec<String>| {
            config_signal.update(|l| {
                l.insert(channel.clone(), vec![]); // clear it first
                for line in data.iter() {
                    l.get_mut(&channel).unwrap().push(line.clone());
                }
            });
        },
    );

    api.add_abort_signal(abort_signal);
}

async fn setup_missions(cx: Scope, api: &AuthorizedApi, mission_signal: &RwSignal<MissionData>) {
    let api = api.clone();
    let mission_signal = mission_signal;

    if let Ok(new_data) = api.get_missions().await {
        mission_signal.set(new_data.missions);
    }
}
