use joaju::engine::pedigree::render_pedigree;

#[test]
fn test_spec_render_pedtools_fenced_block() {
    let input_code = r#"```pedtools
%% legend=auto
id,pai,mae,sex,fenotipo
1,0,0,M,wt
2,0,0,F,wt
3,1,2,F,hom
```"#;

    let result = render_pedigree(input_code, None);
    assert!(result.is_ok(), "Rendering failed: {:?}", result.err());

    let (format, svg) = result.unwrap();
    assert_eq!(format, "pedtools");
    assert!(svg.contains("<svg"), "Response must contain svg tag");
    assert!(svg.contains("</svg>"), "Response must close svg tag");
}

#[test]
fn test_spec_render_kinship2_direct_csv() {
    let input_code = r#"id,pai,mae,sex,fenotipo
1,0,0,M,wt
2,0,0,F,wt
3,1,2,M,het"#;

    let result = render_pedigree(input_code, Some("kinship2"));
    assert!(result.is_ok(), "Rendering failed: {:?}", result.err());

    let (format, svg) = result.unwrap();
    assert_eq!(format, "kinship2");
    assert!(svg.contains("<svg"));
}

#[test]
fn test_spec_render_invalid_pedigree_returns_error() {
    // Impossible genealogical cycle (1 is parent of 2, 2 is parent of 1)
    let input_code = r#"```pedtools
id,pai,mae,sex,fenotipo
1,2,0,M,wt
2,1,0,F,wt
```"#;

    let result = render_pedigree(input_code, None);
    assert!(result.is_err(), "Cyclic pedigree should return an error");
}

#[test]
fn test_mutation_render_with_twin_directives() {
    let input_code = r#"```pedtools
%% twin=3,4,mz
id,pai,mae,sex,fenotipo
1,0,0,M,wt
2,0,0,F,wt
3,1,2,M,wt
4,1,2,M,wt
```"#;

    let result = render_pedigree(input_code, None);
    assert!(result.is_ok(), "Rendering failed: {:?}", result.err());

    let (_, svg) = result.unwrap();
    assert!(
        svg.contains("<line"),
        "MZ twins should generate a connecting horizontal line"
    );
}
