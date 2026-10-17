use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use joaju::create_app;
use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn test_spec_render_openchemlib_fenced_block() {
    let app = create_app();

    // Aspirina
    let input_code = r#"```openchemlib
CC(=O)Oc1ccccc1C(=O)O
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

    assert_eq!(json["format"], "openchemlib");
    assert_eq!(json["type"], "svg");
    let svg = json["svg"].as_str().expect("SVG string");
    assert!(svg.contains("<svg"), "Resposta deve conter tag svg");
    assert!(svg.contains("</svg>"), "Resposta deve fechar tag svg");
}

#[tokio::test]
async fn test_spec_render_openbabel_direct_smiles() {
    let app = create_app();

    // Cafeína
    let input_code = "Cn1cnc2c1c(=O)n(c(=O)n2C)C";

    let payload = json!({
        "code": input_code,
        "format": "openbabel"
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

    assert_eq!(json["format"], "openbabel");
    assert_eq!(json["type"], "svg");
    let svg = json["svg"].as_str().unwrap();
    assert!(svg.contains("<svg"));
}

#[tokio::test]
async fn test_spec_render_invalid_smiles_returns_error() {
    let app = create_app();

    let input_code = r#"```openchemlib
SMILES_TOTALMENTE_INVALIDO_XYZ!!!
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
async fn test_mutation_estradiol_structure() {
    let app = create_app();

    // Estradiol com diretiva de caption
    let input_code = r#"```openchemlib
%% caption=Estradiol
C[C@]12CC[C@H]3[C@H]([C@@H]1CC[C@@H]2O)CCC4=C3C=CC(=C4)O
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
    assert!(svg.contains("<svg"));
}
