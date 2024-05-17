use crate::services::log_parser::LogParser;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct LogQuery {
    debug: Option<bool>,
}

pub async fn get_log_data_handler(Query(params): Query<LogQuery>) -> impl IntoResponse {
    let file_path = "data/qgames.log";
    match LogParser::parse_log(file_path) {
        Ok(report) => {
            if params.debug.unwrap_or(false) {
                info!("{}", report.to_string());
            }
            (StatusCode::OK, Json(report)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to parse log file: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to parse log file"})),
            )
                .into_response()
        }
    }
}
