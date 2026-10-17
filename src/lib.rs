pub mod engine;

use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
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

pub async fn render_handler(
    Json(payload): Json<engine::pedigree::RenderRequest>,
) -> Result<
    Json<engine::pedigree::RenderResponse>,
    (StatusCode, Json<engine::pedigree::RenderErrorResponse>),
> {
    match engine::pedigree::render_pedigree(&payload.code, payload.format.as_deref()) {
        Ok((fmt, svg)) => Ok(Json(engine::pedigree::RenderResponse {
            format: fmt,
            r#type: "svg".to_string(),
            svg,
        })),
        Err(err) => Err((
            StatusCode::BAD_REQUEST,
            Json(engine::pedigree::RenderErrorResponse { error: err }),
        )),
    }
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
        .route("/render", post(render_handler))
        .nest_service("/locales", ServeDir::new("locales"))
        .nest_service("/site", ServeDir::new("site"))
        .nest_service("/public", ServeDir::new("public"))
        .layer(cors)
        .with_state(state)
}
