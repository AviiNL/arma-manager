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
    let loading = app_state.loading.clone();

    let api = UnauthorizedApi::new();
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
                loading.update(|l| *l = Loading::Loading(Some("Creating account...")));
                // verify if email is an actual email address, by finding the @ symbol followed by some dot somewhere after the @
                if !email.contains('@') {
                    set_register_error.update(|e| *e = Some("Invalid email address".to_string()));
                    return;
                }

                if password != repeat_password {
                    set_register_error.update(|e| *e = Some("Passwords do not match".to_string()));
                    return;
                }

                set_wait_for_response.update(|w| *w = true);
                let result = api.register(&credentials).await;
                set_wait_for_response.update(|w| *w = false);
                match result {
                    Ok(res) => {
                        set_register_response.update(|v| *v = Some(res));
                        set_register_error.update(|e| *e = None);
                    }
                    Err(err) => {
                        let msg = match err {
                            api::Error::Fetch(js_err) => {
                                format!("{js_err:?}")
                            }
                            api::Error::Api(err) => err.message,
                        };
                        tracing::warn!("Unable to register new account for {}: {msg}", credentials.email);
                        set_register_error.update(|e| *e = Some(msg));
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
            <p>"You have successfully registered. Please ask the administrator to activate your account, as we can't send emails yet ;)"</p>
            <p>"Once done, you can "<A href=super::Page::Login.path() class="hover:underline">"Login"</A>" with your new account."</p>
        </Show>
    }
}
