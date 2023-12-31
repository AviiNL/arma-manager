use crate::{
    api::{self, AuthorizedApi, UnauthorizedApi},
    app::{API_TOKEN_STORAGE_KEY, DEFAULT_API_URL},
    app_state::{AppState, Loading},
    components::*,
};
use api_schema::request::*;
use api_schema::response::*;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let authorized_api = app_state.api;
    let loading = app_state.loading;

    app_state.check_auth(cx);

    let api = UnauthorizedApi::new(loading);

    let (login_error, set_login_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);

    let login_action = create_action(cx, move |(email, password): &(String, String)| {
        tracing::debug!("Try to login with {email}");
        let email = email.to_string();
        let password = password.to_string();
        let credentials = LoginUserSchema { email, password };
        async move {
            loading.update(|l| *l = Loading::Loading(Some("Logging in...")));
            set_wait_for_response.update(|w| *w = true);
            let result = api.login(&credentials).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    set_login_error.update(|e| *e = None);
                    tracing::info!("Successfully logged in");
                    authorized_api.update(|v| *v = Some(res));
                }
                Err(err) => {
                    tracing::error!("Unable to login with {}: {err}", credentials.email);
                    app_state.toast(
                        cx,
                        format!("Unable to login with {}: {err}", credentials.email),
                        Some(ToastStyle::Error),
                    );
                    set_login_error.update(|e| *e = Some(err.to_string()));
                }
            }
        }
    });

    let disabled = Signal::derive(cx, move || wait_for_response.get());

    view! { cx,
        <LoginForm
            action=login_action
            error=login_error.into()
            disabled
        />
    }
}
