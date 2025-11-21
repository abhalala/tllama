use crate::handlers::AppState;
use crate::models::ollama::{ChatRequest, ChatResponse, Message};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use t3router::t3::message::{Message as T3Message, Type as T3MessageType};
use t3router::t3::models::ModelsClient;
use tracing::error;

/// Handles the `/api/chat` endpoint, providing a conversational interface with a model.
///
/// This function takes a `ChatRequest` containing the model name and a list of messages,
/// and returns a `ChatResponse` with the model's reply.
///
/// The handler interacts with the T3 client by:
/// 1. Clearing any previous messages from the client's session.
/// 2. Appending the conversation history from the request to the client.
/// 3. Sending the last message to the T3 backend.
/// 4. Formatting the response into an Ollama-compatible `ChatResponse`.
pub async fn chat(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    let start = std::time::Instant::now();

    let mut client = state.t3_client.lock().await;
    let mut cache = state.models_cache.lock().await;

    // Ensure the model cache is populated
    if cache.is_none() {
        let models_client = ModelsClient::new(
            state.config.cookies.clone(),
            format!("\"{}\"", state.config.convex_session_id),
        );
        match models_client.get_model_statuses().await {
            Ok(models) => {
                *cache = Some(models);
            }
            Err(e) => {
                let log_message = format!("Failed to fetch models from t3.chat: {}", e);
                error!("{}", log_message);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({{"error": log_message}})),
                )
                .into_response();
            }
        }
    }

    // Find the model ID from the cache
    let model_id = if let Some(models) = &*cache {
        models.iter().find(|m| m.name == req.model).map(|m| m.id.clone())
    } else {
        // This case should not be reachable if the cache is populated above
        None
    };

    let model_id = match model_id {
        Some(id) => id,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({{"error": format!("Model '{}' not found.", req.model)}})),
            )
            .into_response();
        }
    };

    // Clear previous messages and add conversation history
    client.clear_messages();
    for msg in &req.messages[..req.messages.len().saturating_sub(1)] {
        let msg_type = match msg.role.as_str() {
            "user" => T3MessageType::User,
            "assistant" => T3MessageType::Assistant,
            _ => T3MessageType::User,
        };
        client.append_message(T3Message::new(msg_type, msg.content.clone()));
    }

    // Send the last message
    let last_msg = req.messages.last().unwrap();
    let message = T3Message::new(T3MessageType::User, last_msg.content.clone());

    match client.send_message(&model_id, message).await {
        Ok(content) => {
            let duration = start.elapsed();

            let response = ChatResponse {
                model: req.model,
                created_at: Utc::now().to_rfc3339(),
                message: Message {
                    role: "assistant".to_string(),
                    content: content.clone(),
                    images: None,
                },
                done: true,
                done_reason: Some("stop".to_string()),
                total_duration: Some(duration.as_nanos() as u64),
                load_duration: Some(0),
                prompt_eval_count: Some(
                    last_msg.content.split_whitespace().count() as u32
                ),
                prompt_eval_duration: Some(0), // Placeholder
                eval_count: Some(content.split_whitespace().count() as u32),
                eval_duration: Some(duration.as_nanos() as u64),
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Error in chat: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({{"error": e.to_string()}})),
            )
            .into_response()
        }
    }
}