use std::net::SocketAddr;

use axum::{
    http::{header::*, Method},
    Extension, Router,
};
pub use config::*;
use log_service::LogService;
use repository::{PresetRepository, UserRepository, UserTokenRepository};
use route::create_router;
use sqlx::sqlite::SqlitePoolOptions;
use status_service::{State, StatusService};
use tower_http::cors::CorsLayer;

pub async fn start() {
    let config = Config::init();

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to SQLite.");

    let user_repository = UserRepository::new(pool.clone());
    let user_token_repository = UserTokenRepository::new(pool.clone());
    let preset_repository = PresetRepository::new(pool.clone());

    let status = StatusService::new();
    let log = LogService::new();

    status.set_steam(State::Running).await;

    log.register("steamcmd", paths::get_log_path().join("steamcmd.log"));
    log.register("arma", paths::get_arma_log_path().join("*.rpt"));

    let app_state = AppState {
        db: pool,
        config: config.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let api = create_router(app_state)
        .layer(Extension(user_repository))
        .layer(Extension(user_token_repository))
        .layer(Extension(preset_repository))
        .layer(Extension(status))
        .layer(Extension(log))
        .layer(cors);

    let dashboard = dashboard::get_router();

    let app = Router::new().merge(api).merge(dashboard);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Config,
}

mod config;
mod handlers;
mod jwt_auth;
mod log_service;
mod model;
mod repository;
mod response;
mod route;
mod status_service;
