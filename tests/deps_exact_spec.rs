use std::fs;
use std::path::Path;

#[test]
fn test_package_version_is_canonical_dev() {
    let cargo_path = Path::new("Cargo.toml");
    let content = fs::read_to_string(cargo_path).expect("Failed to read Cargo.toml");

    let mut in_package = false;
    let mut pkg_version = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[package]" {
            in_package = true;
            continue;
        } else if trimmed.starts_with('[') {
            in_package = false;
        }

        if in_package && trimmed.starts_with("version") {
            if let Some((_, val)) = trimmed.split_once('=') {
                pkg_version = Some(val.trim().trim_matches('"').to_string());
                break;
            }
        }
    }

    let version = pkg_version.expect("package version must be present in Cargo.toml");
    assert!(
        version.contains("-dev"),
        "package.version in Cargo.toml must contain -dev pre-release identifier during development (got {version})"
    );
}

#[test]
fn test_all_internal_dependencies_have_exact_semver() {
    let cargo_path = Path::new("Cargo.toml");
    let content = fs::read_to_string(cargo_path).expect("Failed to read Cargo.toml");

    let internal_crates = [
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

    for crate_name in &internal_crates {
        let mut found = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(crate_name) {
                found = true;
                assert!(
                    trimmed.contains("version = \"="),
                    "Dependency '{crate_name}' must use exact version pinning with '=' (found: {trimmed})"
                );
                assert!(
                    trimmed.contains("registry = \"gambi\""),
                    "Dependency '{crate_name}' must declare registry = 'gambi' (found: {trimmed})"
                );
                break;
            }
        }
        assert!(found, "Internal crate '{crate_name}' must be listed in Cargo.toml");
    }
}
