use api_schema::response::{State, Status};
use leptos::{ev::progress, *};

use crate::api::AuthorizedApi;

#[derive(Clone, Default)]
pub struct Progress {
    pub value: u64,
    pub max: u64,
}

impl Progress {
    pub fn update(&mut self, value: u64, max: u64) {
        self.value = value;
        self.max = max;
    }

    pub fn update_from_line(&mut self, line: &str) {
        if let Some((value, max)) = extract_progress(line) {
            self.update(value, max);
        }
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

fn extract_progress(s: &str) -> Option<(u64, u64)> {
    let split = s.rsplit_once(" / ")?;

    Some((
        split.0.rsplit_once("(")?.1.parse::<u64>().ok()?,
        split.1.split_once(")")?.0.parse::<u64>().ok()?,
    ))
}
