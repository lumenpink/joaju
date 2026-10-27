use std::ffi::{CStr, CString};
use std::ptr;

#[test]
fn test_ffi_chem_render() {
    let input = CString::new("CCO").unwrap();
    let mut out_fmt = ptr::null_mut();
    let mut out_svg = ptr::null_mut();
    let mut out_err = ptr::null_mut();

    let rc = unsafe {
        joaju::ffi::joaju_render_chem(
            input.as_ptr(),
            ptr::null(),
            &mut out_fmt,
            &mut out_svg,
            &mut out_err,
        )
    };

    assert_eq!(rc, 0);
    assert!(!out_fmt.is_null());
    assert!(!out_svg.is_null());
    assert!(out_err.is_null());

    let fmt_str = unsafe { CStr::from_ptr(out_fmt).to_str().unwrap() };
    let svg_str = unsafe { CStr::from_ptr(out_svg).to_str().unwrap() };
    assert_eq!(fmt_str, "openchemlib");
    assert!(svg_str.contains("<svg"));

    unsafe {
        joaju::ffi::joaju_free_string(out_fmt);
        joaju::ffi::joaju_free_string(out_svg);
    }
}

#[test]
fn test_ffi_pedigree_render() {
    let input = CString::new("id,pai,mae,sex\n1,0,0,1\n2,0,0,2\n3,1,2,1\n").unwrap();
    let mut out_fmt = ptr::null_mut();
    let mut out_svg = ptr::null_mut();
    let mut out_err = ptr::null_mut();

    let rc = unsafe {
        joaju::ffi::joaju_render_pedigree(
            input.as_ptr(),
            ptr::null(),
            &mut out_fmt,
            &mut out_svg,
            &mut out_err,
        )
    };

    assert_eq!(rc, 0);
    assert!(!out_fmt.is_null());
    assert!(!out_svg.is_null());
    assert!(out_err.is_null());

    let fmt_str = unsafe { CStr::from_ptr(out_fmt).to_str().unwrap() };
    let svg_str = unsafe { CStr::from_ptr(out_svg).to_str().unwrap() };
    assert_eq!(fmt_str, "pedtools");
    assert!(svg_str.contains("<svg"));

    unsafe {
        joaju::ffi::joaju_free_string(out_fmt);
        joaju::ffi::joaju_free_string(out_svg);
    }
}

#[test]
fn test_ffi_compile_markdown_to_pdf() {
    let input = CString::new("# Test Title\nHello from C-ABI").unwrap();
    let mut out_data = ptr::null_mut();
    let mut out_len = 0;
    let mut out_err = ptr::null_mut();

    let rc = unsafe {
        joaju::ffi::joaju_compile_markdown_to_pdf(
            input.as_ptr(),
            &mut out_data,
            &mut out_len,
            &mut out_err,
        )
    };

    assert_eq!(rc, 0);
    assert!(!out_data.is_null());
    assert!(out_len > 0);
    assert!(out_err.is_null());

    let pdf_bytes = unsafe { std::slice::from_raw_parts(out_data, out_len) };
    assert!(pdf_bytes.starts_with(b"%PDF"));

    unsafe {
        joaju::ffi::joaju_free_bytes(out_data, out_len);
    }
}
