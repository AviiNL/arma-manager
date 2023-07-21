use api_schema::{request::*, response::*};
use leptos::*;

use crate::{app_state::AppState, components::ToastStyle};

#[component]
pub fn Profile(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("there to be an AppState");
    let user = app_state.user.clone();

    let name_signal = create_rw_signal(cx, String::default());
    let email_signal = create_rw_signal(cx, String::default());
    let password_signal = create_rw_signal(cx, String::default());
    let confirm_password_signal = create_rw_signal(cx, String::default());

    let changed = Signal::derive(cx, move || {
        let name = name_signal.get();
        let email = email_signal.get();
        let password = password_signal.get();
        let confirm_password = confirm_password_signal.get();
        if let Some(user) = user.get() {
            return user.name != name || user.email != email || !password.is_empty() || !confirm_password.is_empty();
        }
        return false;
    });

    create_effect(cx, move |_| {
        if let Some(user) = user.get() {
            name_signal.set(user.name.clone());
            email_signal.set(user.email.clone());
        }
    });

    let tokens = Signal::derive(cx, move || match user.get() {
        None => return vec![],
        Some(user) => {
            let mut tokens = user.tokens.clone();
            // sort by last_used DESC
            tokens.sort_by(|a, b| b.last_used.cmp(&a.last_used));
            return tokens;
        }
    });

    let update = create_action(cx, move |()| {
        let name = name_signal.get_untracked();
        let email = email_signal.get_untracked();
        let password = password_signal.get_untracked();
        let confirm_password = confirm_password_signal.get_untracked();
        async move {
            if !password.is_empty() && password != confirm_password {
                app_state.toast(cx, "Passwords do not match", Some(ToastStyle::Error));
                return;
            }

            let api = app_state.api.get_untracked().expect("api to exist");
            let new_user = api
                .update_user(&UpdateUserSchema {
                    name: name,
                    email: email,
                    password: if password.is_empty() { None } else { Some(password) },
                })
                .await;
            match new_user {
                Ok(new_user) => {
                    user.set(Some(new_user));
                    password_signal.set(String::default());
                    confirm_password_signal.set(String::default());
                }
                Err(err) => {
                    app_state.toast(cx, format!("Unable to update user: {}", err), Some(ToastStyle::Error));
                }
            }
        }
    });

    view! { cx,
        <div class="card w-full md:w-4/6 p-6 bg-base-100 shadow-xl mr-1 mt-2 mb-4">
            <div class="text-xl font-semibold ">"Profile Settings"</div>
            <div class="divider mt-2"></div>
            <div class="max-w-full h-full overflow-y-auto">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="form-control w-full undefined">
                        <label class="label">
                            <span class="label-text text-base-content undefined">"Name"</span>
                        </label>
                        <input
                            type="text"
                            class="input  input-bordered w-full"
                            on:input={move |ev| name_signal.set(event_target_value(&ev))}
                            prop:value=move || name_signal.get()
                        />
                    </div>
                    <div class="form-control w-full undefined">
                        <label class="label">
                            <span class="label-text text-base-content undefined">"Email"</span>
                        </label>
                        <input
                            type="text"
                            class="input input-bordered w-full"
                            on:input={move |ev| email_signal.set(event_target_value(&ev))}
                            prop:value=move || email_signal.get()
                        />
                    </div>
                    <div class="form-control w-full undefined">
                        <label class="label">
                            <span class="label-text text-base-content undefined">"Change Password"</span>
                        </label>
                        <input
                            type="password"
                            class="input input-bordered w-full"
                            autocomplete="new-password"
                            on:input={move |ev| password_signal.set(event_target_value(&ev))}
                        />
                    </div>
                    <div class="form-control w-full undefined">
                        <label class="label">
                            <span class="label-text text-base-content undefined">"Confirm Change Password"</span>
                        </label>
                        <input
                            type="password"
                            class="input input-bordered w-full"
                            autocomplete="new-password"
                            on:input={move |ev| confirm_password_signal.set(event_target_value(&ev))}
                        />
                    </div>
                </div>
                <div class="mt-16">

                    <div class="indicator float-right">
                        <Show when={move || changed.get()} fallback={move |_| view! {cx, <></> }}>
                            <span class="indicator-item indicator-start badge badge-secondary"></span>
                        </Show>
                        <button class="btn btn-primary" on:click={ move |_| update.dispatch(())}>"Update"</button>
                    </div>

                </div>
            </div>
        </div>
        <div class="card w-full md:w-2/6 p-6 bg-base-100 shadow-xl ml-1 mt-2 mb-4">
            <div class="text-xl font-semibold ">"Authorized Devices"</div>
            <div class="divider mt-2"></div>
            <div class="max-w-full h-full overflow-y-auto">
                <table class="table w-full">
                    <thead>
                        <tr>
                            <th>"Device"</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        <For each={move || tokens.get()} key={|token| token.token.clone()} view={move |cx, token| view!{ cx, <Token token=token /> }} />
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
pub fn Token(cx: Scope, token: FilteredUserToken) -> impl IntoView {
    use chrono::{Duration, TimeZone, Utc};

    let timestamp = token.last_used;
    let now = Utc::now();
    let duration = now.signed_duration_since(Utc.timestamp_opt(timestamp, 0).unwrap());
    let last_used = indicatif::HumanDuration(duration.to_std().unwrap());

    let revoke = create_action(cx, move |()| {
        let app_state = use_context::<AppState>(cx).expect("AppState to exist");
        let api = app_state.api.get_untracked().expect("api to exist");
        let user = app_state.user.clone();
        let token = token.token.clone();
        async move {
            if let Ok(new_user) = api.revoke_token(&RevokeTokenSchema { token }).await {
                user.set(Some(new_user));
            }
        }
    });

    view! { cx,
        <tr>
            <td>
                <div class="flex items-center space-x-3">
                    <div>
                        <div class="font-bold">
                            { token.ip }
                        </div>
                        <div class="text-sm opacity-50">
                            { format!("Last used {} ago", last_used) }
                        </div>
                    </div>
                </div>
            </td>
            <td class="text-right">
                <button class="btn btn-ghost hover:btn-error btn-xs" on:click=move |_| revoke.dispatch(())>"Revoke"</button>
            </td>
        </tr>
    }
}
