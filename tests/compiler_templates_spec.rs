use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use joaju::create_app;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_spec_compile_with_ufsc_template_generates_cover_and_numbered_body() {
    let app = create_app();

    let input_markdown = r#"---
titulo: Elaboração de Heredograma Familiar
subtitulo: Avaliação Clínica e Genética
autor: Dra. Lumen Pink
instituicao: Universidade Federal de Santa Catarina
local: Florianópolis
ano: 2026
template: trabalho-ufsc
---

# Introdução

Este é o corpo do trabalho institucional UFSC após a capa.

# Heredograma Clínico

```pedtools
1 0 0 1 0
2 0 0 2 0
3 1 2 1 1
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

    let status = response.status();
    let content_type = response.headers().get(header::CONTENT_TYPE).cloned();
    let pdf_bytes = response.into_body().collect().await.unwrap().to_bytes();

    if status != StatusCode::OK {
        eprintln!("Response error: {}", String::from_utf8_lossy(&pdf_bytes));
    }

    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        content_type.as_ref().map(|v| v.to_str().unwrap()),
        Some("application/pdf")
    );
    assert!(pdf_bytes.len() > 1000);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[tokio::test]
async fn test_spec_compile_with_abnt_template() {
    let app = create_app();

    let input_markdown = r#"---
titulo: Artigo Científico com Normas ABNT
autor: Dra. Lumen Pink
template: abnt
---

# Introdução

Este artigo acadêmico utiliza o template oficial ABNT de data/template.typ.

```table
%% caption=Tabela de Resultados
Variável | Valor
Amostras | 150
Precisão | 99.4%
```
"#;

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
    let pdf_bytes = response.into_body().collect().await.unwrap().to_bytes();
    assert!(pdf_bytes.len() > 500);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[tokio::test]
async fn test_spec_partial_update_preserves_cover_and_updates_content() {
    let app = create_app();

    let frontmatter = r#"---
titulo: Relatório Clínico Dinâmico
autor: Dra. Lumen Pink
instituicao: UFSC - Hospital Universitário
template: trabalho-ufsc
---
"#;

    // Versão 1 (apenas introdução)
    let md_v1 = format!("{}\n# Introdução\n\nPrimeiro rascunho no Mandu.", frontmatter);
    let payload_v1 = json!({ "markdown": md_v1 });

    let response_v1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/compile")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload_v1).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_v1.status(), StatusCode::OK);
    let pdf_v1 = response_v1.into_body().collect().await.unwrap().to_bytes();

    // Versão 2 (update parcial do Mandu com nova seção e molécula)
    let md_v2 = format!(
        "{}\n# Introdução\n\nPrimeiro rascunho no Mandu.\n\n# Farmacologia\n\n```openchemlib\nCC(=O)Oc1ccccc1C(=O)O\n```\n",
        frontmatter
    );
    let payload_v2 = json!({ "markdown": md_v2 });

    let response_v2 = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/compile")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload_v2).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_v2.status(), StatusCode::OK);
    let pdf_v2 = response_v2.into_body().collect().await.unwrap().to_bytes();

    // A versão 2 deve ser maior devido à adição da nova seção e SVG molecular
    assert!(pdf_v2.len() > pdf_v1.len());
    assert_eq!(&pdf_v1[0..5], b"%PDF-");
    assert_eq!(&pdf_v2[0..5], b"%PDF-");
}
