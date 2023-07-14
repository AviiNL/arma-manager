use crate::{
    api::{self, AuthorizedApi, UnauthorizedApi},
    components::*,
};
use api_schema::request::*;
use api_schema::response::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Login<F>(cx: Scope, api: UnauthorizedApi, on_success: F) -> impl IntoView
where
    F: Fn(AuthorizedApi) + 'static + Clone,
{
    let (login_error, set_login_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);

    let login_action = create_action(cx, move |(email, password): &(String, String)| {
        tracing::debug!("Try to login with {email}");
        let email = email.to_string();
        let password = password.to_string();
        let credentials = LoginUserSchema { email, password };
        let on_success = on_success.clone();
        async move {
            set_wait_for_response.update(|w| *w = true);
            let result = api.login(&credentials).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    set_login_error.update(|e| *e = None);
                    on_success(res);
                }
                Err(err) => {
                    let msg = match err {
                        api::Error::Fetch(js_err) => {
                            format!("{js_err:?}")
                        }
                        api::Error::Api(err) => err.message,
                    };
                    error!("Unable to login with {}: {msg}", credentials.email);
                    set_login_error.update(|e| *e = Some(msg));
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
