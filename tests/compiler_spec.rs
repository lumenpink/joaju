use joaju::compiler::compile_markdown_to_pdf;

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
