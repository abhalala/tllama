use crate::db;
use crate::handlers::AppState;
use crate::models::ollama::{ModelDetails, ModelInfo, ModelList};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use t3router::t3::models::ModelsClient;
use tracing::error;

/// Handles the `/api/tags` endpoint, returning a list of available models.
///
/// This function returns a hardcoded list of models that are known to be available
/// through the T3 Chat backend. The response is formatted to be compatible with the
/// Ollama API.
pub async fn list_models(State(state): State<AppState>) -> impl IntoResponse {
    let db_conn = state.db_conn.clone();
    let models_client = ModelsClient::new(
        state.config.cookies.clone(),
        format!("\"{}\"", state.config.convex_session_id),
    );

    let models = match models_client.get_model_statuses().await {
        Ok(models) => {
            let num_models = models.len();
            let log_message = format!("Successfully fetched {} models from t3.chat", num_models);
            tokio::task::spawn_blocking(move || {
                let conn = db_conn.lock().unwrap();
                if let Err(e) = db::log_status(&conn, "status", &log_message) {
                    error!("Failed to log status to db: {}", e);
                }
            });
            models
                .into_iter()
                .map(|m| create_model_info(&m.name))
                .collect()
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
            vec![]
        }
    };

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

fn create_model_info(name: &str) -> ModelInfo {
    ModelInfo {
        name: name.to_string(),
        modified_at: Utc::now().to_rfc3339(),
        size: 0,
        digest: format!("sha256:{}", uuid::Uuid::new_v4()),
        details: ModelDetails {
            format: "t3chat".to_string(),
            family: "t3".to_string(),
            families: vec!["t3".to_string()],
            parameter_size: "cloud".to_string(),
            quantization_level: "cloud".to_string(),
        },
    }
}
