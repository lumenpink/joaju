use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use joaju::create_app;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_spec_compile_markdown_to_pdf() {
    let app = create_app();

    let input_markdown = r#"---
titulo: Relatório Clínico de Avaliação
autor: Dra. Lumen Pink
---

# Introdução

Este documento foi gerado pelo compilador in-process do Joaju.

```table
%% caption=Tabela de Frequências
Marcador | Frequência
D1S80 | 0.45
D2S1338 | 0.82
```

Fim do relatório."#;

    let payload = json!({
        "markdown": input_markdown
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/compile")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        "application/pdf"
    );

    let pdf_bytes = response.into_body().collect().await.unwrap().to_bytes();
    assert!(pdf_bytes.len() > 100);
    // PDF Magic Bytes: %PDF-
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[tokio::test]
async fn test_spec_compile_empty_markdown_returns_bad_request() {
    let app = create_app();

    let payload = json!({
        "markdown": "   "
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/compile")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
