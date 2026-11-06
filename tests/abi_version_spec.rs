use std::ffi::CStr;

#[test]
fn test_rust_version_constant() {
    assert!(!joaju::VERSION.is_empty(), "VERSION const cannot be empty");
    assert!(
        joaju::VERSION.starts_with("0.") || joaju::VERSION.starts_with("1."),
        "VERSION should start with SemVer major digit"
    );
}

#[test]
fn test_rust_version_matches_version_file_ssot() {
    let version_file_content = std::fs::read_to_string("VERSION")
        .expect("VERSION file must exist at repo root");
    let expected = version_file_content.trim();
    assert_eq!(
        joaju::VERSION,
        expected,
        "joaju::VERSION must match the canonical VERSION file SSOT"
    );
}

#[test]
fn test_c_abi_joaju_version() {
    let version_ptr = joaju::ffi::joaju_version();
    assert!(!version_ptr.is_null(), "C-ABI pointer must not be null");

    let c_str = unsafe { CStr::from_ptr(version_ptr) };
    let version_str = c_str
        .to_str()
        .expect("Version C-string must be valid UTF-8");

    assert_eq!(
        version_str,
        joaju::VERSION,
        "C-ABI version must match Rust VERSION constant"
    );
}

#[test]
fn test_c_abi_submodule_pdf_version() {
    let version_ptr = joaju::ffi::joaju_submodule_pdf_version();
    if !version_ptr.is_null() {
        let c_str = unsafe { CStr::from_ptr(version_ptr) };
        let version_str = c_str
            .to_str()
            .expect("Submodule pdf version must be valid UTF-8");
        assert!(!version_str.is_empty());
    }
}
