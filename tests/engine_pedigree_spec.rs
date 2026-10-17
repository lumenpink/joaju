use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use joaju::create_app;
use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn test_spec_render_pedtools_fenced_block() {
    let app = create_app();

    let input_code = r#"```pedtools
%% legend=auto
id,pai,mae,sex,fenotipo
1,0,0,M,wt
2,0,0,F,wt
3,1,2,F,hom
```"#;

    let payload = json!({
        "code": input_code
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/render")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("JSON response");

    assert_eq!(json["format"], "pedtools");
    assert_eq!(json["type"], "svg");
    let svg = json["svg"].as_str().expect("SVG string");
    assert!(svg.contains("<svg"), "Resposta deve conter tag svg");
    assert!(svg.contains("</svg>"), "Resposta deve fechar tag svg");
}

#[tokio::test]
async fn test_spec_render_kinship2_direct_csv() {
    let app = create_app();

    let input_code = r#"id,pai,mae,sex,fenotipo
1,0,0,M,wt
2,0,0,F,wt
3,1,2,M,het"#;

    let payload = json!({
        "code": input_code,
        "format": "kinship2"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/render")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("JSON response");

    assert_eq!(json["format"], "kinship2");
    assert_eq!(json["type"], "svg");
    let svg = json["svg"].as_str().unwrap();
    assert!(svg.contains("<svg"));
}

#[tokio::test]
async fn test_spec_render_invalid_pedigree_returns_error() {
    let app = create_app();

    // Ciclo genealógico impossível (1 é pai de 2, 2 é pai de 1)
    let input_code = r#"```pedtools
id,pai,mae,sex,fenotipo
1,2,0,M,wt
2,1,0,F,wt
```"#;

    let payload = json!({
        "code": input_code
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/render")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("JSON response");
    assert!(json["error"].is_string(), "Deve retornar mensagem de erro");
}

#[tokio::test]
async fn test_mutation_render_with_twin_directives() {
    let app = create_app();

    let input_code = r#"```pedtools
%% twin=3,4,mz
id,pai,mae,sex,fenotipo
1,0,0,M,wt
2,0,0,F,wt
3,1,2,M,wt
4,1,2,M,wt
```"#;

    let payload = json!({
        "code": input_code
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/render")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let svg = json["svg"].as_str().unwrap();
    assert!(svg.contains("<line"), "Gêmeos MZ devem gerar linha horizontal de ligação");
}
