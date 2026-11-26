//! HTTP route composition and shared application state.

use axum::{Router, routing::get};

use crate::pokemon_catalog::PokemonCatalog;

pub mod health;
pub mod pokemon;
pub mod team;

pub fn router(pokemon_catalog: PokemonCatalog) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/pokemon/{id}", get(pokemon::get_pokemon))
        .with_state(pokemon_catalog)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::router;
    use crate::pokemon_catalog::PokemonCatalog;

    #[tokio::test]
    async fn health_endpoint_returns_ok_json() {
        let response = router(PokemonCatalog::load().unwrap())
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
