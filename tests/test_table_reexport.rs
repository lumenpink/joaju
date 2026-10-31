//! Red test: Garante que joaju reexporta formalmente joaju_table como table.

#[test]
fn test_table_reexport_available() {
    let raw = "col1,col2\nval1,val2\n";
    let res = joaju::table::render_table_block(raw);
    assert!(
        res.is_ok(),
        "Falha ao invocar joaju::table::render_table_block"
    );
}
