use std::cell::RefCell;

use leptos::*;

#[component]
pub fn ClientOnly(cx: Scope, children: Children) -> impl IntoView {
    let (children_view, set_children_view) = create_signal(cx, None::<View>);
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    request_animation_frame(move || set_children_view.set(Some(children(cx).into_view(cx))));
    move || children_view.get()
}
