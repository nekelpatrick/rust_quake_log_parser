use crate::handlers::health_check_handler;
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/api/healthCheck", get(health_check_handler))
}
