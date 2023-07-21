use std::net::SocketAddr;

use axum::{
    http::{header::*, Method},
    Extension, Router,
};
pub use config::*;
use repository::{PresetRepository, UserRepository, UserTokenRepository};
use route::create_router;
pub use service::*;
use sqlx::sqlite::SqlitePoolOptions;
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

    let config_service = ConfigService::new();
    let status = StatusService::new();
    let preset = PresetService::new(preset_repository.clone());
    let log = LogService::new();

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
        .layer(Extension(config_service))
        .layer(Extension(status))
        .layer(Extension(preset))
        .layer(Extension(log))
        .layer(cors);

    let dashboard = dashboard::get_router();

    let app = Router::new().merge(api).merge(dashboard);

    tracing::info!("Webserver listening on port 3000");

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
mod model;
mod repository;
mod response;
mod route;
mod service;
