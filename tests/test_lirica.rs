use joaju::compiler::preprocess_code_blocks;

#[test]
fn test_preprocess_lirica_block_replaces_with_typst() {
    let md = r#"# Meu Poema

```lirica
%% titulo = Soneto de Fidelidade
%% autor = Vinicius de Moraes
%% metro = decassilabo

De tudo, ao meu amor serei atento
Antes, e com tal zelo, e sempre, e tanto
Que mesmo em face do maior encanto
Dele se encante mais meu pensamento.
```

Fim do poema.
"#;

    let (processed, assets) = preprocess_code_blocks(md);

    // Lirica emits native Typst markup, so 0 image assets are generated
    assert!(
        assets.is_empty(),
        "Lirica should not generate binary image assets"
    );

    // The code block must be replaced with Typst markup
    assert!(!processed.contains("```lirica"));
    assert!(processed.contains("== Soneto de Fidelidade"));
    assert!(processed.contains("_por Vinicius de Moraes_"));
    assert!(processed.contains("Decassílabo Heroico"));
    assert!(processed.contains("#grid"));
}

#[test]
fn test_preprocess_lirica_with_single_verse() {
    let md = r#"```lirica
Minha terra tem palmeiras
```"#;

    let (processed, assets) = preprocess_code_blocks(md);
    assert!(assets.is_empty());
    assert!(!processed.contains("```lirica"));
    assert!(processed.contains("Redondilha Maior"));
}
