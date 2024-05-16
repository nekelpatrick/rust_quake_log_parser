use axum::{response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;

mod handlers;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = routes::create_routes();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Server started successfully at {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
