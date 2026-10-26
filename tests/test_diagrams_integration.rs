use joaju::compiler::{compile_markdown_to_pdf, preprocess_code_blocks};

#[test]
fn test_wireframe_salt_preprocessing() {
    let md = r#"# Exemplo de Wireframe

```wireframe
[ (X) Opção 1 | () Opção 2 ]
[ "Campo de Busca" | [ Buscar ] ]
```
"#;

    let (typst, assets) = preprocess_code_blocks(md);
    assert!(
        !typst.contains("```wireframe"),
        "Bloco ```wireframe deve ser pré-processado"
    );
    assert!(
        typst.contains("table(") || typst.contains("#box"),
        "Markup Typst deve conter elementos nativos de wireframe/salt"
    );
    assert!(
        assets.is_empty(),
        "Wireframe gera Typst inline, sem assets SVG binários"
    );
}

#[test]
fn test_salt_alias_preprocessing() {
    let md = r#"# Exemplo Salt

```salt
[ [ Salvar ] | [ Cancelar ] ]
```
"#;

    let (typst, assets) = preprocess_code_blocks(md);
    assert!(
        !typst.contains("```salt"),
        "Bloco ```salt deve ser pré-processado"
    );
    assert!(
        typst.contains("table(") || typst.contains("#box"),
        "Markup Typst de salt deve conter tabela ou caixas nativas"
    );
    assert!(assets.is_empty());
}

#[test]
fn test_sequence_preprocessing() {
    let md = r#"# Diagrama de Sequência

```sequence
Cliente -> Servidor: GET /api/v1/status
Servidor --> Cliente: 200 OK
```
"#;

    let (typst, assets) = preprocess_code_blocks(md);
    assert!(
        !typst.contains("```sequence"),
        "Bloco ```sequence deve ser pré-processado"
    );
    assert!(
        typst.contains("#image") && typst.contains("<svg"),
        "Markup Typst de sequência deve conter elemento de imagem SVG nativo"
    );
    assert!(assets.is_empty(), "Sequência gera Typst inline");
}

#[test]
fn test_flowchart_preprocessing() {
    let md = r#"# Fluxograma

```flowchart
A[Início] --> B[Processar]
B --> C[Fim]
```
"#;

    let (typst, assets) = preprocess_code_blocks(md);
    assert!(
        !typst.contains("```flowchart"),
        "Bloco ```flowchart deve ser pré-processado"
    );
    assert!(
        typst.contains("#image") && typst.contains("<svg"),
        "Markup Typst de fluxograma deve conter elemento de imagem SVG nativo"
    );
    assert!(assets.is_empty(), "Flowchart gera Typst inline");
}

#[test]
fn test_end_to_end_pdf_compilation_with_diagrams() {
    let md = r#"---
title: "Relatório de Arquitetura com Diagramas Nativos"
author: "Engenharia de Software"
date: "2026-10-26"
---

# Interface do Usuário

```wireframe
[ "Nome do Usuário" | [ Enviar ] ]
```

# Fluxo de Autenticação

```sequence
Usuario -> Auth: Login
Auth --> Usuario: Token JWT
```

# Topologia do Sistema

```flowchart
Frontend --> API
API --> Banco
```
"#;

    let pdf_result = compile_markdown_to_pdf(md);
    assert!(
        pdf_result.is_ok(),
        "Compilação E2E para PDF com diagramas nativos deve ter sucesso: {:?}",
        pdf_result.err()
    );

    let pdf_bytes = pdf_result.unwrap();
    assert!(
        pdf_bytes.starts_with(b"%PDF"),
        "Saída binária gerada deve ser um arquivo PDF válido"
    );
    assert!(
        pdf_bytes.len() > 1000,
        "PDF gerado deve ter tamanho substancial contendo as páginas e desenhos"
    );
}
