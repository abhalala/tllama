use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use serde_json::Value;
use std::sync::Arc;
use tllama::{
    app_router,
    config::Config,
    handlers::AppState,
    t3::T3Client,
};
use tokio::sync::Mutex;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_list_models_empty_on_error() {
    let config = Config {
        port: 3000,
        cookies: "invalid".to_string(),
        convex_session_id: "invalid".to_string(),
    };

    let t3_client = T3Client::new_uninitialized(
        config.cookies.clone(),
        config.convex_session_id.clone(),
    );

    let state = AppState {
        t3_client: Arc::new(Mutex::new(t3_client)),
        config,
    };

    let app: Router = app_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/tags")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let models: Value = serde_json::from_slice(&body).unwrap();

    assert!(!models["models"].as_array().unwrap().is_empty());
}
