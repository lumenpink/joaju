use axum::{extract::State, http::Method, response::Json, routing::get, Router};
use serde::Serialize;
use std::sync::Arc;
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub start_time: Instant,
    pub version: &'static str,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub uptime: f64,
}

pub async fn health_handler(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        name: "joaju",
        version: state.version,
        uptime: state.start_time.elapsed().as_secs_f64(),
    })
}

pub fn create_app() -> Router {
    let state = Arc::new(AppState {
        start_time: Instant::now(),
        version: env!("CARGO_PKG_VERSION"),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::HEAD])
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_handler))
        .nest_service("/locales", ServeDir::new("locales"))
        .nest_service("/site", ServeDir::new("site"))
        .nest_service("/public", ServeDir::new("public"))
        .layer(cors)
        .with_state(state)
}
