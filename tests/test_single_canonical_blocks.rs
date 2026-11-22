use joaju::compile_markdown_to_html;

#[test]
fn test_canonical_table_renders_and_legacy_aliases_ignored() {
    let md_canonical = "```table\na,b\n1,2\n```";
    let html_canonical = compile_markdown_to_html(md_canonical).expect("table should compile");
    assert!(html_canonical.contains("<table"), "Must render <table> tag");

    let md_legacy = "```csv\na,b\n1,2\n```";
    let html_legacy = compile_markdown_to_html(md_legacy).expect("legacy csv block");
    assert!(!html_legacy.contains("<table"), "Legacy alias csv must NOT render as table");
    assert!(html_legacy.contains("data-language=\"csv\""), "Must fallback to generic code block");
}

#[test]
fn test_canonical_pedigree_renders_and_wraps_in_diagram_block() {
    let md = "```pedigree\nid,pai,mae,sex\n1,0,0,1\n2,0,0,2\n3,1,2,1\n```";
    let html = compile_markdown_to_html(md).expect("pedigree should compile");
    assert!(html.contains("class=\"joaju-diagram-block joaju-pedigree-block\""));
    assert!(html.contains("data-engine=\"pedigree\""));
    assert!(html.contains("<svg"));

    let md_legacy = "```pedtools\nid,pai,mae,sex\n1,0,0,1\n2,0,0,2\n3,1,2,1\n```";
    let html_legacy = compile_markdown_to_html(md_legacy).expect("legacy pedtools block");
    assert!(!html_legacy.contains("joaju-pedigree-block"), "Legacy alias pedtools must NOT render as pedigree");
}

#[test]
fn test_canonical_chem_renders_and_wraps_in_diagram_block() {
    let md = "```chem\nCCO\n```";
    let html = compile_markdown_to_html(md).expect("chem should compile");
    assert!(html.contains("class=\"joaju-diagram-block joaju-chem-block\""));
    assert!(html.contains("data-engine=\"chem\""));
    assert!(html.contains("<svg"));

    let md_legacy = "```openchemlib\nCCO\n```";
    let html_legacy = compile_markdown_to_html(md_legacy).expect("legacy openchemlib block");
    assert!(!html_legacy.contains("joaju-chem-block"), "Legacy alias openchemlib must NOT render as chem");
}
