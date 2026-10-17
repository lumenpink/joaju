use std::fs;
use std::path::Path;

#[test]
fn test_site_index_canonical_and_social_urls() {
    let html_path = Path::new("site/index.html");
    assert!(html_path.exists(), "site/index.html deve existir");

    let content = fs::read_to_string(html_path).expect("falha ao ler site/index.html");

    // Canonical link
    assert!(
        content.contains(r#"<link rel="canonical" href="https://joaju.2lp.in/">"#),
        "site/index.html deve ter canonical apontando para https://joaju.2lp.in/"
    );

    // OpenGraph
    assert!(
        content.contains(r#"<meta property="og:url" content="https://joaju.2lp.in/">"#),
        "site/index.html deve ter og:url apontando para https://joaju.2lp.in/"
    );
    assert!(
        content.contains(r#"<meta property="og:image" content="https://joaju.2lp.in/assets/og_preview.png">"#),
        "site/index.html deve ter og:image apontando para https://joaju.2lp.in/assets/og_preview.png"
    );

    // Twitter card
    assert!(
        content.contains(r#"<meta name="twitter:url" content="https://joaju.2lp.in/">"#),
        "site/index.html deve ter twitter:url apontando para https://joaju.2lp.in/"
    );
    assert!(
        content.contains(r#"<meta name="twitter:image" content="https://joaju.2lp.in/assets/og_preview.png">"#),
        "site/index.html deve ter twitter:image apontando para https://joaju.2lp.in/assets/og_preview.png"
    );
}

#[test]
fn test_site_no_legacy_domains() {
    let files_to_check = [
        "site/index.html",
        "site/style.css",
        "site/app.js",
        "README.md",
    ];

    let forbidden_patterns = ["joaju.local", "lohn.in", "obitwo"];

    for file_path in &files_to_check {
        let p = Path::new(file_path);
        if !p.exists() {
            continue;
        }
        let content = fs::read_to_string(p).unwrap_or_else(|_| panic!("falha ao ler {file_path}"));
        for pattern in &forbidden_patterns {
            assert!(
                !content.contains(pattern),
                "Arquivo {file_path} não deve conter a referência legada '{pattern}'"
            );
        }
    }
}

#[test]
fn test_site_style_no_orphan_playground() {
    let css_path = Path::new("site/style.css");
    assert!(css_path.exists(), "site/style.css deve existir");

    let content = fs::read_to_string(css_path).expect("falha ao ler site/style.css");

    assert!(
        !content.contains(".btn-playground"),
        "site/style.css não deve conter a classe descontinuada .btn-playground"
    );
}

#[test]
fn test_site_security_headers() {
    let headers_path = Path::new("site/_headers");
    assert!(headers_path.exists(), "site/_headers deve existir");

    let content = fs::read_to_string(headers_path).expect("falha ao ler site/_headers");
    assert!(
        content.contains("X-Content-Type-Options: nosniff"),
        "site/_headers deve conter X-Content-Type-Options"
    );
    assert!(
        content.contains("Content-Security-Policy:"),
        "site/_headers deve conter Content-Security-Policy"
    );
}
