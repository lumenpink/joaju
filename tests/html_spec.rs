use std::ffi::{CStr, CString};
use std::ptr;

#[test]
fn test_compile_markdown_to_html_basic() {
    let md = "# Título Principal\n\nEste é um parágrafo com **negrito** e _itálico_.\n\n- Item 1\n- Item 2";
    let html = joaju::compile_markdown_to_html(md).expect("Falha ao compilar markdown para html");

    assert!(html.contains("<h1>Título Principal</h1>"));
    assert!(html.contains("<strong>negrito</strong>"));
    assert!(html.contains("<em>itálico</em>"));
    assert!(html.contains("<li>Item 1</li>"));
}

#[test]
fn test_compile_markdown_to_html_with_table() {
    let md = r#"# Demonstração de Tabela

```table
nome,versão
cccp,1.0.0
joaju,0.2.0
```
"#;
    let html = joaju::compile_markdown_to_html(md).expect("Falha ao compilar tabela joaju para html");

    assert!(html.contains("<table"));
    assert!(html.contains("cccp"));
    assert!(html.contains("1.0.0"));
    assert!(html.contains("joaju"));
}

#[test]
fn test_compile_markdown_to_html_with_pedigree_svg() {
    let md = r#"# Heredograma

```pedtools
id,dad,mom,sex
1,0,0,1
2,0,0,2
3,1,2,1
```
"#;
    let html = joaju::compile_markdown_to_html(md).expect("Falha ao compilar pedigree para html");

    assert!(html.contains("<svg"));
    assert!(html.contains("</svg>"));
}

#[test]
fn test_ffi_compile_markdown_to_html() {
    let md = CString::new("# Documento C-ABI\n\nTexto compilado via FFI.").unwrap();
    let mut out_html = ptr::null_mut();
    let mut out_err = ptr::null_mut();

    let rc = unsafe {
        joaju::ffi::joaju_compile_markdown_to_html(
            md.as_ptr(),
            &mut out_html,
            &mut out_err,
        )
    };

    assert_eq!(rc, 0);
    assert!(!out_html.is_null());
    assert!(out_err.is_null());

    let html_str = unsafe { CStr::from_ptr(out_html).to_str().unwrap() };
    assert!(html_str.contains("<h1>Documento C-ABI</h1>"));

    unsafe {
        joaju::ffi::joaju_free_string(out_html);
    }

    // Teste com ponteiro nulo
    let mut null_out = ptr::null_mut();
    let mut null_err = ptr::null_mut();
    let rc_null = unsafe {
        joaju::ffi::joaju_compile_markdown_to_html(
            ptr::null(),
            &mut null_out,
            &mut null_err,
        )
    };
    assert_eq!(rc_null, -1);
    assert!(!null_err.is_null());
    unsafe {
        joaju::ffi::joaju_free_string(null_err);
    }
}
