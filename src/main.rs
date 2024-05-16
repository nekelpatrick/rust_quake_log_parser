use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;

mod handlers;
mod routes;
mod services;
mod utils;

use services::log_parser::LogParser;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse the log file
    if let Err(e) = LogParser::parse_log("data/qgames.log") {
        error!("Failed to parse log file: {}", e);
    }

    let app = routes::create_routes();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Server started successfully at {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
