use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{handlers::*, jwt_auth::auth, AppState};

pub fn create_router(app_state: AppState) -> Router {
    let auth_layer = middleware::from_fn_with_state(app_state.clone(), auth);

    Router::new()
        // Protected routes
        .route("/api/v1/users/me", get(get_me_handler))
        .route("/api/v1/auth/logout", get(logout_handler))
        .route("/api/v1/steam/status/:app_id", get(update_arma))
        .route("/api/v1/status", get(api_status_handler))
        .route("/api/v1/arma/update", get(update_arma))
        .route("/api/v1/arma/cancel_update", get(cancel_update_arma))
        .route("/api/v1/arma/start", get(start_arma))
        .route("/api/v1/arma/stop", get(stop_arma))
        .route("/api/v1/arma/restart", get(restart_arma))
        .route("/api/v1/logs/:channel", get(api_logs))
        // SSE routes
        .route("/sse/v1/status", get(sse_status_handler))
        .route("/sse/v1/logs", get(sse_logs))
        .layer(auth_layer)
        // Public routes
        .route("/api/v1/auth/register", post(register_user_handler))
        .route("/api/v1/auth/login", post(login_user_handler))
        .with_state(app_state)
}
