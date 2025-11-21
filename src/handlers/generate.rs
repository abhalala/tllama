use crate::handlers::AppState;
use crate::models::ollama::{GenerateRequest, GenerateResponse};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use t3router::t3::message::{Message as T3Message, Type as T3MessageType};

/// Handles the `/api/generate` endpoint for single-turn text generation.
///
/// This function takes a `GenerateRequest` containing the model name and a prompt,
/// and returns a `GenerateResponse` with the generated text.
///
/// It sends the prompt to the T3 backend and formats the response to be compatible
/// with the Ollama API.
pub async fn generate(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();

    let message =
        T3Message::new(T3MessageType::User, req.prompt.clone());

    let mut client = state.t3_client.lock().await;

    match client.send_message(&req.model, message).await {
        Ok(content) => {
            let duration = start.elapsed();

            let response = GenerateResponse {
                model: req.model,
                created_at: Utc::now().to_rfc3339(),
                response: content.clone(),
                done: true,
                done_reason: Some("stop".to_string()),
                context: Some(vec![]),
                total_duration: Some(duration.as_nanos() as u64),
                load_duration: Some(0),
                prompt_eval_count: Some(
                    req.prompt.split_whitespace().count() as u32
                ),
                prompt_eval_duration: Some(0),
                eval_count: Some(content.split_whitespace().count() as u32),
                eval_duration: Some(duration.as_nanos() as u64),
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Error generating response: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response()
        }
    }
}
