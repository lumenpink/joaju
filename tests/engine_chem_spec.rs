use joaju::engine::chem::render_chem;

#[test]
fn test_spec_render_openchemlib_fenced_block() {
    // Aspirin
    let input_code = r#"```openchemlib
CC(=O)Oc1ccccc1C(=O)O
```"#;

    let result = render_chem(input_code, None);
    assert!(result.is_ok(), "Rendering failed: {:?}", result.err());

    let (format, svg) = result.unwrap();
    assert_eq!(format, "openchemlib");
    assert!(svg.contains("<svg"), "Response must contain svg tag");
    assert!(svg.contains("</svg>"), "Response must close svg tag");
}

#[test]
fn test_spec_render_openbabel_direct_smiles() {
    // Caffeine
    let input_code = "Cn1cnc2c1c(=O)n(c(=O)n2C)C";

    let result = render_chem(input_code, Some("openbabel"));
    assert!(result.is_ok(), "Rendering failed: {:?}", result.err());

    let (format, svg) = result.unwrap();
    assert_eq!(format, "openbabel");
    assert!(svg.contains("<svg"));
}

#[test]
fn test_spec_render_invalid_smiles_returns_error() {
    let input_code = r#"```openchemlib
SMILES_TOTALMENTE_INVALIDO_XYZ!!!
```"#;

    let result = render_chem(input_code, None);
    assert!(result.is_err(), "Invalid SMILES should return an error");
}

#[test]
fn test_mutation_estradiol_structure() {
    // Estradiol with caption directive
    let input_code = r#"```openchemlib
%% caption=Estradiol
C[C@]12CC[C@H]3[C@H]([C@@H]1CC[C@@H]2O)CCC4=C3C=CC(=C4)O
```"#;

    let result = render_chem(input_code, None);
    assert!(result.is_ok(), "Rendering failed: {:?}", result.err());

    let (_, svg) = result.unwrap();
    assert!(svg.contains("<svg"));
}
