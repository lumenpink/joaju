#[test]
fn test_landing_page_e_readme_modulos_visuais() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let site_html =
        std::fs::read_to_string(root.join("site/index.html")).expect("ler site/index.html");
    let readme = std::fs::read_to_string(root.join("README.md")).expect("ler README.md");

    let site_lower = site_html.to_lowercase();
    let readme_lower = readme.to_lowercase();

    // Proibição de chavões
    let proibidos = [
        "motor soberano em rust",
        "motor soberano",
    ];

    for termo in proibidos {
        assert!(
            !site_lower.contains(termo),
            "site/index.html ainda contém chavão proibido: '{termo}'"
        );
        assert!(
            !readme_lower.contains(termo),
            "README.md ainda contém chavão proibido: '{termo}'"
        );
    }

    // Presença das explicações visuais de cada ferramenta
    assert!(
        site_lower.contains("joaju-csv") && (site_lower.contains("tabelas") || site_lower.contains("tabela")),
        "site/index.html deve apresentar joaju-csv com foco em tabelas"
    );
    assert!(
        site_lower.contains("joaju-pedigree") && (site_lower.contains("diagramas") || site_lower.contains("heredogramas")),
        "site/index.html deve apresentar joaju-pedigree com foco em diagramas/heredogramas"
    );
    assert!(
        site_lower.contains("joaju-unfurl") && (site_lower.contains("cartões") || site_lower.contains("cartoes") || site_lower.contains("cartão")),
        "site/index.html deve apresentar joaju-unfurl com foco em cartões visuais"
    );

    assert!(
        readme_lower.contains("joaju-csv") && (readme_lower.contains("tabela") || readme_lower.contains("tables")),
        "README.md deve apresentar joaju-csv"
    );
    assert!(
        readme_lower.contains("joaju-pedigree") && (readme_lower.contains("diagram") || readme_lower.contains("heredograma")),
        "README.md deve apresentar joaju-pedigree"
    );
    assert!(
        readme_lower.contains("joaju-unfurl") && (readme_lower.contains("cartão") || readme_lower.contains("cartões") || readme_lower.contains("card")),
        "README.md deve apresentar joaju-unfurl"
    );
}
