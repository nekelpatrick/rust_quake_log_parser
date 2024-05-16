use axum::{response::IntoResponse, Json};
use tracing::info;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    info!("Health check endpoint was called");

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}
