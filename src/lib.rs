pub mod t3_scraper;

use axum::{
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod config;
pub mod handlers;
pub mod models;
pub mod t3;
pub mod db;

use config::Config;
use handlers::AppState;

pub async fn app() -> anyhow::Result<Router> {
    let config = Config::from_env()?;
    let state = AppState::new(config).await?;
    Ok(app_router(state))
}

pub fn app_router(state: AppState) -> Router {
    Router::new()
        // Ollama API endpoints - exact replicas
        .route("/api/generate", post(handlers::generate::generate))
        .route("/api/chat", post(handlers::chat::chat))
        .route("/api/tags", get(handlers::models::list_models))
        .route("/api/ps", get(handlers::models::list_running))
        .route("/api/show", post(handlers::models::show_model))
        .route("/api/copy", post(handlers::models::copy_model))
        .route("/api/delete", delete(handlers::models::delete_model))
        .route("/api/pull", post(handlers::models::pull_model))
        .route("/", get(handlers::health::root))
        .route("/api/version", get(handlers::health::version))
        .nest_service("/docs", ServeDir::new("target/doc"))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

pub async fn run() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tllama=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    tracing::info!("tllama listening on {}", addr);
    
    axum::serve(listener, app().await?).await?;

    Ok(())
}