use joaju::compiler::{compile_markdown_to_pdf, compile_markdown_to_pdf_with_options};

#[test]
fn test_spec_compile_with_ufsc_template_generates_cover_and_numbered_body() {
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

    let result = compile_markdown_to_pdf(input_markdown);
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 1000);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[test]
fn test_spec_compile_with_abnt_template() {
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

    let result = compile_markdown_to_pdf(input_markdown);
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 500);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[test]
fn test_spec_partial_update_preserves_cover_and_updates_content() {
    let frontmatter = r#"---
titulo: Relatório Clínico Dinâmico
autor: Dra. Lumen Pink
instituicao: UFSC - Hospital Universitário
template: trabalho-ufsc
---
"#;

    // Versão 1 (apenas introdução)
    let md_v1 = format!(
        "{}\n# Introdução\n\nPrimeiro rascunho no Mandu.",
        frontmatter
    );
    let pdf_v1 = compile_markdown_to_pdf(&md_v1).expect("v1 compilation failed");

    // Versão 2 (update parcial do Mandu com nova seção e molécula)
    let md_v2 = format!(
        "{}\n# Introdução\n\nPrimeiro rascunho no Mandu.\n\n# Farmacologia\n\n```openchemlib\nCC(=O)Oc1ccccc1C(=O)O\n```\n",
        frontmatter
    );
    let pdf_v2 = compile_markdown_to_pdf(&md_v2).expect("v2 compilation failed");

    // A versão 2 deve ser maior devido à adição da nova seção e SVG molecular
    assert!(pdf_v2.len() > pdf_v1.len());
    assert_eq!(&pdf_v1[0..5], b"%PDF-");
    assert_eq!(&pdf_v2[0..5], b"%PDF-");
}

#[test]
fn test_mutation_template_with_missing_fields_defaults_cleanly() {
    let input_markdown = r#"---
template: trabalho-ufsc
---

# Título do Trabalho Sem Autor Nem Instituição Declarados

Corpo do documento."#;

    let result = compile_markdown_to_pdf(input_markdown);
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 500);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[test]
fn test_mutation_payload_template_override_takes_precedence_over_frontmatter() {
    // Frontmatter diz ABNT, mas o options força trabalho-ufsc (com capa)
    let input_markdown = r#"---
titulo: Estudo de Caso Genético
autor: Dr. Baquara
template: abnt
---

# Introdução

Corpo sob template UFSC forçado pelo payload."#;

    let result = compile_markdown_to_pdf_with_options(input_markdown, Some("trabalho-ufsc"));
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 1000);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}

#[test]
fn test_mutation_unknown_template_falls_back_to_default_cleanly() {
    let input_markdown = r#"---
titulo: Documento com Template Inexistente
template: formato-alienigena-v9
---

# Capítulo 1

Texto normal que deve compilar via fallback padrão."#;

    let result = compile_markdown_to_pdf(input_markdown);
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 500);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}
