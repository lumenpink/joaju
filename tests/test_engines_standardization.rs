use std::ffi::{CStr, CString};
use std::ptr;

#[test]
fn test_standardized_rust_api_reexports() {
    // 1. Testar reexportações de topo
    let lirica_res = joaju::render_lirica("%% titulo = Poema\nAmor é fogo que arde sem se ver");
    assert!(lirica_res.is_ok(), "joaju::render_lirica deve estar disponível");
    let lirica_out = lirica_res.unwrap();
    assert!(lirica_out.contains("== Poema"));

    let wireframe_res = joaju::render_wireframe("[Button]");
    assert!(wireframe_res.is_ok(), "joaju::render_wireframe deve estar disponível");

    let sequence_res = joaju::render_sequence("Alice -> Bob: Oi");
    assert!(sequence_res.is_ok(), "joaju::render_sequence deve estar disponível");

    let graph_res = joaju::render_graph("graph TD\n  A --> B");
    assert!(graph_res.is_ok(), "joaju::render_graph deve estar disponível");

    // 2. Testar acesso via submódulos dedicados
    assert!(joaju::lirica::render_lirica("teste").is_ok());
    assert!(joaju::wireframe::render_wireframe("teste").is_ok());
    assert!(joaju::sequence::render_sequence("teste").is_ok());
    assert!(joaju::graph::render_graph("teste").is_ok());
}

#[test]
fn test_standardized_ffi_exports() {
    // 1. joaju_render_lirica via FFI
    let c_code = CString::new("%% titulo = FFI Test\nMinha terra tem palmeiras").unwrap();
    let mut out_typst = ptr::null_mut();
    let mut out_error = ptr::null_mut();

    let rc = unsafe {
        joaju::ffi::joaju_render_lirica(c_code.as_ptr(), &mut out_typst, &mut out_error)
    };
    assert_eq!(rc, 0, "joaju_render_lirica deve retornar 0");
    assert!(!out_typst.is_null());
    let typst_str = unsafe { CStr::from_ptr(out_typst).to_str().unwrap() };
    assert!(typst_str.contains("FFI Test"));
    unsafe { joaju::ffi::joaju_free_string(out_typst) };

    // 2. joaju_render_wireframe via FFI
    let c_wire = CString::new("[OK]").unwrap();
    let mut out_wire = ptr::null_mut();
    let mut out_wire_err = ptr::null_mut();
    let rc_wire = unsafe {
        joaju::ffi::joaju_render_wireframe(c_wire.as_ptr(), &mut out_wire, &mut out_wire_err)
    };
    assert_eq!(rc_wire, 0, "joaju_render_wireframe deve retornar 0");
    assert!(!out_wire.is_null());
    unsafe { joaju::ffi::joaju_free_string(out_wire) };

    // 3. joaju_render_sequence via FFI
    let c_seq = CString::new("User -> Server: Request").unwrap();
    let mut out_seq = ptr::null_mut();
    let mut out_seq_err = ptr::null_mut();
    let rc_seq = unsafe {
        joaju::ffi::joaju_render_sequence(c_seq.as_ptr(), &mut out_seq, &mut out_seq_err)
    };
    assert_eq!(rc_seq, 0, "joaju_render_sequence deve retornar 0");
    assert!(!out_seq.is_null());
    unsafe { joaju::ffi::joaju_free_string(out_seq) };

    // 4. joaju_render_graph via FFI
    let c_graph = CString::new("graph TD\n  Start --> End").unwrap();
    let mut out_graph = ptr::null_mut();
    let mut out_graph_err = ptr::null_mut();
    let rc_graph = unsafe {
        joaju::ffi::joaju_render_graph(c_graph.as_ptr(), &mut out_graph, &mut out_graph_err)
    };
    assert_eq!(rc_graph, 0, "joaju_render_graph deve retornar 0");
    assert!(!out_graph.is_null());
    unsafe { joaju::ffi::joaju_free_string(out_graph) };
}

#[test]
fn test_mutation_ffi_null_pointers_and_error_handling() {
    let mut out_typst = ptr::null_mut();
    let mut out_error = ptr::null_mut();

    // 1. Lirica com ponteiro nulo deve retornar -1 e setar out_error
    let rc = unsafe { joaju::ffi::joaju_render_lirica(ptr::null(), &mut out_typst, &mut out_error) };
    assert_eq!(rc, -1);
    assert!(out_typst.is_null());
    assert!(!out_error.is_null());
    unsafe { joaju::ffi::joaju_free_string(out_error) };

    // 2. Wireframe com ponteiro nulo
    let mut out_err_wire = ptr::null_mut();
    let rc_wire = unsafe {
        joaju::ffi::joaju_render_wireframe(ptr::null(), &mut out_typst, &mut out_err_wire)
    };
    assert_eq!(rc_wire, -1);
    assert!(!out_err_wire.is_null());
    unsafe { joaju::ffi::joaju_free_string(out_err_wire) };

    // 3. Sequence com ponteiro nulo
    let mut out_err_seq = ptr::null_mut();
    let rc_seq = unsafe {
        joaju::ffi::joaju_render_sequence(ptr::null(), &mut out_typst, &mut out_err_seq)
    };
    assert_eq!(rc_seq, -1);
    assert!(!out_err_seq.is_null());
    unsafe { joaju::ffi::joaju_free_string(out_err_seq) };

    // 4. Graph com ponteiro nulo
    let mut out_err_graph = ptr::null_mut();
    let rc_graph = unsafe {
        joaju::ffi::joaju_render_graph(ptr::null(), &mut out_typst, &mut out_err_graph)
    };
    assert_eq!(rc_graph, -1);
    assert!(!out_err_graph.is_null());
    unsafe { joaju::ffi::joaju_free_string(out_err_graph) };
}
