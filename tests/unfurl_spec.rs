use base64::{engine::general_purpose, Engine as _};
use joaju::unfurl::{extract_metadata_from_html, unfurl_url};

#[tokio::test]
async fn test_spec_unfurl_missing_url_returns_bad_request() {
    let result = unfurl_url("   ", false).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_spec_unfurl_invalid_protocol_returns_error() {
    let result = unfurl_url("ftp://files.example.com/doc.pdf", false).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.contains("Protocolo inválido") || err.contains("protocol") || err.contains("invalid")
    );
}

#[test]
fn test_spec_extract_metadata_from_html() {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Título Fallback</title>
    <meta property="og:title" content="Novo Avanço na Genética Médica" />
    <meta property="og:description" content="Pesquisadores identificam variante patogênica associada à cardiopatia." />
    <meta property="og:image" content="https://saude.org/imagens/cardio.png" />
    <meta property="og:site_name" content="Portal Saúde & Genética" />
    <meta property="article:author" content="Dra. Lumen Pink" />
    <meta property="article:published_time" content="2026-10-15" />
    <link rel="icon" href="/favicon.svg" />
</head>
<body>
    <article>
        <h1>Novo Avanço na Genética Médica</h1>
        <p>Este é o primeiro parágrafo de um artigo com detalhes importantes sobre heredogramas clínicos.</p>
        <p>A investigação permitiu esclarecer os padrões de herança autossômica dominante na genealogia avaliada.</p>
    </article>
</body>
</html>"#;

    let target_url = "https://saude.org/artigos/genetica-medica";
    let meta = extract_metadata_from_html(html, target_url, false).unwrap();

    assert_eq!(
        meta.title.as_deref(),
        Some("Novo Avanço na Genética Médica")
    );
    assert_eq!(
        meta.description.as_deref(),
        Some("Pesquisadores identificam variante patogênica associada à cardiopatia.")
    );
    assert_eq!(
        meta.image_url.as_deref(),
        Some("https://saude.org/imagens/cardio.png")
    );
    assert_eq!(meta.site_name.as_deref(), Some("Portal Saúde & Genética"));
    assert_eq!(meta.author.as_deref(), Some("Dra. Lumen Pink"));
    assert_eq!(meta.date.as_deref(), Some("2026-10-15"));
    assert_eq!(
        meta.favicon_url.as_deref(),
        Some("https://saude.org/favicon.svg")
    );
    assert_eq!(meta.domain, "saude.org");
    assert!(meta.has_article);
    assert_eq!(meta.reading_time_min, 1);
    assert!(meta.pdf_base64.is_none());
}

#[test]
fn test_spec_extract_metadata_with_archive_generates_pdf() {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Relatório Editorial de Arquivamento</title>
    <meta property="og:title" content="Relatório Editorial" />
</head>
<body>
    <article>
        <h1>Relatório Editorial</h1>
        <p>Conteúdo relevante a ser preservado em snapshot tipográfico em PDF nativo.</p>
    </article>
</body>
</html>"#;

    let target_url = "https://arquivo.org/relatorio";
    let meta = extract_metadata_from_html(html, target_url, true).unwrap();

    assert_eq!(meta.title.as_deref(), Some("Relatório Editorial"));
    assert!(meta.pdf_base64.is_some());
    let pdf_b64 = meta.pdf_base64.unwrap();
    let pdf_bytes = general_purpose::STANDARD.decode(pdf_b64).unwrap();
    assert!(pdf_bytes.len() > 100);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[test]
fn test_mutation_extract_metadata_twitter_cards_and_relative_favicon() {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta name="twitter:title" content="Twitter Card Title" />
    <meta name="twitter:description" content="Twitter Card Description" />
    <meta name="twitter:image" content="/assets/card.jpg" />
    <meta name="author" content="Reporter Baquara" />
    <link rel="shortcut icon" href="/static/custom.ico" />
</head>
<body>
    <div>Apenas uma nota breve sobre atualizações do laboratório.</div>
</body>
</html>"#;

    let target_url = "https://noticias.org/lab/updates";
    let meta = extract_metadata_from_html(html, target_url, false).unwrap();

    assert_eq!(meta.title.as_deref(), Some("Twitter Card Title"));
    assert_eq!(
        meta.description.as_deref(),
        Some("Twitter Card Description")
    );
    assert_eq!(
        meta.image_url.as_deref(),
        Some("https://noticias.org/assets/card.jpg")
    );
    assert_eq!(meta.author.as_deref(), Some("Reporter Baquara"));
    assert_eq!(
        meta.favicon_url.as_deref(),
        Some("https://noticias.org/static/custom.ico")
    );
    assert_eq!(meta.domain, "noticias.org");
    assert!(!meta.has_article);
}

#[test]
fn test_mutation_extract_metadata_empty_body_fallback() {
    let html =
        r#"<!DOCTYPE html><html><head><title>Página Vazia</title></head><body></body></html>"#;
    let target_url = "https://vazio.org";
    let meta = extract_metadata_from_html(html, target_url, false).unwrap();

    assert_eq!(meta.title.as_deref(), Some("Página Vazia"));
    assert_eq!(meta.reading_time_min, 1);
    assert!(!meta.has_article);
    assert_eq!(meta.domain, "vazio.org");
}
