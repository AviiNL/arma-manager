use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use crate::{
    api::*,
    app_state::AppState,
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
pub const API_TOKEN_STORAGE_KEY: &str = "token";

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

pub async fn sleep(duration: Duration) {
    let (send, recv) = oneshot::channel();

    set_timeout(
        move || {
            let _ = send.send(());
        },
        duration,
    );

    recv.await;
}

pub type LogData = HashMap<String, Vec<String>>;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    provide_context(cx, AppState::new(cx));
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");

    let html_attributes = app_state.load_theme(cx);

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
                        view! { cx, <AuthenticatedBase /> }
                    }
                >
                    <Route path=Page::Dashboard.path()
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
                    <Route path=Page::Config.path()
                        view=move |cx| {
                            view! { cx, <Config /> }
                        }
                    />
                    <Route path=Page::Presets.path()
                        view=move |cx| {
                            view! { cx, <Presets /> }
                        }
                    />
                    <Route path=Page::Missions.path()
                        view=move |cx| {
                            view! { cx, <Blank title="Missions" /> }
                        }
                    />
                </Route>
                <Route
                    path=Page::Login.path()
                    view=move |cx| {
                        view! { cx,
                            <Login />
                        }
                    }
                />
                <Route
                    path=Page::Register.path()
                    view=move |cx| {
                        view! { cx, <Register /> }
                    }
                />
            </Routes>
        </Router>
        <Loading />
    }
}
