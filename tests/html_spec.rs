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

#[test]
fn test_cli_render_html_stdin_stdout() {
    use std::io::Write;
    use std::process::Command;

    let input = "# Título CLI\n\n```table\na,b\n1,2\n```\n";
    let mut child = Command::new(env!("CARGO_BIN_EXE_joaju"))
        .arg("render-html")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn joaju render-html");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(input.as_bytes()).unwrap();
    }

    let output = child.wait_with_output().expect("Failed to wait on child");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("<h1>Título CLI</h1>"));
    assert!(stdout.contains("<table"));

    // Teste de mutação: alias "html"
    let mut child_alias = Command::new(env!("CARGO_BIN_EXE_joaju"))
        .arg("html")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn joaju html");
    {
        let stdin = child_alias.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(b"# Alias Test\n").unwrap();
    }
    let out_alias = child_alias.wait_with_output().unwrap();
    assert!(out_alias.status.success());
    assert!(String::from_utf8_lossy(&out_alias.stdout).contains("<h1>Alias Test</h1>"));
}

#[test]
fn test_compile_markdown_to_html_chem_and_empty() {
    // Teste de mutação: entrada vazia
    let empty_html = joaju::compile_markdown_to_html("").unwrap();
    assert!(empty_html.trim().is_empty());

    // Teste de mutação: bloco de química
    let md_chem = "# Estrutura Química\n\n```chem\nCCO\n```\n";
    let chem_html = joaju::compile_markdown_to_html(md_chem).unwrap();
    assert!(chem_html.contains("<svg"));
    assert!(chem_html.contains("</svg>"));
}
