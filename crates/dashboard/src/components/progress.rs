use api_schema::response::{State, Status};
use leptos::{ev::progress, *};

use crate::api::AuthorizedApi;

#[derive(Clone, Default)]
pub struct Progress {
    value: i64,
    max: i64,
}

impl Progress {
    pub fn update(&mut self, value: i64, max: i64) {
        self.value = value;
        self.max = max;
    }
}

#[component]
pub fn ProgressBar(cx: Scope, values: RwSignal<Progress>) -> impl IntoView {
    let value = create_rw_signal(cx, 0);
    let max = create_rw_signal(cx, 0);

    create_effect(cx, move |_| {
        let values = values.get();
        value.set(values.value);
        max.set(values.max);
    });

    view! { cx,
        {move || {
            if value.get() == 0 && max.get() == 0 {
                view! { cx,
                    <progress class="progress progress-info w-full mt-2"></progress>
                }.into_view(cx)
            } else {
                view! { cx,
                    <progress class="progress progress-info w-full mt-2" value=move || value.get() max=move || max.get()></progress>
                }.into_view(cx)
            }
        }}
    }
}
