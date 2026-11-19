use joaju::{prepare_markdown, ReferenceNote, ReferenceSource};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn create_temp_dir(prefix: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), now));
    fs::create_dir_all(&dir).expect("Falha ao criar pasta de teste temporaria");
    dir
}

#[test]
fn test_prepare_markdown_none_references() {
    let raw = "Texto %%comentário%% com ==destaque== e [[link]]";
    let output = prepare_markdown(raw, None);

    assert!(!output.contains("%%comentário%%"));
    assert!(output.contains("<mark>destaque</mark>"));
    assert!(output.contains(r#"<a class="internal-link" data-href="link" href="link">link</a>"#));
}

#[test]
fn test_prepare_markdown_memory_references() {
    let raw = "Segundo @silva2024, a teoria é válida. %%comentário%%";

    let mut refs = HashMap::new();
    refs.insert(
        "silva2024".to_string(),
        ReferenceNote {
            citekey: "silva2024".to_string(),
            title: "Tratado de Testes".to_string(),
            authors: vec!["Silva, João".to_string()],
            year: Some(2024),
            doi: None,
            publisher: None,
            journal: None,
        },
    );

    let output = prepare_markdown(raw, Some(ReferenceSource::Memory(&refs)));

    assert!(!output.contains("%%comentário%%"));
    assert!(output.contains(r##"href="#ref-silva2024""##));
    assert!(output.contains("SILVA, 2024"));
    assert!(output.contains("Tratado de Testes"));
}

#[test]
fn test_prepare_markdown_from_into_conversion() {
    let raw = "Conforme [[kollontai1920]], novas relações emergem.";

    let mut refs = HashMap::new();
    refs.insert(
        "kollontai1920".to_string(),
        ReferenceNote {
            citekey: "kollontai1920".to_string(),
            title: "Relações Sociais".to_string(),
            authors: vec!["Kollontai, Alexandra".to_string()],
            year: Some(1920),
            doi: None,
            publisher: None,
            journal: None,
        },
    );

    let source: ReferenceSource = (&refs).into();
    let output = prepare_markdown(raw, Some(source));

    assert!(output.contains(r##"href="#ref-kollontai1920""##));
    assert!(output.contains("KOLLONTAI, 1920"));
    assert!(output.contains("Relações Sociais"));
}

#[test]
fn test_prepare_markdown_hybrid_resolution() {
    let temp_dir = create_temp_dir("joaju_hybrid");
    let refs_dir = temp_dir.join("refs");
    fs::create_dir_all(&refs_dir).unwrap();

    let disk_note =
        "---\ncitekey: smith1776\ntitle: Riqueza\nauthors: [\"Smith, Adam\"]\nyear: 1776\n---\n";
    fs::write(refs_dir.join("smith1776.md"), disk_note).unwrap();

    let mut mem_refs = HashMap::new();
    mem_refs.insert(
        "marx1867".to_string(),
        ReferenceNote {
            citekey: "marx1867".to_string(),
            title: "O Capital".to_string(),
            authors: vec!["Marx, Karl".to_string()],
            year: Some(1867),
            doi: None,
            publisher: None,
            journal: None,
        },
    );

    let raw = "Comparando @marx1867 e @smith1776 no mesmo texto.";
    let output = prepare_markdown(
        raw,
        Some(ReferenceSource::Hybrid {
            memory: &mem_refs,
            base_dir: Some(&temp_dir),
        }),
    );

    assert!(output.contains(r##"href="#ref-marx1867""##));
    assert!(output.contains("MARX, 1867"));
    assert!(output.contains(r##"href="#ref-smith1776""##));
    assert!(output.contains("SMITH, 1776"));
}

#[test]
fn test_compile_html_with_memory_source() {
    let mut refs = HashMap::new();
    refs.insert(
        "tatupeba2025".to_string(),
        ReferenceNote {
            citekey: "tatupeba2025".to_string(),
            title: "Tatupeba Engine".to_string(),
            authors: vec!["Pink, Lumen".to_string()],
            year: Some(2025),
            doi: None,
            publisher: None,
            journal: None,
        },
    );

    let raw = "# Título\n\nVer @tatupeba2025.";
    let html =
        joaju::compile_markdown_to_html_with_source(raw, Some(ReferenceSource::Memory(&refs)))
            .expect("Falha ao compilar para HTML com source");

    assert!(html.contains(r##"href="#ref-tatupeba2025""##));
    assert!(html.contains("PINK, 2025"));
}
