use axum::{middleware, routing::*, Router};

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
        .route("/api/v1/arma/mods/download", get(download_missing_mods))
        .route("/api/v1/arma/mods/check", get(force_check))
        .route("/api/v1/arma/config/:channel", get(get_config))
        .route("/api/v1/arma/config/:channel", post(post_config))
        .route("/api/v1/logs/:channel", get(api_logs))
        .route("/api/v1/presets", get(get_presets))
        .route("/api/v1/presets", post(create_preset))
        .route("/api/v1/presets", patch(select_preset))
        .route("/api/v1/presets", delete(delete_preset))
        .route("/api/v1/presets/item", patch(update_preset_item))
        .route("/api/v1/presets/item/blacklist", post(blacklist_item))
        .route("/api/v1/presets/item/blacklist", delete(unblacklist_item))
        // SSE routes
        .route("/sse/v1/status", get(sse_status_handler))
        .route("/sse/v1/logs", get(sse_logs))
        .route("/sse/v1/presets", get(sse_preset_handler))
        .route("/sse/v1/arma/config", get(sse_config))
        .layer(auth_layer)
        // Public routes
        .route("/api/v1/auth/register", post(register_user_handler))
        .route("/api/v1/auth/login", post(login_user_handler))
        .with_state(app_state)
}
