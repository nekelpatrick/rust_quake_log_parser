use crate::handlers::{get_log_data_handler, health_check_handler};
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/api/healthCheck", get(health_check_handler))
        .route("/api/logs", get(get_log_data_handler))
}
