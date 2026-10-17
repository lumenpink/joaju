use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use joaju::create_app;
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_spec_health_endpoint() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Header CORS presente
    let cors_header = response.headers().get("access-control-allow-origin");
    assert!(cors_header.is_some(), "Header CORS deve estar presente");
    assert_eq!(cors_header.unwrap(), "*");

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("JSON válido no health check");

    assert_eq!(json["status"], "ok");
    assert_eq!(json["name"], "joaju");
    assert!(json["version"].is_string());
    assert!(json["uptime"].is_number());
}

#[tokio::test]
async fn test_spec_cors_preflight_options() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/health")
                .header("Origin", "http://localhost:3000")
                .header("Access-Control-Request-Method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status().is_success(),
        "Preflight OPTIONS deve retornar sucesso"
    );
    let cors_header = response.headers().get("access-control-allow-origin");
    assert!(cors_header.is_some());
}

#[tokio::test]
async fn test_spec_static_locales_route() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/locales/pt.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("Arquivo pt.json deve ser JSON válido");
    assert!(json.is_object());
}

#[tokio::test]
async fn test_mutation_health_uptime_is_non_negative() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let uptime = json["uptime"].as_f64().unwrap();
    assert!(uptime >= 0.0);
}
