//! HTTP route composition and shared application state.

use axum::{Router, routing::get};

pub mod health;
pub mod team;

pub fn router() -> Router {
    Router::new().route("/health", get(health::health))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::router;

    #[tokio::test]
    async fn health_endpoint_returns_ok_json() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body.as_ref(), br#"{"status":"ok"}"#);
    }
}
