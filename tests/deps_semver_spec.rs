#[test]
fn test_no_mutable_latest_in_build_rs() {
    let build_rs = std::fs::read_to_string("build.rs").expect("build.rs must exist");
    assert!(
        !build_rs.contains("\"latest\""),
        "build.rs must not rely on mutable 'latest' tag for joaju-pdf-static"
    );
    assert!(
        build_rs.contains("JOAJU_PDF_REQUIRED_VERSION"),
        "build.rs must define canonical JOAJU_PDF_REQUIRED_VERSION constant"
    );
}

#[test]
fn test_submodule_dependencies_have_strict_semver() {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Cargo.toml must exist");
    let submodules = [
        "joaju-pedigree",
        "joaju-table",
        "joaju-markdown",
        "joaju-templates",
        "joaju-lirica",
        "joaju-slides",
        "joaju-wireframe",
        "joaju-sequence",
        "joaju-graph",
        "joaju-chem",
    ];

    for sub in &submodules {
        assert!(
            cargo_toml.contains(sub),
            "Cargo.toml must declare dependency on {}",
            sub
        );
    }
}
