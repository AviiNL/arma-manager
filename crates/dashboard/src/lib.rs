#![allow(unused)]
use app::App;
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        mod fileserv;

        use axum::{body::Body as AxumBody, routing::get, Router, extract::*, http::*};
        use axum::response::{Response, IntoResponse};
        use fileserv::file_and_error_handler;
        use leptos_axum::*;
        use leptos::leptos_config::get_config_from_env;

        async fn leptos_routes_handler(
            State(leptos_options): State<LeptosOptions>,
            req: Request<AxumBody>,
        ) -> Response {
            let handler = leptos_axum::render_app_to_stream(
                leptos_options.clone(),
                |cx| view! { cx, <App/> },
            );
            handler(req).await.into_response()
        }

        pub fn get_router() -> Router {
            let conf = get_config_from_env().unwrap();
            let leptos_options = conf.leptos_options;
            let routes = generate_route_list(|cx| view! { cx, <App/> });

            Router::new()
                .leptos_routes_with_handler(routes, get(leptos_routes_handler))
                .fallback(file_and_error_handler)
                .with_state(leptos_options)
        }
    }
}

mod api;
pub mod app;
mod app_state;
mod components;
mod error_template;
mod pages;
mod preset_parser;
mod sse;
