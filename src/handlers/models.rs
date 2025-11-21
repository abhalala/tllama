use crate::db;
use crate::handlers::AppState;
use crate::models::ollama::{ModelDetails, ModelInfo, ModelList};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use t3router::t3::models::ModelsClient;
use tracing::error;
use uuid::Uuid;

/// Handles the `/api/tags` endpoint, returning a list of available models.
///
/// This function returns a hardcoded list of models that are known to be available
/// through the T3 Chat backend. The response is formatted to be compatible with the
/// Ollama API.
pub async fn list_models(State(state): State<AppState>) -> impl IntoResponse {
    let mut cache = state.models_cache.lock().await;

    let models_to_process = if let Some(cached_models) = &*cache {
        cached_models.clone()
    } else {
        let models_client = ModelsClient::new(
            state.config.cookies.clone(),
            format!("\"{}\"", state.config.convex_session_id),
        );
        match models_client.get_model_statuses().await {
            Ok(fetched_models) => {
                let num_models = fetched_models.len();
                let log_message = format!("Successfully fetched {} models from t3.chat and cached them", num_models);
                let db_conn = state.db_conn.clone();
                tokio::task::spawn_blocking(move || {
                    let conn = db_conn.lock().unwrap();
                    if let Err(e) = db::log_status(&conn, "status", &log_message) {
                        error!("Failed to log status to db: {}", e);
                    }
                });
                
                *cache = Some(fetched_models.clone());
                fetched_models
            }
            Err(e) => {
                let log_message = format!("Failed to fetch models from t3.chat: {}", e);
                error!("{}", log_message);
                let db_conn = state.db_conn.clone();
                tokio::task::spawn_blocking(move || {
                    let conn = db_conn.lock().unwrap();
                    if let Err(e) = db::log_status(&conn, "error", &log_message) {
                        error!("Failed to log error to db: {}", e);
                    }
                });
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(ModelList { models: vec![] })).into_response();
            }
        }
    };

    let models = models_to_process
        .into_iter()
        .map(|m| ModelInfo {
            name: m.name,
            modified_at: Utc::now().to_rfc3339(), // Placeholder, adjust if t3.chat provides this
            size: 0, // Placeholder
            digest: format!("sha256:{}", Uuid::new_v4()), // Generate new digest
            details: ModelDetails {
                format: "t3chat".to_string(), // Placeholder
                family: "t3".to_string(),    // Placeholder
                families: vec!["t3".to_string()], // Placeholder
                parameter_size: "cloud".to_string(), // Placeholder
                quantization_level: "cloud".to_string(), // Placeholder
            },
        })
        .collect();

    (StatusCode::OK, Json(ModelList { models })).into_response()
}

pub async fn list_running(State(_state): State<AppState>) -> impl IntoResponse {
    // Return empty for now
    (
        StatusCode::OK,
        Json(serde_json::json!({"models": []})),
    )
        .into_response()
}

pub async fn show_model(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Not implemented").into_response()
}

pub async fn copy_model(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Not implemented").into_response()
}

pub async fn delete_model(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "Not implemented").into_response()
}

pub async fn pull_model(State(_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "success"})))
        .into_response()
}
