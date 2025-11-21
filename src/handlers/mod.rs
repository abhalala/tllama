pub mod chat;
pub mod generate;
pub mod health;
pub mod models;

use crate::config::Config;
use crate::db;
use crate::t3::T3Client;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;
use t3router::t3::models::ModelInfo as T3ModelInfo;

/// Contains the shared state for the application.
///
/// This struct is used by `axum` to provide shared access to resources like the T3 client
/// and application configuration across all handlers. It is created once at startup and
/// then cloned for each request.
#[derive(Clone)]
pub struct AppState {
    /// A thread-safe, shared instance of the T3 client.
    pub t3_client: Arc<Mutex<T3Client>>,
    /// The application's configuration.
    pub config: Config,
    /// A thread-safe, shared instance of the database connection.
    pub db_conn: Arc<std::sync::Mutex<Connection>>,
    /// Cache for the list of available models.
    pub models_cache: Arc<Mutex<Option<Vec<T3ModelInfo>>>>,
}

impl AppState {
    /// Creates a new instance of `AppState`.
    ///
    /// This function initializes the `T3Client` with the provided configuration and
    /// wraps it in an `Arc<Mutex<>>` for safe concurrent access.
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let t3_client = T3Client::new(
            config.cookies.clone(),
            format!("\"{}\"", config.convex_session_id),
        )
        .await?;

        let db_conn = Arc::new(std::sync::Mutex::new(db::initialize_database()?));

        Ok(Self {
            t3_client: Arc::new(Mutex::new(t3_client)),
            config,
            db_conn,
            models_cache: Arc::new(Mutex::new(None)),
        })
    }
}