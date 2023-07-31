use crate::{
    api::{self, UnauthorizedApi},
    app_state::{AppState, Loading},
    components::*,
};
use api_schema::{request::*, response::FilteredUser};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Register(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("AppState to exist");
    let loading = app_state.loading;

    let api = UnauthorizedApi::new(loading);
    let (register_response, set_register_response) = create_signal(cx, None::<FilteredUser>);
    let (register_error, set_register_error) = create_signal(cx, None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(cx, false);

    let register_action = create_action(
        cx,
        move |(name, email, password, repeat_password): &(String, String, String, String)| {
            let name = name.to_string();
            let email = email.to_string();
            let password = password.to_string();
            let repeat_password = repeat_password.to_string();

            let credentials = RegisterUserSchema {
                name: name.clone(),
                email: email.clone(),
                password: password.clone(),
            };
            tracing::info!("Try to register new account for {}", credentials.email);
            async move {
                loading.set(Loading::Loading(Some("Creating account...")));
                // verify if email is an actual email address, by finding the @ symbol followed by some dot somewhere after the @
                if !email.contains('@') {
                    set_register_error.update(|e| *e = Some("Invalid email address".to_string()));
                    loading.set(Loading::Ready);
                    return;
                }

                if password != repeat_password {
                    set_register_error.update(|e| *e = Some("Passwords do not match".to_string()));
                    loading.set(Loading::Ready);
                    return;
                }

                set_wait_for_response.update(|w| *w = true);
                let result = api.register(&credentials).await;
                set_wait_for_response.update(|w| *w = false);
                match result {
                    Ok(res) => {
                        set_register_response.update(|v| *v = Some(res));
                        set_register_error.update(|e| *e = None);
                        loading.set(Loading::Ready);
                        app_state.toast(cx, "Account created", Some(ToastStyle::Success));
                    }
                    Err(err) => {
                        tracing::warn!("Unable to register new account for {}: {err}", credentials.email);
                        loading.set(Loading::Ready);
                        set_register_error.update(|e| *e = Some(err.to_string()));
                    }
                }
            }
        },
    );

    let disabled = Signal::derive(cx, move || wait_for_response.get());

    view! { cx,
        <Show
            when=move || register_response.get().is_some()
            fallback=move |_| {
                view! { cx,
                    <RegisterForm
                        action=register_action
                        error=register_error.into()
                        disabled
                    />
                }
            }
        >
            <Redirect path="/" />
        </Show>
    }
}
