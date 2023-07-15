use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use crate::{
    api::*,
    components::*,
    pages::{log::Log, *},
};
use api_schema::response::{ApiToken, FilteredUser, Status};
use derive_more::*;
use futures::{channel::oneshot, stream, StreamExt};
use gloo_net::eventsource::futures::EventSource;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub const DEFAULT_API_URL: &str = "/api/v1";
pub const DEFAULT_SSE_URL: &str = "/sse/v1";
const API_TOKEN_STORAGE_KEY: &str = "token";

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

async fn sleep(duration: Duration) {
    let (send, recv) = oneshot::channel();

    set_timeout(
        move || {
            let _ = send.send(());
        },
        duration,
    );

    recv.await;
}

// #[derive(Clone)]
// pub struct ArmaLog(pub String);
// impl AsRef<str> for ArmaLog {
//     fn as_ref(&self) -> &str {
//         &self.0
//     }
// }

// impl From<String> for ArmaLog {
//     fn from(s: String) -> Self {
//         Self(s)
//     }
// }

// #[derive(Clone)]
// pub struct SteamcmdLog(pub String);
// impl AsRef<str> for SteamcmdLog {
//     fn as_ref(&self) -> &str {
//         &self.0
//     }
// }

// impl From<String> for SteamcmdLog {
//     fn from(s: String) -> Self {
//         Self(s)
//     }
// }

pub type LogData = HashMap<String, Vec<String>>;

#[derive(Clone, Display, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    #[display(fmt = "default")]
    Default,
    #[display(fmt = "dark")]
    Dark,
    #[display(fmt = "light")]
    Light,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // -- signals -- //

    let authorized_api = create_rw_signal(cx, None::<AuthorizedApi>);
    let user_info = create_rw_signal(cx, None::<FilteredUser>);
    let status = create_rw_signal(cx, None::<Status>);

    provide_context(cx, authorized_api);
    provide_context(cx, status);

    // -- actions -- //

    let fetch_user_info = create_action(cx, move |_| async move {
        match authorized_api.get_untracked() {
            Some(api) => match api.user_info().await {
                Ok(info) => {
                    provide_context(cx, api.clone());
                    user_info.update(|i| *i = Some(info));
                }
                Err(err) => {
                    tracing::error!("Unable to fetch user info: {err}");
                }
            },
            None => {
                tracing::error!("Unable to fetch user info: not logged in");
            }
        }
    });

    let logout = create_action(cx, move |_| async move {
        // this is also outside a reactive tracking context for some reason
        match authorized_api.get() {
            Some(api) => match api.logout().await {
                Ok(_) => {
                    authorized_api.update(|a| *a = None);
                    user_info.update(|i| *i = None);
                }
                Err(err) => {
                    tracing::error!("Unable to logout: {err}");
                }
            },
            None => {
                tracing::error!("Unable to logout user: not logged in");
            }
        }
    });

    let fetch_status = create_action(cx, move |_| async move {
        match authorized_api.get_untracked() {
            Some(api) => match api.last_status().await {
                Ok(s) => {
                    status.set(Some(s));
                }
                Err(err) => {
                    tracing::error!("Unable to fetch status: {err}");
                }
            },
            None => {}
        }
    });

    // Channel, Log lines
    let base_logs = create_rw_signal(cx, LogData::default());
    provide_context(cx, base_logs);

    let fetch_logs = create_action(cx, move |_| async move {
        match authorized_api.get_untracked() {
            Some(api) => {
                let api = api.clone();
                if let Ok(new_data) = api.get_log("arma").await {
                    base_logs.update(|l| {
                        if !l.contains_key("arma") {
                            l.insert("arma".into(), vec![]);
                        }
                        l.get_mut("arma").unwrap().extend(new_data.log.clone())
                    });
                }

                if let Ok(new_data) = api.get_log("steamcmd").await {
                    base_logs.update(|l| {
                        if !l.contains_key("steamcmd") {
                            l.insert("steamcmd".into(), vec![]);
                        }
                        l.get_mut("steamcmd").unwrap().extend(new_data.log.clone())
                    });
                }

                let uri = format!("{}/logs?token={}", DEFAULT_SSE_URL, api.token().token);
                let mut event_source = EventSource::new(&uri).expect("EventSource::new");

                loop {
                    let state = event_source.state();
                    tracing::info!("Waiting for eventsource to connect: {}", (state as u16));
                    if (state as u16) == 1 {
                        break;
                    }
                    sleep(Duration::from_millis(100)).await;
                }

                let mut steamcmd_stream = event_source.subscribe("steamcmd").unwrap();
                let mut arma_stream = event_source.subscribe("arma").unwrap();

                let base_logs = base_logs.clone();
                spawn_local(async move {
                    let _ = event_source.state();
                    let mut all_streams = stream::select(steamcmd_stream, arma_stream);
                    tracing::info!("Starting eventsource");
                    while let Some(Ok((event_type, msg))) = all_streams.next().await {
                        let new_data = msg.data().as_string().unwrap();
                        let new_data = serde_json::from_str::<Vec<String>>(&new_data).unwrap();
                        base_logs.update(|l| {
                            if !l.contains_key(&event_type) {
                                l.insert(event_type.clone().into(), vec![]);
                            }
                            l.get_mut(&event_type).unwrap().extend(new_data.clone())
                        });
                    }
                    tracing::info!("Ending eventsource");
                });
            }
            None => {}
        }
    });

    // -- theme management -- //

    let (theme, set_theme) = create_signal(cx, Theme::Default);
    provide_context(cx, theme);
    provide_context(cx, set_theme);

    create_effect(cx, move |_| {
        let theme = match LocalStorage::get("theme") {
            Ok(theme) => theme,
            Err(e) => Theme::Dark,
        };
        set_theme.set(theme);
    });

    let html_attributes =
        create_rw_signal::<AdditionalAttributes>(cx, vec![("data-theme", move || theme.get().to_string())].into());

    // -- callbacks -- //

    let on_logout = move || {
        logout.dispatch(());
    };

    // -- init API -- //

    let unauthorized_api = UnauthorizedApi::new(DEFAULT_API_URL);

    create_effect(cx, move |_| {
        if let Ok(token) = LocalStorage::get(API_TOKEN_STORAGE_KEY) {
            let api = AuthorizedApi::new(DEFAULT_API_URL, token);
            authorized_api.update(|a| *a = Some(api));
            fetch_status.dispatch(());
            fetch_user_info.dispatch(());
            fetch_logs.dispatch(());
        }
    });

    // -- status -- //

    create_effect(cx, move |_| {
        match authorized_api.get() {
            Some(api) => {
                let url = format!("{}/status?token={}", DEFAULT_SSE_URL, api.token().token);

                let mut event_source = EventSource::new(&url).expect("EventSource::new");
                let mut stream = event_source.subscribe("message").unwrap();

                spawn_local(async move {
                    let _ = event_source.state(); // this blocks until connected?
                    while let Some(Ok((event_type, msg))) = stream.next().await {
                        if authorized_api.get_untracked().is_none() {
                            break;
                        }

                        status.set(Some(serde_json::from_str(&msg.data().as_string().unwrap()).unwrap()));
                    }
                });
            }
            None => {}
        }
    });

    // -- effects -- //

    create_effect(cx, move |_| {
        tracing::debug!("API authorization state changed");
        match authorized_api.get() {
            Some(api) => {
                tracing::debug!("API is now authorized: save token in LocalStorage");
                LocalStorage::set(API_TOKEN_STORAGE_KEY, api.token()).expect("LocalStorage::set");
            }
            None => {
                tracing::debug!(
                    "API is no longer authorized: delete token from \
                     LocalStorage"
                );
                LocalStorage::delete(API_TOKEN_STORAGE_KEY);
            }
        }
    });

    provide_meta_context(cx);
    view! { cx,
        <Title text="Arma Manager"/>
        <Stylesheet id="leptos" href="/pkg/arma-manager.css"/>
        <Link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" integrity="sha512-iecdLmaskl7CVkqkXNQ/ZH/XLlvWZOJyj7Yy7tcenmpD1ypASozpmT/E0iPtmFIB46ZmdtAc9eNBvH0H/ZpiBw==" crossorigin="anonymous" referrerpolicy="no-referrer" />

        <Html attributes=html_attributes />

        <Router>
            <Routes>
                <Route
                    path=Page::Home.path()
                    view=move |cx| {
                        view! { cx, <Home on_logout=on_logout user_info=user_info.into()/> }
                    }
                >
                    <Route path=Page::Home.path()
                        view=move |cx| {
                            view! { cx, <Blank title="Dashboard" /> }
                        }
                    />
                    <Route path=Page::Profile.path()
                        view=move |cx| {
                            view! { cx, <Blank title="Profile" /> }
                        }
                    />
                    <Route path=Page::Logs.path()
                        view=move |cx| {
                            view! { cx, <Log /> }
                        }
                    />
                    <Route path=Page::Mods.path()
                        view=move |cx| {
                            view! { cx, <Blank title="Mod Presets" /> }
                        }
                    />
                </Route>
                <Route
                    path=Page::Login.path()
                    view=move |cx| {
                        view! { cx,
                            <Login
                                api=unauthorized_api
                                on_success=move |api| {
                                    tracing::info!("Successfully logged in");
                                    authorized_api.update(|v| *v = Some(api));
                                    let navigate = use_navigate(cx);
                                    navigate(Page::Home.path(), Default::default()).expect("Home route");
                                    fetch_user_info.dispatch(());
                                }
                            />
                        }
                    }
                />
                <Route
                    path=Page::Register.path()
                    view=move |cx| {
                        view! { cx, <Register api=unauthorized_api/> }
                    }
                />
            </Routes>
        </Router>

    }
}
