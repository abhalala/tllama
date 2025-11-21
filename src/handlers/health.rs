use axum::{http::StatusCode, response::IntoResponse, Json};

/// Handles the root (`/`) endpoint, serving as a health check.
///
/// This function returns a simple "Ollama is running" message to indicate that the
/// server is up and running.
pub async fn root() -> impl IntoResponse {
    (
        StatusCode::OK,
        "Ollama is running (TLlama replication layer)",
    )
}

pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"version": "0.1.0"})))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_root_health_check() {
        let response = root().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(body, "Ollama is running (TLlama replication layer)");
    }
}
