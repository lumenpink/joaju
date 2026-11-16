use joaju::compiler::{compile_markdown_to_pdf, compile_typst_to_pdf};

#[cfg(not(feature = "pdf"))]
#[test]
fn test_pdf_feature_disabled_by_default_returns_descriptive_error() {
    let md = "# Titulo\nTexto comum.";
    let res = compile_markdown_to_pdf(md);
    assert!(res.is_err(), "compile_markdown_to_pdf should fail when pdf feature is disabled");
    let err_msg = res.unwrap_err();
    assert!(
        err_msg.to_lowercase().contains("pdf") && (err_msg.contains("disabled") || err_msg.contains("feature")),
        "Error message should mention that PDF compilation is disabled or requires the feature: {}",
        err_msg
    );

    let typst_res = compile_typst_to_pdf("= Titulo\nTexto");
    assert!(typst_res.is_err());
}
