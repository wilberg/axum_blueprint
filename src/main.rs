mod config;
mod handlers;
mod openapi;

use axum::serve;
use openapi::get_router;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    // Install tracing subscriber.
    tracing_subscriber::fmt::init();

    // Retrieve app config.
    let config = config::get().await;

    // Start server.
    start_server(config.host(), config.port()).await
}

async fn start_server(host: &str, port: &u16) {
    let router = get_router();

    let listener = TcpListener::bind(format!("{host}:{port}"))
        .await
        .expect("Error listening");

    info!("Listening for connections on {host}:{port}");

    serve(listener, router)
        .await
        .expect("Could not start server");
}
