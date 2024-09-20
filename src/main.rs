pub mod models;
pub mod network;
pub mod rest;

use axum::Router;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;

use std::{net::SocketAddr, sync::Arc};

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    #[cfg(debug_assertions)]
    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(LevelFilter::DEBUG)
        .without_time()
        .finish();

    #[cfg(not(debug_assertions))]
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let state: Arc<Mutex<models::app::ApplicationState>> = models::app::ApplicationState::new();

    // build our application with a route
    let app = Router::new()
        //.nest("/v1/websocket", network::websocket::generate_router())
        .nest("/v1", rest::player::generate_router())
        .layer(TraceLayer::new_for_http())
        // Adds the state.
        .with_state(state.clone());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        state.lock().await.config.host.clone(),
        state.lock().await.config.port.clone()
    ))
    .await
    .expect("Failed to bind to address");
    axum::serve(listener, app,)
        .await
        .expect("Failed creating server");

    Ok(())
}
