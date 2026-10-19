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

pub mod compiler;
pub mod engine;
pub mod preprocessor;
pub mod unfurl;

pub use compiler::{compile_markdown_to_pdf, compile_markdown_to_pdf_with_options, CompileRequest};
pub use engine::chem::render_chem;
pub use engine::pedigree::{render_pedigree, RenderRequest, RenderResponse};
pub use preprocessor::preprocess_markdown;
pub use unfurl::{unfurl_url, UnfurlRequest, UnfurlResponse};
