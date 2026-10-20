use joaju::compiler::{compile_markdown_to_pdf, convert_markdown_to_typst, preprocess_code_blocks};

#[test]
fn test_spec_compile_markdown_to_pdf() {
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

    let result = compile_markdown_to_pdf(input_markdown);
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 100);
    // PDF Magic Bytes: %PDF-
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[test]
fn test_spec_compile_empty_markdown_returns_bad_request() {
    let result = compile_markdown_to_pdf("   ");
    assert!(result.is_err(), "Empty markdown should return an error");
}

#[test]
fn test_spec_compile_markdown_with_pedigree_and_chem_to_pdf() {
    let input_markdown = r#"---
titulo: Estudo Genético e Farmacológico
autor: Laboratório Joaju
---

# Heredograma

```pedtools
1 0 0 1 0
2 0 0 2 0
3 1 2 1 1
```

# Molécula

```openchemlib
CCO
```
"#;

    let result = compile_markdown_to_pdf(input_markdown);
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 100);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[test]
fn test_spec_compile_trabalho_ufsc_multiline_instituicao_and_no_dashes() {
    let md = r#"---
titulo: ELABORAÇÃO DE HEREDOGRAMA FAMILIAR
autor: Lumen Müller Lohn Freitas
instituicao: |-
  UNIVERSIDADE FEDERAL DE SANTA CATARINA
  CENTRO DE CIÊNCIAS DA SAÚDE
  CURSO DE GRADUAÇÃO EM MEDICINA
perfil: trabalho-ufsc
ano: 2026
---

# INTRODUÇÃO

Texto introdutório.

---

# CAPÍTULO 2

Texto do capítulo 2.
"#;

    let typst = joaju::compiler::convert_markdown_to_typst(md);
    // Não deve conter linhas com travessões '---' isolados
    for line in typst.lines() {
        assert_ne!(
            line.trim(),
            "---",
            "Typst markup must not contain raw '---' horizontal rules"
        );
    }

    let result = compile_markdown_to_pdf(md);
    assert!(
        result.is_ok(),
        "Compilation with multiline instituicao failed: {:?}",
        result.err()
    );
    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 100);
}

#[test]
fn test_spec_compile_comparativo_pedtools_vs_kinship2_document() {
    let path =
        "/home/lumen/obsidian/01_inbox/202610121625 Teste Comparativo Pedtools vs Kinship2.md";
    let md = std::fs::read_to_string(path).expect("arquivo comparativo deve existir");
    let result = compile_markdown_to_pdf(&md);
    assert!(
        result.is_ok(),
        "Compilation of comparative test document failed: {:?}",
        result.err()
    );
    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 1000);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
    std::fs::write(
        "/home/lumen/obsidian/01_inbox/202610121625 Teste Comparativo Pedtools vs Kinship2.pdf",
        pdf_bytes,
    )
    .expect("escrita do PDF comparativo deve ter sucesso");
}

#[test]
fn test_spec_pagebreak_tags_converted_to_typst() {
    let md = r#"# Seção 1

Conteúdo da primeira página.

<pagebreak>

# Seção 2

Conteúdo da segunda página.

<!-- pagebreak -->

# Seção 3

Conteúdo da terceira página.

\pagebreak

Fim."#;

    let typst = convert_markdown_to_typst(md);
    let pagebreaks = typst.lines().filter(|l| l.trim() == "#pagebreak()").count();
    assert_eq!(
        pagebreaks, 3,
        "All 3 pagebreak variants must be converted to #pagebreak()"
    );
}

#[test]
fn test_spec_pedtools_code_block_height_and_scale_directives() {
    let md = r#"```pedtools
%% height=16.5cm scale=85%
1 0 0 1 0
2 0 0 2 0
3 1 2 1 1
```"#;

    let (typst_output, assets) = preprocess_code_blocks(md);
    assert_eq!(assets.len(), 1);
    assert!(
        typst_output.contains("height: 16.5cm"),
        "Typst image invocation must contain height: 16.5cm, got: {typst_output}"
    );
    assert!(
        typst_output.contains("width: 85%"),
        "Typst image invocation must contain width: 85%, got: {typst_output}"
    );
}
