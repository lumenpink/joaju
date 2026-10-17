pub mod compiler;
pub mod engine;
pub mod preprocessor;
pub mod unfurl;

use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
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
    let is_chem = payload
        .format
        .as_deref()
        .map(|f| matches!(f, "openchemlib" | "openbabel"))
        .unwrap_or(false)
        || payload.code.contains("```openchemlib")
        || payload.code.contains("```openbabel");

    let is_table = payload
        .format
        .as_deref()
        .map(|f| f == "table")
        .unwrap_or(false)
        || payload.code.contains("```table");

    if is_table {
        let clean = payload.code.trim();
        let body = if clean.starts_with("```table") {
            clean
                .trim_start_matches("```table")
                .trim_end_matches("```")
                .trim()
        } else {
            clean
        };
        return match preprocessor::render_single_table(body) {
            Ok(typst) => Ok(Json(engine::pedigree::RenderResponse {
                format: "table".to_string(),
                r#type: "typst".to_string(),
                svg: typst,
            })),
            Err(err) => Err((
                StatusCode::BAD_REQUEST,
                Json(engine::pedigree::RenderErrorResponse { error: err }),
            )),
        };
    }

    let result = if is_chem {
        engine::chem::render_chem(&payload.code, payload.format.as_deref())
    } else {
        engine::pedigree::render_pedigree(&payload.code, payload.format.as_deref())
    };

    match result {
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

pub async fn compile_handler(
    Json(payload): Json<compiler::CompileRequest>,
) -> Result<(HeaderMap, Vec<u8>), (StatusCode, String)> {
    let md = payload.markdown.unwrap_or_default();
    match compiler::compile_markdown_to_pdf_with_options(&md, payload.template.as_deref()) {
        Ok(pdf_bytes) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/pdf"),
            );
            Ok((headers, pdf_bytes))
        }
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

pub async fn unfurl_handler(
    Json(payload): Json<unfurl::UnfurlRequest>,
) -> Result<Json<unfurl::UnfurlResponse>, (StatusCode, Json<unfurl::UnfurlErrorResponse>)> {
    let url = match payload.url {
        Some(u) if !u.trim().is_empty() => u.trim().to_string(),
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(unfurl::UnfurlErrorResponse {
                    ok: false,
                    error: "Parâmetro \"url\" é obrigatório.".to_string(),
                }),
            ));
        }
    };

    match unfurl::unfurl_url(&url, payload.archive).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => {
            let status = if err.contains("Protocolo inválido")
                || err.contains("URL inválida")
                || err.contains("não é permitido")
            {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::UNPROCESSABLE_ENTITY
            };
            Err((
                status,
                Json(unfurl::UnfurlErrorResponse {
                    ok: false,
                    error: err,
                }),
            ))
        }
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
        .route("/compile", post(compile_handler))
        .route("/unfurl", post(unfurl_handler))
        .nest_service("/locales", ServeDir::new("locales"))
        .nest_service("/site", ServeDir::new("site"))
        .nest_service("/public", ServeDir::new("public"))
        .layer(cors)
        .with_state(state)
}
