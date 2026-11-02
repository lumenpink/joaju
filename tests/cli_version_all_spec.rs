#[test]
fn test_ecosystem_inventory_contains_all_modules() {
    let inventory = joaju::ecosystem_inventory();
    assert!(!inventory.is_empty(), "Inventory must not be empty");

    let expected_modules = [
        "joaju-pdf",
        "joaju-chem",
        "joaju-pedigree",
        "joaju-table",
        "joaju-markdown",
        "joaju-templates",
        "joaju-lirica",
        "joaju-slides",
        "joaju-wireframe",
        "joaju-sequence",
        "joaju-graph",
    ];

    for module in &expected_modules {
        let found = inventory.iter().find(|(name, _)| name == module);
        assert!(
            found.is_some(),
            "Module {} must be present in ecosystem inventory",
            module
        );
        let (_, version) = found.unwrap();
        assert!(
            !version.is_empty(),
            "Version for {} cannot be empty",
            module
        );
    }
}
