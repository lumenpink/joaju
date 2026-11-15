use joaju::compile_markdown_to_html;

#[test]
fn test_compile_markdown_to_html_with_lirica() {
    let md = r#"# Poema Teste

```lirica
%% titulo = Soneto de Fidelidade
De tudo, ao meu amor serei atento
Antes, e com tal zelo, e sempre, e tanto
```
"#;
    let html = compile_markdown_to_html(md).expect("Falha ao compilar lírica para html");

    assert!(html.contains("joaju-lirica-card"), "Deve conter o container de cartão lírico");
    assert!(html.contains("joaju-card-badge"), "Deve conter o badge de lírica");
    assert!(html.contains("De tudo, ao meu amor serei atento"), "Deve conter os versos originais");
    assert!(html.contains("joaju-scansion"), "Deve conter o bloco de escansão");
}

#[test]
fn test_compile_markdown_to_html_with_wireframe() {
    let md = r#"# Protótipo

```wireframe
[ Salvar ] [ Cancelar ]
```
"#;
    let html = compile_markdown_to_html(md).expect("Falha ao compilar wireframe para html");

    assert!(html.contains("joaju-diagram-block"), "Deve conter o contêiner de diagrama");
    assert!(html.contains("data-engine=\"wireframe\""), "Deve indicar o motor wireframe");
    assert!(html.contains("<svg") && html.contains("</svg>"), "Deve conter SVG gerado");
}

#[test]
fn test_compile_markdown_to_html_with_sequence() {
    let md = r#"# Fluxo

```sequence
Alice -> Bob: Olá
Bob -> Alice: Oi
```
"#;
    let html = compile_markdown_to_html(md).expect("Falha ao compilar sequence para html");

    assert!(html.contains("joaju-diagram-block"), "Deve conter o contêiner de diagrama");
    assert!(html.contains("data-engine=\"sequence\""), "Deve indicar o motor sequence");
    assert!(html.contains("<svg") && html.contains("</svg>"), "Deve conter SVG gerado");
}

#[test]
fn test_compile_markdown_to_html_with_graph() {
    let md = r#"# Grafo

```graph
A -> B
B -> C
```
"#;
    let html = compile_markdown_to_html(md).expect("Falha ao compilar grafo para html");

    assert!(html.contains("joaju-diagram-block"), "Deve conter o contêiner de diagrama");
    assert!(html.contains("data-engine=\"graph\""), "Deve indicar o motor graph");
    assert!(html.contains("<svg") && html.contains("</svg>"), "Deve conter SVG gerado");
}

#[test]
fn test_compile_markdown_to_html_with_code_card() {
    let md = r#"# Código Rust

```rust
fn main() {
    println!("Olá!");
}
```
"#;
    let html = compile_markdown_to_html(md).expect("Falha ao compilar código para html");

    assert!(html.contains("joaju-code-block"), "Deve envelopar em joaju-code-block");
    assert!(html.contains("data-language=\"rust\""), "Deve indicar data-language=rust");
    assert!(html.contains("joaju-code-badge"), "Deve conter o badge de linguagem");
    assert!(html.contains("language-rust"), "Deve manter classe language-rust");
    assert!(html.contains("println!"), "Deve conter o código interno");
}

#[test]
fn test_compile_markdown_to_html_with_typst_card() {
    let md = r#"# Typst Bloco

```typst
#let x = 10
#align(center)[Typst]
```
"#;
    let html = compile_markdown_to_html(md).expect("Falha ao compilar typst para html");

    assert!(html.contains("joaju-code-block"), "Deve envelopar em joaju-code-block");
    assert!(html.contains("joaju-typst-block"), "Deve identificar classe joaju-typst-block");
    assert!(html.contains("data-language=\"typst\""), "Deve indicar data-language=typst");
}

#[test]
fn test_compile_markdown_to_html_with_semantic_figure() {
    let md = r#"# Figura Acadêmica

![Esquema da Arquitetura](https://exemplo.com/diagrama.png)
"#;
    let html = compile_markdown_to_html(md).expect("Falha ao compilar figura para html");

    assert!(html.contains("<figure class=\"joaju-figure\">"), "Deve converter em <figure>");
    assert!(html.contains("<img src=\"https://exemplo.com/diagrama.png\" alt=\"Esquema da Arquitetura\""), "Deve conter img");
    assert!(html.contains("<figcaption class=\"joaju-figcaption\">Esquema da Arquitetura</figcaption>"), "Deve conter figcaption");
}

#[test]
fn test_compile_markdown_to_html_edge_cases_and_fallbacks() {
    // 1. Imagem sem legenda: não deve criar figcaption vazio
    let md_no_alt = "![](https://exemplo.com/sem_legenda.png)";
    let html_no_alt = compile_markdown_to_html(md_no_alt).expect("Falha ao compilar imagem sem alt");
    assert!(!html_no_alt.contains("<figcaption>"), "Imagem sem alt não deve ter figcaption");
    assert!(html_no_alt.contains("<img src=\"https://exemplo.com/sem_legenda.png\""));

    // 2. Lírica vazia / inválida: não deve panicar
    let md_empty_lirica = "```lirica\n```";
    let html_empty_lirica = compile_markdown_to_html(md_empty_lirica).expect("Falha ao compilar lirica vazia");
    assert!(!html_empty_lirica.is_empty());
}
