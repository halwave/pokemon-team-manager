//! HTTP route composition and shared application state.

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put},
};
use tokio::sync::Mutex;

use crate::{pokemon_catalog::PokemonCatalog, repo::TeamRepository};

pub mod health;
pub mod pokemon;
pub mod team;

#[derive(Clone)]
pub struct AppState {
    pub pokemon_catalog: PokemonCatalog,
    pub team_repository: Arc<dyn TeamRepository>,
    pub team_lock: Arc<Mutex<()>>,
}

pub fn router(pokemon_catalog: PokemonCatalog, team_repository: Arc<dyn TeamRepository>) -> Router {
    let state = AppState {
        pokemon_catalog,
        team_repository,
        team_lock: Arc::new(Mutex::new(())),
    };

    Router::new()
        .route("/health", get(health::health))
        .route("/pokemon/{id}", get(pokemon::get_pokemon))
        .route("/team", get(team::get_team))
        .route("/team/slots", post(team::add_pokemon))
        .route(
            "/team/slots/{slot_id}",
            put(team::replace_pokemon).delete(team::remove_pokemon),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
        response::Response,
    };
    use serde_json::Value;
    use tower::ServiceExt;

    use super::router;
    use crate::{pokemon_catalog::PokemonCatalog, repo::sqlite::SqliteTeamRepository};

    async fn test_router() -> axum::Router {
        let repository = SqliteTeamRepository::in_memory().await.unwrap();
        router(PokemonCatalog::load().unwrap(), Arc::new(repository))
    }

    async fn request(app: &axum::Router, method: &str, uri: &str, body: &str) -> Response {
        app.clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(uri)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_owned()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    async fn json_body(response: Response) -> Value {
        let body = to_bytes(response.into_body(), 4096).await.unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    #[tokio::test]
    async fn health_endpoint_returns_ok_json() {
        let app = test_router().await;
        let response = request(&app, "GET", "/health", "").await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            json_body(response).await,
            serde_json::json!({"status": "ok"})
        );
    }

    #[tokio::test]
    async fn team_routes_persist_slot_operations_and_validate_inputs() {
        let app = test_router().await;

        let response = request(&app, "GET", "/team", "").await;
        assert_eq!(response.status(), StatusCode::OK);
        let team = json_body(response).await;
        assert_eq!(team["slots"].as_array().unwrap().len(), 6);
        assert!(team["slots"][0]["pokemon"].is_null());

        let response = request(&app, "POST", "/team/slots", r#"{"pokemon_id":25}"#).await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let team = json_body(response).await;
        assert_eq!(team["slots"][0]["slot_id"], 1);
        assert_eq!(team["slots"][0]["pokemon"]["id"], 25);

        let response = request(&app, "POST", "/team/slots", r#"{"pokemon_id":25}"#).await;
        assert_eq!(response.status(), StatusCode::CREATED);
        let team = json_body(response).await;
        assert_eq!(team["slots"][1]["pokemon"]["id"], 25);

        let response = request(&app, "PUT", "/team/slots/1", r#"{"pokemon_id":1}"#).await;
        assert_eq!(response.status(), StatusCode::OK);
        let team = json_body(response).await;
        assert_eq!(team["slots"][0]["pokemon"]["id"], 1);

        let response = request(&app, "DELETE", "/team/slots/2", "").await;
        assert_eq!(response.status(), StatusCode::OK);
        let team = json_body(response).await;
        assert!(team["slots"][1]["pokemon"].is_null());

        let response = request(&app, "POST", "/team/slots", r#"{"pokemon_id":152}"#).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            json_body(response).await["error"],
            "Pokemon ID 152 is not in the catalog"
        );

        let response = request(&app, "DELETE", "/team/slots/2", "").await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let response = request(&app, "PUT", "/team/slots/7", r#"{"pokemon_id":1}"#).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response = request(&app, "GET", "/pokemon/152", "").await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let response = request(
            &app,
            "POST",
            "/team/slots",
            r#"{"pokemon_id":"not-a-number"}"#,
        )
        .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            json_body(response).await["error"],
            "Invalid JSON request body; expected a pokemon_id number"
        );

        let response = request(&app, "GET", "/pokemon/not-a-number", "").await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(json_body(response).await["error"], "Invalid path parameter");

        for _ in 0..5 {
            let response = request(&app, "POST", "/team/slots", r#"{"pokemon_id":25}"#).await;
            assert_eq!(response.status(), StatusCode::CREATED);
        }
        let response = request(&app, "POST", "/team/slots", r#"{"pokemon_id":25}"#).await;
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}
