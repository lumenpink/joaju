use joaju::preprocessor::{
    preprocess_gists, preprocess_images, preprocess_tables, render_single_table,
};

#[test]
fn test_spec_render_abnt_three_line_table() {
    let input = r#"%% caption=População das Regiões do Brasil
%% source=IBGE (2022)
%% note=Dados censitários consolidados.
Região | População (hab.) | Percentual (%)
Norte | 17.349.000 | 8,5
Nordeste | 54.658.000 | 26,9
Sudeste | 84.847.000 | 41,8
Sul | 29.933.000 | 14,7
Centro-Oeste | 16.287.000 | 8,0"#;

    let typst = render_single_table(input).expect("Renderiza tabela ABNT");

    assert!(typst.contains("#figure("), "Deve gerar #figure");
    assert!(typst.contains("caption: [População das Regiões do Brasil]"));
    assert!(typst.contains("#table("));
    assert!(
        typst.contains("stroke: none"),
        "Tabela ABNT tem bordas laterais transparentes"
    );
    assert!(
        typst.contains("table.hline"),
        "Tabela ABNT possui as linhas mestras horizontais"
    );
    assert!(
        typst.contains("table.header("),
        "Cabeçalho com linha inferior"
    );
    assert!(
        typst.contains("Fonte: IBGE (2022)"),
        "Nota de rodapé da tabela com Fonte"
    );
    assert!(typst.contains("Nota: Dados censitários consolidados."));
}

#[test]
fn test_spec_preprocess_tables_in_markdown() {
    let markdown = r#"# Introdução

```table
%% caption=Tabela de Teste
A | B
1 | 2
```

Texto posterior."#;

    let processed = preprocess_tables(markdown);
    assert!(
        !processed.contains("```table"),
        "O bloco fence deve ser substituído por typst"
    );
    assert!(processed.contains("#figure("));
}

#[test]
fn test_spec_preprocess_obsidian_wikilinks_to_typst_images() {
    let markdown = r#"Veja o heredograma abaixo:
![[heredograma.png|450]]
E a foto do exame:
![[exame.jpg]]
E link comum:
![Ultrassom](figuras/usg.png)"#;

    let processed = preprocess_images(markdown);
    assert!(processed.contains(r#"#image("heredograma.png", width: 450pt)"#));
    assert!(processed.contains(r#"#image("exame.jpg")"#));
    assert!(processed.contains(r#"#image("figuras/usg.png")"#));
}

#[test]
fn test_mutation_table_csv_comma_support() {
    let csv_input = r#"%% caption=Tabela CSV
Nome,Idade,Status
Lumen,28,Ativo
Sam,24,Pendente"#;

    let typst = render_single_table(csv_input).expect("CSV delimitado por vírgula");
    assert!(typst.contains("Lumen"));
    assert!(typst.contains("Ativo"));
}

#[test]
fn test_spec_preprocess_gists_in_markdown() {
    let markdown = "Código:\n```gist\nfn main() {}\n```";
    let processed = preprocess_gists(markdown);
    assert!(!processed.contains("```gist"));
    assert!(processed.contains("fn main() {}"));
}
