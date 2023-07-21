use std::{fmt::Display, time::Duration};

use leptos::*;
use serde::{Deserialize, Serialize};

use crate::app_state;

#[derive(Debug, Clone)]
pub struct Toast {
    pub cx: Scope,
    pub id: u64,
    pub msg: String,
    pub style: ToastStyle,
}

impl Toast {
    pub fn new(cx: Scope, msg: impl Into<String>, style: Option<ToastStyle>) -> Self {
        Self {
            cx,
            id: chrono::Utc::now().timestamp_micros() as u64,
            msg: msg.into(),
            style: style.unwrap_or_default(),
        }
    }
}

#[component]
pub fn ToastContainer(cx: Scope) -> impl IntoView {
    let app_state = use_context::<app_state::AppState>(cx).expect("AppState to exist");

    view! { cx,
        <div class="toast toast-top toast-center">
        <For each={move || app_state.toasts.get()} key={move |toast| { toast.id }} view={move |cx, toast| {
            view! { cx,
                <Toast toast=toast />
            }
        }} />
        </div>
    }
}

// This just poofs out of view when dismissed, but it should animate out
#[component]
pub fn Toast(cx: Scope, toast: Toast) -> impl IntoView {
    let app_state = use_context::<app_state::AppState>(cx).expect("AppState to exist");

    let begone = move |_| {
        app_state.toasts.update(|toasts| {
            toasts.retain(|t| t.id != toast.id);
        });
    };

    create_effect(cx, move |_| {
        set_timeout(
            move || {
                app_state.toasts.update(|toasts| {
                    toasts.retain(|t| t.id != toast.id);
                });
            },
            Duration::from_secs(5),
        );
    });

    view! { cx,
        <div on:click=begone class={format!("alert {} cursor-pointer whitespace-nowrap drop-shadow-lg", toast.style)}>
            <div>
                <span>{toast.msg}</span>
            </div>
        </div>
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum ToastStyle {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl Display for ToastStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = match self {
            ToastStyle::Info => "alert-info text-info-content",
            ToastStyle::Success => "alert-success text-success-content",
            ToastStyle::Warning => "alert-warning text-warning-content",
            ToastStyle::Error => "alert-error text-error-content",
        };

        write!(f, "{}", class)
    }
}
