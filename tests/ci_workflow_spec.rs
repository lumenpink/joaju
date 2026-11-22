use std::fs;

#[test]
fn test_ci_workflow_has_kombinat_notification_and_no_latest() {
    let ci_path = ".forgejo/workflows/ci.yml";
    let content = fs::read_to_string(ci_path).expect("ci.yml must exist");

    assert!(
        !content.contains("127.0.0.1:4040"),
        "ci.yml must not notify legacy Kombinat daemon on 4040"
    );

    assert!(
        content.contains("cargo publish --registry gambi"),
        "ci.yml must publish to Gambi registry"
    );

    assert!(
        !content.contains("joaju-static/latest/"),
        "ci.yml must not publish static packages to mutable 'latest' alias"
    );
}

#[test]
fn test_ci_workflow_stages_catalog_json_to_banca() {
    let ci_path = ".forgejo/workflows/ci.yml";
    let content = fs::read_to_string(ci_path).expect("ci.yml must exist");

    assert!(
        content.contains("site/downloads/catalog.json"),
        "ci.yml must stage site/downloads/catalog.json when committing to Banca"
    );
}

#[test]
fn test_ci_workflow_injects_full_version_in_build() {
    let ci_path = ".forgejo/workflows/ci.yml";
    let content = fs::read_to_string(ci_path).expect("ci.yml must exist");

    assert!(
        content.contains("JOAJU_FULL_VERSION"),
        "ci.yml must provide JOAJU_FULL_VERSION to compiler"
    );
}
