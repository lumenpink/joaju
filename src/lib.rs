//! # Joaju
//!
//! A fast, scientific academic compiler and typographical orchestrator in Rust
//! for Typst output, clinical pedigrees, and tables.
//!
//! > ⚠️ **Early Development Notice (pre-1.0 / alpha)**  
//! > Joaju is currently in early active development (`0.1.0`). It is NOT recommended
//! > for production use beyond experimental testing and research. APIs are subject
//! > to change prior to a stable 1.0.0 release.
//!
//! ## Quick Example
//!
//! ```rust
//! use joaju::compiler::compile_markdown_to_pdf;
//!
//! let markdown = "# Hello\nThis is compiled by Joaju.";
//! let result = compile_markdown_to_pdf(markdown);
//! assert!(result.is_ok());
//! ```

/// Canonical SemVer version of the crate.
pub const VERSION: &str = env!("JOAJU_FULL_VERSION");

pub mod compiler;
pub mod engine;
pub mod ffi;
pub mod preprocessor;

pub use compiler::{
    compile_markdown_to_html, compile_markdown_to_pdf, compile_markdown_to_pdf_with_options,
    compile_typst_to_pdf, CompileRequest,
};
pub use engine::chem::render_chem;
pub use engine::pedigree::{render_pedigree, RenderRequest, RenderResponse};
pub use engine::slides;
pub use joaju_table as table;
pub use joaju_templates as templates;
pub use preprocessor::preprocess_markdown;

/// Returns the inventory of all linked Joaju ecosystem submodules and their SemVer versions.
pub fn ecosystem_inventory() -> Vec<(&'static str, &'static str)> {
    vec![
        ("joaju-pdf", "0.1.0"),
        ("joaju-chem", "0.1.1"),
        ("joaju-pedigree", "0.2.2"),
        ("joaju-table", "0.1.0"),
        ("joaju-markdown", "0.1.0"),
        ("joaju-templates", "0.1.0"),
        ("joaju-lirica", "0.1.0"),
        ("joaju-slides", "0.2.0"),
        ("joaju-wireframe", "0.1.0"),
        ("joaju-sequence", "0.1.0"),
        ("joaju-graph", "0.1.0"),
    ]
}
