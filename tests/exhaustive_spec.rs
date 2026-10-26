use joaju::compiler::compile_markdown_to_pdf;
use joaju::engine::chem::render_chem;
use joaju::engine::pedigree::render_pedigree;
use joaju::preprocessor::table::render_single_table;

// =========================================================================
// 1. MOTOR DE HEREDOGRAMAS CLÍNICOS (PEDIGREE) - CASOS LIMÍTROFES
// =========================================================================

#[test]
fn test_edge_pedigree_multiple_twins_and_proband() {
    // Fratria com gêmeos múltiplos e probando
    let input = r#"
%% proband 4
%% twin 4,5
%% width 800
%% legend true
1 0 0 1 0
2 0 0 2 0
3 1 2 1 0
4 1 2 2 1
5 1 2 2 1
"#;
    let (fmt, svg) = render_pedigree(input, Some("pedtools")).unwrap();
    assert_eq!(fmt, "pedtools");
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
}

#[test]
fn test_edge_pedigree_consanguineous_mating_loop() {
    // Casamento entre primos (consanguinidade)
    let input = r#"
1 0 0 1 0
2 0 0 2 0
3 1 2 1 0
4 1 2 2 0
5 3 0 1 0
6 0 4 2 0
7 5 6 1 1
"#;
    let (fmt, svg) = render_pedigree(input, Some("pedtools")).unwrap();
    assert_eq!(fmt, "pedtools");
    assert!(svg.contains("<svg"));
}

#[test]
fn test_edge_pedigree_empty_and_whitespace_only() {
    let empty = "    \n\n   \t  \n";
    assert!(render_pedigree(empty, None).is_err());
}

#[test]
fn test_edge_pedigree_malformed_columns() {
    // Linha com número insuficiente de colunas
    let input = "1 0 0 1\n2 0";
    assert!(render_pedigree(input, None).is_err());
}

// =========================================================================
// 2. MOTOR DE QUÍMICA ESTRUTURAL 2D (CHEM) - CASOS LIMÍTROFES
// =========================================================================

#[test]
fn test_edge_chem_complex_polycyclic_caffeine() {
    // Cafeína (1,3,7-trimetilxantina): anéis fundidos aromáticos com heteroátomos
    let smiles = "CN1C=NC2=C1C(=O)N(C(=O)N2C)C";
    let (fmt, svg) = render_chem(smiles, Some("openchemlib")).unwrap();
    assert_eq!(fmt, "openchemlib");
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
}

#[test]
fn test_edge_chem_chiral_centers_amino_acid() {
    // L-Alanina com centro quiral explícito
    let smiles = "N[C@@H](C)C(=O)O";
    let (fmt, svg) = render_chem(smiles, Some("openbabel")).unwrap();
    assert_eq!(fmt, "openbabel");
    assert!(svg.contains("<svg"));
}

#[test]
fn test_edge_chem_unbalanced_and_invalid_smiles() {
    // Parênteses desbalanceados e anel não-fechado
    assert!(render_chem("CC(C", None).is_err());
    assert!(render_chem("C1CCCCC", None).is_err());
    assert!(render_chem("   ", None).is_err());
}

// =========================================================================
// 3. PRÉ-PROCESSADORES (TABELAS ABNT E WIKILINKS) - CASOS LIMÍTROFES
// =========================================================================

#[test]
fn test_edge_table_ragged_rows_and_special_chars() {
    // Tabela com número de colunas irregular e caracteres com aspas/cerquilha
    let input = r#"
%% caption=Tabela de Marcadores #1 (ABNT)
%% source=Laboratório de Genômica "Joaju"
Marcador | Alelo A | Alelo B | Frequência
D1S80 | 18 | 24 | 0.45
D2S1338 | 19 | 0.82
D3S1358 | 15 | 16
"#;
    let result = render_single_table(input).unwrap();
    assert!(result.contains("#figure("));
    assert!(result.contains("Marcadores #1"));
    assert!(result.contains("Laboratório de Genômica"));
}

#[test]
fn test_edge_wikilinks_with_spaces_and_accents() {
    let input = "Veja o laudo em ![[Relatório Clínico - Genética Médica 2026.png|450]].";
    let processed = joaju::preprocessor::image::preprocess_images(input);
    assert!(processed
        .contains(r#"#image("Relatório Clínico - Genética Médica 2026.png", width: 450pt)"#));
}

// =========================================================================
// 5. COMPILAÇÃO INTEGRAL MULTI-MOTOR (TESTE DE INTEGRAÇÃO EXAUSTIVO)
// =========================================================================

#[test]
fn test_edge_exhaustive_full_academic_document_compilation() {
    // Documento completo combinando todos os subsistemas em um único documento
    let full_document = r#"---
titulo: Estudo Multidisciplinar de Genética e Farmacogenômica
subtitulo: Análise Genealógica e Estrutural de Inibidores
autor: Dra. Lumen Pink
instituicao: Universidade Federal de Santa Catarina
local: Florianópolis
ano: 2026
template: trabalho-ufsc
---

# Introdução

Este documento constitui o teste exaustivo de integração de todos os subsistemas nativos do Joaju.

# Heredograma da Família Investigada

Abaixo apresentamos o heredograma clínico com afecção autossômica dominante.

```pedtools
%% proband 3
%% width 650
1 0 0 1 0
2 0 0 2 0
3 1 2 1 1
4 1 2 2 0
```

# Estrutura Química do Composto Terapêutico

A molécula de ácido acetilsalicílico foi sintetizada e avaliada:

```openchemlib
CC(=O)Oc1ccccc1C(=O)O
```

# Resultados Experimentais

Os achados laboratoriais estão sintetizados na tabela ABNT de 3 linhas a seguir:

```table
%% caption=Parâmetros Farmacocinéticos Observados
%% source=Laboratório Joaju / UFSC
Parâmetro | Unidade | Média | Desvio Padrão
Depuração | mL/min | 14.5 | 1.2
Meia-vida | h | 3.2 | 0.4
Biodisponibilidade | % | 88.0 | 3.5
```

# Conclusão

Todos os componentes vetoriais foram compilados in-process com fidelidade tipográfica estrita.
"#;

    let result = compile_markdown_to_pdf(full_document);
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    let pdf_bytes = result.unwrap();
    assert!(pdf_bytes.len() > 5000);
    assert_eq!(&pdf_bytes[0..5], b"%PDF-");
}
