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
fn test_references_in_root_refs_dir() {
    let dir = create_temp_dir("joaju_refs_root");
    let refs_dir = dir.join("refs");
    fs::create_dir_all(&refs_dir).expect("Criar refs/");

    fs::write(
        refs_dir.join("silva2024.md"),
        r#"---
title: "Inteligência Artificial na Prática"
authors:
  - "[[Silva, João]]"
year: 2024
citekey: silva2024
---
# Resumo da nota
"#,
    )
    .expect("Gravar silva2024.md");

    fs::write(
        refs_dir.join("santos2023.md"),
        r#"---
title: "Redes Neurais Modernas"
authors:
  - "[[Santos, Maria]]"
year: 2023
citekey: santos2023
---
# Resumo da nota
"#,
    )
    .expect("Gravar santos2023.md");

    let markdown = r#"---
title: "Artigo de Exemplo"
---
Segundo @silva2024 e [[santos2023]], o aprendizado profundo evoluiu rapidamente.
"#;

    let html = joaju::compile_markdown_to_html_with_base_dir(markdown, Some(&dir))
        .expect("Falha ao compilar markdown com referencias para html");

    assert!(
        html.contains("SILVA, 2024") || html.contains("Silva, 2024"),
        "HTML deve conter citacao de Silva: {}",
        html
    );
    assert!(
        html.contains("SANTOS, 2023") || html.contains("Santos, 2023"),
        "HTML deve conter citacao de Santos: {}",
        html
    );
    assert!(
        html.contains("href=\"#ref-silva2024\""),
        "HTML deve conter link ancorado para silva2024"
    );

    assert!(
        html.contains("Referências"),
        "HTML deve conter cabecalho de Referencias"
    );
    assert!(
        html.contains("Inteligência Artificial na Prática"),
        "HTML deve conter titulo de Silva"
    );
    assert!(
        html.contains("Redes Neurais Modernas"),
        "HTML deve conter titulo de Santos"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_references_in_subfolders() {
    let dir = create_temp_dir("joaju_refs_sub");
    let artigos_dir = dir.join("refs").join("artigos");
    let livros_dir = dir.join("refs").join("livros");
    fs::create_dir_all(&artigos_dir).expect("Criar refs/artigos/");
    fs::create_dir_all(&livros_dir).expect("Criar refs/livros/");

    fs::write(
        artigos_dir.join("kollontai1920.md"),
        r#"---
title: "A Nova Mulher e a Moral Sexual"
authors:
  - "[[Kollontai, Alexandra]]"
year: 1920
citekey: kollontai1920
---
"#,
    )
    .expect("Gravar kollontai1920.md");

    fs::write(
        livros_dir.join("luxemburgo1900.md"),
        r#"---
title: "Reforma ou Revolução"
authors:
  - "[[Luxemburgo, Rosa]]"
year: 1900
citekey: luxemburgo1900
---
"#,
    )
    .expect("Gravar luxemburgo1900.md");

    let markdown = r#"---
title: "Teoria Política"
---
Conforme apontado por [[kollontai1920]] e [@luxemburgo1900, p. 15].
"#;

    let html = joaju::compile_markdown_to_html_with_base_dir(markdown, Some(&dir))
        .expect("Falha ao compilar markdown com subpastas de refs");

    assert!(
        html.contains("KOLLONTAI, 1920"),
        "HTML deve conter citacao de Kollontai: {}",
        html
    );
    assert!(
        html.contains("LUXEMBURGO, 1900, p. 15"),
        "HTML deve conter citacao de Luxemburgo com localizador: {}",
        html
    );
    assert!(
        html.contains("A Nova Mulher e a Moral Sexual"),
        "HTML deve conter titulo na bibliografia"
    );
    assert!(
        html.contains("Reforma ou Revolução"),
        "HTML deve conter titulo na bibliografia"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_references_custom_refs_dir_and_numeric_style() {
    let dir = create_temp_dir("joaju_refs_custom");
    let custom_dir = dir.join("minha_bibliografia");
    fs::create_dir_all(&custom_dir).expect("Criar minha_bibliografia/");

    fs::write(
        custom_dir.join("silva2024.md"),
        r#"---
title: "Sistemas Embarcados Robustos"
authors:
  - "[[Silva, João]]"
year: 2024
citekey: silva2024
---
"#,
    )
    .expect("Gravar silva2024.md");

    let markdown = r#"---
title: "Engenharia de Software"
refs_dir: "minha_bibliografia"
estilo: numerico
---
Estudos anteriores [@silva2024] demonstraram estabilidade.
"#;

    let html = joaju::compile_markdown_to_html_with_base_dir(markdown, Some(&dir))
        .expect("Falha ao compilar com estilo numerico");

    assert!(
        html.contains("[1]"),
        "HTML deve conter citacao numerica [1]: {}",
        html
    );
    assert!(
        html.contains("Sistemas Embarcados Robustos"),
        "HTML deve conter titulo da referencia numerica: {}",
        html
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_references_fallback_renamed_file() {
    let dir = create_temp_dir("joaju_refs_renamed");
    let refs_dir = dir.join("refs");
    fs::create_dir_all(&refs_dir).expect("Criar refs/");

    fs::write(
        refs_dir.join("Silva (2024) - Estudo Sobre Compiladores.md"),
        r#"---
title: "Estudo Sobre Compiladores"
authors:
  - "[[Silva, João]]"
year: 2024
citekey: silva2024
---
"#,
    )
    .expect("Gravar arquivo renomeado");

    let markdown = r#"---
title: "Artigo de Compiladores"
---
Conforme visto em [[silva2024]].
"#;

    let html = joaju::compile_markdown_to_html_with_base_dir(markdown, Some(&dir))
        .expect("Falha ao compilar com fallback de arquivo renomeado");

    assert!(
        html.contains("SILVA, 2024"),
        "HTML deve resolver pelo frontmatter mesmo com arquivo renomeado: {}",
        html
    );
    assert!(
        html.contains("Estudo Sobre Compiladores"),
        "HTML deve conter titulo na bibliografia: {}",
        html
    );

    let _ = fs::remove_dir_all(&dir);
}
