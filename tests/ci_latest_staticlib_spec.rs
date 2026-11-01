//! Test ensuring CI publishes joaju-static under latest and build.rs defaults to latest.

#[test]
fn test_ci_workflow_publishes_latest_staticlib() {
    let ci_yaml = std::fs::read_to_string(".forgejo/workflows/ci.yml").expect("ci.yml must exist");
    assert!(
        ci_yaml.contains("generic/joaju-static/latest/${DEST_NAME}"),
        "CI workflow must publish joaju-static under latest alias"
    );
}

#[test]
fn test_build_rs_defaults_to_latest() {
    let build_rs = std::fs::read_to_string("build.rs").expect("build.rs must exist");
    assert!(
        build_rs.contains("JOAJU_PDF_STATIC_VERSION"),
        "build.rs must support JOAJU_PDF_STATIC_VERSION"
    );
    assert!(
        build_rs.contains("latest"),
        "build.rs must default to latest for staticlib resolution"
    );
}
