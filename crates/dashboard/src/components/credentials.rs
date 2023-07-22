use leptos::{ev, *};
use leptos_router::A;

use crate::app_state::{AppState, Loading};

#[component]
pub fn LoginForm(
    cx: Scope,
    action: Action<(String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("to have found the app_state provided");
    let loading = app_state.loading.clone();

    let (password, set_password) = create_signal(cx, String::new());
    let (email, set_email) = create_signal(cx, String::new());

    let dispatch_action = move || action.dispatch((email.get(), password.get()));

    let button_is_disabled = Signal::derive(cx, move || {
        disabled.get() || password.get().is_empty() || email.get().is_empty()
    });

    view! { cx,
        <div class="relative flex flex-col items-center justify-center h-screen overflow-hidden">
            <div class="w-full p-6 bg-base-200 border-t-4 border-accent rounded-md shadow-md border-top lg:max-w-lg">
                <form class="space-y-4" on:submit=|ev| ev.prevent_default()>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                loading.set(Loading::Ready);
                                view! { cx, <p style="color:red;">{err}</p> }
                            })
                    }}
                    <div>
                        <label class="label">
                            <span class="text-base label-text">"Email"</span>
                        </label>
                        <input
                            type="email"
                            class="w-full input input-bordered"
                            required
                            placeholder="Email Address"
                            prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val);
                            }
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val);
                            }
                        />
                    </div>
                    <div>
                        <label class="label">
                            <span class="text-base label-text">"Password"</span>
                        </label>
                        <input
                            type="password"
                            class="w-full input input-bordered"
                            required
                            placeholder="Password"
                            prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        dispatch_action();
                                    }
                                    _ => {
                                        let val = event_target_value(&ev);
                                        set_password.update(|p| *p = val);
                                    }
                                }
                            }
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_password.update(|p| *p = val);
                            }
                        />
                    </div>
                    <p>"Don't have an account? "<A href=crate::pages::Page::Register.path() class="hover:underline">"Register"</A></p>
                    <div>
                        <button
                            class="btn btn-block"
                            prop:disabled=move || button_is_disabled.get()
                            on:click=move |_| dispatch_action()
                        >
                            "Login"
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

#[component]
pub fn RegisterForm(
    cx: Scope,
    action: Action<(String, String, String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
) -> impl IntoView {
    let app_state = use_context::<AppState>(cx).expect("to have found the app_state provided");
    let loading = app_state.loading.clone();

    let (repeat_password, set_repeat_password) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (email, set_email) = create_signal(cx, String::new());
    let (name, set_name) = create_signal(cx, String::new());

    let passwords_match = Signal::derive(cx, move || {
        password.get() == repeat_password.get() && !password.get().is_empty()
    });

    let dispatch_action = move || action.dispatch((name.get(), email.get(), password.get(), repeat_password.get()));

    let button_is_disabled = Signal::derive(cx, move || {
        disabled.get()
            || password.get().is_empty()
            || email.get().is_empty()
            || name.get().is_empty()
            || !passwords_match.get()
    });

    view! { cx,
        <div class="relative flex flex-col items-center justify-center h-screen overflow-hidden">
            <div class="w-full p-6 bg-base-200 border-t-4 border-accent rounded-md shadow-md border-top lg:max-w-lg">
                <form class="space-y-4" on:submit=|ev| ev.prevent_default()>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                loading.set(Loading::Ready);
                                view! { cx, <p style="color:red;">{err}</p> }
                            })
                    }}
                    <div>
                        <label class="label">
                            <span class="text-base label-text">"Name"</span>
                        </label>
                        <input
                            type="text"
                            class="w-full input input-bordered"
                            required
                            placeholder="Name"
                            prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_name.update(|v| *v = val);
                            }
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_name.update(|v| *v = val);
                            }
                        />
                    </div>
                    <div>
                        <label class="label">
                            <span class="text-base label-text">"Email"</span>
                        </label>
                        <input
                            type="email"
                            class="w-full input input-bordered"
                            required
                            placeholder="Email Address"
                            prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val);
                            }
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val);
                            }
                        />
                    </div>
                    <div>
                        <label class="label">
                            <span class="text-base label-text">"Password"</span>
                        </label>
                        <input
                            type="password"
                            class="w-full input input-bordered"
                            required
                            placeholder="Password"
                            prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        dispatch_action();
                                    }
                                    _ => {
                                        let val = event_target_value(&ev);
                                        set_password.update(|p| *p = val);
                                    }
                                }
                            }
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_password.update(|p| *p = val);
                            }
                        />
                    </div>
                    <div>
                        <label class="label">
                            <span class="text-base label-text">"Repeat Password"</span>
                        </label>
                        <input
                            type="password"
                            class="w-full input input-bordered"
                            required
                            placeholder="Repeat Password"
                            prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        dispatch_action();
                                    }
                                    _ => {
                                        let val = event_target_value(&ev);
                                        set_repeat_password.update(|p| *p = val);
                                    }
                                }
                            }
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_repeat_password.update(|p| *p = val);
                            }
                        />
                    </div>
                    <p>"Your already have an account? "<A href=crate::pages::Page::Login.path() class="hover:underline">"Login"</A></p>
                    <div>
                        <button
                            class="btn btn-block"
                            prop:disabled=move || button_is_disabled.get()
                            on:click=move |_| dispatch_action()
                        >
                            "Create Account"
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
