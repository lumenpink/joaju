use std::fs;

#[test]
fn test_ci_workflow_segregates_dev_and_stable_aliases() {
    let ci_path = ".forgejo/workflows/ci.yml";
    let content = fs::read_to_string(ci_path).expect("ci.yml must exist");

    assert!(
        content.contains("PRIMARY_ALIAS=\"latest-dev\""),
        "ci.yml must configure PRIMARY_ALIAS as latest-dev for nightly channel"
    );
    assert!(
        content.contains("PRIMARY_ALIAS=\"latest\""),
        "ci.yml must configure PRIMARY_ALIAS as latest for stable channel"
    );
    assert!(
        content.contains("NAMED_ALIAS=\"${BASE_VERSION}-dev\""),
        "ci.yml must configure NAMED_ALIAS as BASE_VERSION-dev for nightly channel"
    );
    assert!(
        content.contains("NAMED_ALIAS=\"${BASE_VERSION}\""),
        "ci.yml must configure NAMED_ALIAS as BASE_VERSION for stable channel"
    );
    assert!(
        content.contains("-X DELETE") && content.contains("${PRIMARY_ALIAS}/${DEST_NAME}"),
        "ci.yml must clear PRIMARY_ALIAS with DELETE before PUT to prevent HTTP 409 Conflict"
    );
    assert!(
        content.contains("-X DELETE") && content.contains("${NAMED_ALIAS}/${DEST_NAME}"),
        "ci.yml must clear NAMED_ALIAS with DELETE before PUT to prevent HTTP 409 Conflict"
    );
}

#[test]
fn test_build_rs_requires_latest_pdf_canonical_version() {
    let build_content = fs::read_to_string("build.rs").expect("build.rs must exist");
    assert!(
        build_content.contains("const JOAJU_PDF_REQUIRED_VERSION: &str = \"0.1.0-dev.47\";"),
        "build.rs must require joaju-pdf staticlib version 0.1.0-dev.47"
    );
}
