//! C-ABI Foreign Function Interface (FFI) for `joaju`.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Compiles Markdown text to PDF using the Joaju compiler.
///
/// Returns `0` on success, populating `out_pdf_data` and `out_pdf_len`.
/// Returns `-1` on error, populating `out_error` if non-null.
///
/// # Safety
/// Pointers must be valid or null as documented. The caller must free
/// non-null `out_pdf_data` via [`joaju_free_bytes`] and non-null `out_error`
/// via [`joaju_free_string`].
#[no_mangle]
pub unsafe extern "C" fn joaju_compile_markdown_to_pdf(
    markdown: *const c_char,
    out_pdf_data: *mut *mut u8,
    out_pdf_len: *mut usize,
    out_error: *mut *mut c_char,
) -> i32 {
    if !out_pdf_data.is_null() {
        *out_pdf_data = ptr::null_mut();
    }
    if !out_pdf_len.is_null() {
        *out_pdf_len = 0;
    }
    if !out_error.is_null() {
        *out_error = ptr::null_mut();
    }

    if markdown.is_null() {
        if !out_error.is_null() {
            if let Ok(c_err) = CString::new("markdown pointer is null") {
                *out_error = c_err.into_raw();
            }
        }
        return -1;
    }

    let md_str = match CStr::from_ptr(markdown).to_str() {
        Ok(s) => s,
        Err(e) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(format!("UTF-8 error in markdown: {}", e)) {
                    *out_error = c_err.into_raw();
                }
            }
            return -1;
        }
    };

    match crate::compiler::compile_markdown_to_pdf(md_str) {
        Ok(pdf_bytes) => {
            if !out_pdf_len.is_null() {
                *out_pdf_len = pdf_bytes.len();
            }
            if !out_pdf_data.is_null() {
                let mut boxed = pdf_bytes.into_boxed_slice();
                *out_pdf_data = boxed.as_mut_ptr();
                std::mem::forget(boxed);
            }
            0
        }
        Err(err) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(err) {
                    *out_error = c_err.into_raw();
                }
            }
            -1
        }
    }
}

/// Compiles Typst source text to PDF using the Joaju compiler.
///
/// # Safety
/// Caller must free allocated memory using [`joaju_free_bytes`] and [`joaju_free_string`].
#[no_mangle]
pub unsafe extern "C" fn joaju_compile_typst_to_pdf(
    typst_code: *const c_char,
    out_pdf_data: *mut *mut u8,
    out_pdf_len: *mut usize,
    out_error: *mut *mut c_char,
) -> i32 {
    if !out_pdf_data.is_null() {
        *out_pdf_data = ptr::null_mut();
    }
    if !out_pdf_len.is_null() {
        *out_pdf_len = 0;
    }
    if !out_error.is_null() {
        *out_error = ptr::null_mut();
    }

    if typst_code.is_null() {
        if !out_error.is_null() {
            if let Ok(c_err) = CString::new("typst_code pointer is null") {
                *out_error = c_err.into_raw();
            }
        }
        return -1;
    }

    let code_str = match CStr::from_ptr(typst_code).to_str() {
        Ok(s) => s,
        Err(e) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(format!("UTF-8 error in typst_code: {}", e)) {
                    *out_error = c_err.into_raw();
                }
            }
            return -1;
        }
    };

    match crate::compiler::compile_typst_to_pdf(code_str) {
        Ok(pdf_bytes) => {
            if !out_pdf_len.is_null() {
                *out_pdf_len = pdf_bytes.len();
            }
            if !out_pdf_data.is_null() {
                let mut boxed = pdf_bytes.into_boxed_slice();
                *out_pdf_data = boxed.as_mut_ptr();
                std::mem::forget(boxed);
            }
            0
        }
        Err(err) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(err) {
                    *out_error = c_err.into_raw();
                }
            }
            -1
        }
    }
}

/// Renders a pedigree code block to SVG.
///
/// # Safety
/// Caller must free allocated strings using [`joaju_free_string`].
#[no_mangle]
pub unsafe extern "C" fn joaju_render_pedigree(
    raw_code: *const c_char,
    forced_format: *const c_char,
    out_format: *mut *mut c_char,
    out_svg: *mut *mut c_char,
    out_error: *mut *mut c_char,
) -> i32 {
    if !out_format.is_null() {
        *out_format = ptr::null_mut();
    }
    if !out_svg.is_null() {
        *out_svg = ptr::null_mut();
    }
    if !out_error.is_null() {
        *out_error = ptr::null_mut();
    }

    if raw_code.is_null() {
        if !out_error.is_null() {
            if let Ok(c_err) = CString::new("raw_code pointer is null") {
                *out_error = c_err.into_raw();
            }
        }
        return -1;
    }

    let code_str = match CStr::from_ptr(raw_code).to_str() {
        Ok(s) => s,
        Err(e) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(format!("UTF-8 error in raw_code: {}", e)) {
                    *out_error = c_err.into_raw();
                }
            }
            return -1;
        }
    };

    let fmt_str_opt = if forced_format.is_null() {
        None
    } else {
        match CStr::from_ptr(forced_format).to_str() {
            Ok(s) if !s.is_empty() => Some(s),
            _ => None,
        }
    };

    match crate::engine::pedigree::render_pedigree(code_str, fmt_str_opt) {
        Ok((fmt, svg)) => {
            if !out_format.is_null() {
                if let Ok(c_fmt) = CString::new(fmt) {
                    *out_format = c_fmt.into_raw();
                }
            }
            if !out_svg.is_null() {
                if let Ok(c_svg) = CString::new(svg) {
                    *out_svg = c_svg.into_raw();
                }
            }
            0
        }
        Err(err) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(err) {
                    *out_error = c_err.into_raw();
                }
            }
            -1
        }
    }
}

/// Renders a chemical SMILES or code block to SVG.
///
/// # Safety
/// Caller must free allocated strings using [`joaju_free_string`].
#[no_mangle]
pub unsafe extern "C" fn joaju_render_chem(
    raw_code: *const c_char,
    forced_format: *const c_char,
    out_format: *mut *mut c_char,
    out_svg: *mut *mut c_char,
    out_error: *mut *mut c_char,
) -> i32 {
    if !out_format.is_null() {
        *out_format = ptr::null_mut();
    }
    if !out_svg.is_null() {
        *out_svg = ptr::null_mut();
    }
    if !out_error.is_null() {
        *out_error = ptr::null_mut();
    }

    if raw_code.is_null() {
        if !out_error.is_null() {
            if let Ok(c_err) = CString::new("raw_code pointer is null") {
                *out_error = c_err.into_raw();
            }
        }
        return -1;
    }

    let code_str = match CStr::from_ptr(raw_code).to_str() {
        Ok(s) => s,
        Err(e) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(format!("UTF-8 error in raw_code: {}", e)) {
                    *out_error = c_err.into_raw();
                }
            }
            return -1;
        }
    };

    let fmt_str_opt = if forced_format.is_null() {
        None
    } else {
        match CStr::from_ptr(forced_format).to_str() {
            Ok(s) if !s.is_empty() => Some(s),
            _ => None,
        }
    };

    match crate::engine::chem::render_chem(code_str, fmt_str_opt) {
        Ok((fmt, svg)) => {
            if !out_format.is_null() {
                if let Ok(c_fmt) = CString::new(fmt) {
                    *out_format = c_fmt.into_raw();
                }
            }
            if !out_svg.is_null() {
                if let Ok(c_svg) = CString::new(svg) {
                    *out_svg = c_svg.into_raw();
                }
            }
            0
        }
        Err(err) => {
            if !out_error.is_null() {
                if let Ok(c_err) = CString::new(err) {
                    *out_error = c_err.into_raw();
                }
            }
            -1
        }
    }
}

/// Frees a byte slice allocated by the C-ABI FFI.
///
/// # Safety
/// `ptr` must point to a buffer allocated via `Box::into_raw` with length `len`.
#[no_mangle]
pub unsafe extern "C" fn joaju_free_bytes(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        let slice = std::slice::from_raw_parts_mut(ptr, len);
        drop(Box::from_raw(slice));
    }
}

/// Frees a string allocated by the C-ABI FFI.
///
/// # Safety
/// `ptr` must point to a string allocated via `CString::into_raw` or be null.
#[no_mangle]
pub unsafe extern "C" fn joaju_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}
