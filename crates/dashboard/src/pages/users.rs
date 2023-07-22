use api_schema::{request::UpdateUserSchema, response::FilteredUser};
use leptos::*;
use leptos_use::on_click_outside;

use crate::{app_state::AppState, components::ToastStyle};

#[component]
pub fn Users(cx: Scope) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("there to be an AppState");
    let users_signal = create_rw_signal(cx, vec![]);
    let selected_user = create_rw_signal(cx, None::<FilteredUser>);

    create_effect(cx, move |_| {
        spawn_local(async move {
            let api = app_state.api.get_untracked().expect("there to be an api");
            let Ok(users) = api.get_users().await else {
                app_state.toast(cx, "Failed to get users", Some(ToastStyle::Error));
                return;
            };
            users_signal.set(users.iter().map(|user| create_rw_signal(cx, user.clone())).collect());
        });
    });

    create_effect(cx, move |_| {
        if let Some(user) = selected_user.get() {
            users_signal.update(|user_signal| {
                // selected user has been updated, we need to update the matching user in the list
                for (i, u) in user_signal.iter().enumerate() {
                    if u.get().id == user.id {
                        user_signal[i].set(user.clone());
                        break;
                    }
                }
            })
        }
    });

    view! {cx,
        <div class="card w-full p-6 bg-base-100 shadow-xl mr-1 mt-2 mb-4" class=("md:w-2/6", move || selected_user.get().is_some())>
            <div class="text-xl font-semibold ">"Users"</div>
            <div class="divider mt-2"></div>
            <div class="max-w-full h-full overflow-y-auto">
                <table class="table w-full">
                    <thead>
                        <tr>
                            <th>"User"</th>
                        </tr>
                    </thead>
                    <tbody>
                        // This for doesnt update children reactively.
                        // <For each={move || users_signal.get()} key={|user: &RwSignal<FilteredUser>| user.get().id.clone()} view={move |cx, user| view!{ cx, <ListUser user=user selected_user=selected_user /> }} />

                        // This does work.
                        { move || {
                            let users = users_signal.get();
                            let mut v = Vec::with_capacity(users.len());
                            for user in users {
                                v.push(view! { cx, <ListUser user=user selected_user=selected_user /> });
                            }
                            v
                        }}



                    </tbody>
                </table>
            </div>
        </div>
        { move || { if let Some(user) = selected_user.get() {
            view! { cx, <User user_signal=selected_user /> }.into_view(cx)
        } else {
            view! { cx, <></> }.into_view(cx)
        }}}
    }
}

#[component]
pub fn ListUser(
    cx: Scope,
    user: RwSignal<FilteredUser>,
    selected_user: RwSignal<Option<FilteredUser>>,
) -> impl IntoView {
    let select = create_action(cx, move |user: &FilteredUser| {
        let user = user.clone();
        async move { selected_user.set(Some(user)) }
    });

    view! { cx,
        <tr on:click=move |_| select.dispatch(user.get().clone()) class="cursor-pointer">
            <td>
                <div class="flex items-center space-x-3">
                    <div>
                        <div class="font-bold">
                            { move || user.get().name }
                        </div>
                        <div class="text-sm opacity-50">
                            { move || user.get().email }
                        </div>
                    </div>
                </div>
            </td>
        </tr>
    }
}

#[component]
pub fn User(cx: Scope, user_signal: RwSignal<Option<FilteredUser>>) -> impl IntoView {
    let user = user_signal.get_untracked().expect("there to be a user");

    let id = user.id.clone();
    let name_signal = create_rw_signal(cx, user.name.clone());
    let email_signal = create_rw_signal(cx, user.email.clone());
    let verified_signal = create_rw_signal(cx, user.verified.clone());

    let changed = Signal::derive(cx, move || {
        let user = user.clone();
        let name = name_signal.get();
        let email = email_signal.get();
        let verified = verified_signal.get();
        return user.name != name || user.email != email || user.verified != verified;
    });

    let update = create_action(cx, move |()| {
        let id = id.clone();
        let name = name_signal.get_untracked();
        let email = email_signal.get_untracked();
        let verified = verified_signal.get_untracked();

        async move {
            let app_state = use_context::<AppState>(cx).expect("there to be an AppState");

            if !email.contains('@') {
                app_state.toast(cx, "Invalid email", Some(ToastStyle::Error));
                return;
            }

            let api = app_state.api.get_untracked().expect("there to be an api");
            let Ok(user) = api
                .update_user(&UpdateUserSchema {
                    id: Some(id),
                    name,
                    email,
                    password: None,
                    verified: Some(verified),
                })
                .await
            else {
                app_state.toast(cx, "Failed to update user", Some(ToastStyle::Error));
                return;
            };
            user_signal.set(Some(user.clone()));
            app_state.toast(cx, "Updated user", Some(ToastStyle::Success));
        }
    });

    view! { cx,
        <div class="card w-full md:w-4/6 p-6 bg-base-100 shadow-xl ml-1 mt-2 mb-4">
            <div class="text-xl font-semibold">{ move || name_signal.get_untracked() }</div>
            <div class="divider mt-2"></div>
            <div class="max-w-full h-full overflow-y-auto">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">

                    <div class="form-control w-full">
                        <label class="label">
                            <span class="label-text text-base-content">"Name"</span>
                        </label>
                        <input
                            type="text"
                            class="input  input-bordered w-full"
                            on:input={move |ev| name_signal.set(event_target_value(&ev))}
                            prop:value=move || name_signal.get()
                        />
                    </div>
                    <div class="form-control w-full">
                        <label class="label">
                            <span class="label-text text-base-content">"Email"</span>
                        </label>
                        <input
                            type="text"
                            class="input input-bordered w-full"
                            on:input={move |ev| email_signal.set(event_target_value(&ev))}
                            prop:value=move || email_signal.get()
                        />
                    </div>

                    // checkbox for verified
                    <div class="form-control w-full">
                        <label class="label justify-start">
                            <input
                                type="checkbox"
                                class="checkbox"
                                on:input={move |ev| verified_signal.set(event_target_checked(&ev))}
                                prop:checked=move || verified_signal.get()
                            />
                            <span class="label-text text-base-content ml-4">"Verified"</span>
                        </label>
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
        </div>
    }
    .into_view(cx)
}
