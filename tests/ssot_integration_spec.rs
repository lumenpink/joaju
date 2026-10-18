use joaju::compiler::compile_markdown_to_pdf;
use std::fs;

#[test]
fn test_ssot_single_csv_feeding_pedtools_and_table() {
    let csv_content = r#"id,pai,mae,sex,geracao,fenotipo_abo,proband,label_abo
1,0,0,M,1,B,0,I-1 (Avô)
2,0,0,F,1,A,0,I-2 (Avó)
3,1,2,F,2,AB,1,II-1 (Probanda)
4,1,2,M,2,O,0,II-2 (Irmão)
5,0,0,F,3,A,0,III-1 (Neta)
"#;
    let tmp_dir = std::env::temp_dir();
    let csv_path = tmp_dir.join("joaju_ssot_familia.csv");
    fs::write(&csv_path, csv_content).expect("falha ao criar CSV de teste");

    let md = format!(
        r#"# Relatório de Genética Médica

Este relatório utiliza uma Fonte Única de Dados (SSOT).

## Heredograma do Sistema ABO

```pedtools
%% src={}
%% select=id, pai, mae, sex, fenotipo_abo as fenotipo, proband, label_abo as label
%% where=geracao <= 2
%% layout.width=90%
```

## Tabela Genealógica

```table
%% src={}
%% title=Tabela 1 – Fenótipos ABO das Primeiras Gerações
%% note=Fonte: Elaborado pelos autores a partir do SSOT (2026).
%% select=id, pai, mae, sex, fenotipo_abo
%% where=geracao <= 2
```
"#,
        csv_path.display(),
        csv_path.display()
    );

    let (processed_md, assets) = joaju::compiler::preprocess_code_blocks(&md);
    assert_eq!(
        assets.len(),
        1,
        "Deve gerar 1 asset SVG para o heredograma pedtools via SSOT"
    );
    assert!(processed_md.contains("#image(\"asset_ped_0.svg\")"));

    let pdf_bytes =
        compile_markdown_to_pdf(&md).expect("falha ao compilar markdown com SSOT para PDF");
    assert!(!pdf_bytes.is_empty(), "PDF gerado não deve ser vazio");
    assert!(
        pdf_bytes.starts_with(b"%PDF-"),
        "Saída deve ser um documento PDF válido"
    );
    assert!(
        pdf_bytes.len() > 3000,
        "PDF deve conter conteúdo estruturado"
    );

    let _ = fs::remove_file(csv_path);
}

#[test]
fn test_ssot_with_official_pedigree_headers() {
    let csv_content = r#"id,fid,mid,sex,geracao,aff,proband
1,0,0,1,1,1,0
2,0,0,2,1,1,0
3,1,2,2,2,2,1
"#;
    let tmp_dir = std::env::temp_dir();
    let csv_path = tmp_dir.join("joaju_ssot_official.csv");
    fs::write(&csv_path, csv_content).expect("falha ao criar CSV oficial de teste");

    let md = format!(
        r#"# Análise de Pedigree Oficial

```pedtools
%% src={}
%% select=id, fid as pai, mid as mae, sex, aff as fenotipo, proband
%% where=geracao <= 2
```
"#,
        csv_path.display()
    );

    let pdf_bytes =
        compile_markdown_to_pdf(&md).expect("falha ao compilar com cabeçalhos oficiais");
    assert!(pdf_bytes.starts_with(b"%PDF-"));

    let _ = fs::remove_file(csv_path);
}

#[test]
fn test_ssot_combined_text_and_numeric_filters() {
    let csv_content = r#"id,pai,mae,sex,geracao,status,fenotipo
1,0,0,M,1,ativo,wt
2,0,0,F,1,ativo,wt
3,1,2,M,2,inativo,afetado
4,1,2,F,2,ativo,afetado
"#;
    let tmp_dir = std::env::temp_dir();
    let csv_path = tmp_dir.join("joaju_ssot_combined.csv");
    fs::write(&csv_path, csv_content).expect("falha ao criar CSV combinado");

    let md = format!(
        r#"# Estudo com Filtros Compostos

```pedtools
%% src={}
%% select=id, pai, mae, sex, fenotipo
%% where=geracao <= 2 and status == ativo
```
"#,
        csv_path.display()
    );

    let (processed_md, assets) = joaju::compiler::preprocess_code_blocks(&md);
    assert_eq!(assets.len(), 1);
    assert!(processed_md.contains("#image(\"asset_ped_0.svg\")"));

    let _ = fs::remove_file(csv_path);
}
