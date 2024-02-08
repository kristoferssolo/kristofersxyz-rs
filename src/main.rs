use anyhow::Result;
use app::{config::Config, router::init_router, state::AppState};
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing::debug;
use tracing_subscriber::fmt::init;

lazy_static! {
    static ref CONFIG: Config = Config::from_env().expect("Failed to load configuration");
}

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.database_url)
        .await?;

    let state = Arc::new(AppState::new(pool));

    let router = init_router(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.port));
    let listener = TcpListener::bind(addr).await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
