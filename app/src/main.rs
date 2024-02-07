use std::net::SocketAddr;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::debug;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr).await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello there!"
}
