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
//! use joaju::compiler::compile_markdown_to_html;
//!
//! let markdown = "# Hello\nThis is compiled by Joaju.";
//! let result = compile_markdown_to_html(markdown);
//! assert!(result.is_ok());
//! ```

/// Canonical SemVer version of the crate.
pub const VERSION: &str = env!("JOAJU_FULL_VERSION");

pub mod compiler;
pub mod engine;
pub mod ffi;
pub mod preprocessor;

pub use compiler::{
    compile_markdown_to_html, compile_markdown_to_html_with_base_dir,
    compile_markdown_to_html_with_source, compile_markdown_to_pdf,
    compile_markdown_to_pdf_with_options, compile_preprocessed_markdown_to_html,
    compile_typst_to_pdf, CompileRequest, ReferenceNote, ReferenceSource,
};
pub use engine::chem;
pub use engine::chem::render_chem;
pub use engine::graph;
pub use engine::graph::render_graph;
pub use engine::lirica;
pub use engine::lirica::render_lirica;
pub use engine::pedigree;
pub use engine::pedigree::{render_pedigree, RenderRequest, RenderResponse};
pub use engine::sequence;
pub use engine::sequence::render_sequence;
pub use engine::slides;
pub use engine::wireframe;
pub use engine::wireframe::render_wireframe;
pub use joaju_markdown as markdown;
pub use joaju_table as table;
pub use joaju_templates as templates;
pub use preprocessor::preprocess_markdown;

/// Prepara o texto Markdown aplicando a esteira canônica unificada Joaju:
/// 1. Resolução de citações e anexação bibliográfica (via joaju-refs) usando [`ReferenceSource`].
/// 2. Remoção de comentários `%%...%%` (via joaju-markdown).
/// 3. Conversão de callouts `> [!type]` (via joaju-markdown).
/// 4. Conversão de realces `==...==` (via joaju-markdown).
/// 5. Normalização de wikilinks `[[...]]` (via joaju-markdown).
#[must_use]
pub fn prepare_markdown(markdown: &str, references: Option<ReferenceSource>) -> String {
    let with_refs = compiler::references::resolve_references_with_source(markdown, references);
    let s = joaju_markdown::preprocess_comments(&with_refs);
    let s = joaju_markdown::preprocess_callouts(&s);
    let s = joaju_markdown::preprocess_highlights(&s);
    joaju_markdown::preprocess_wikilinks(&s)
}

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
        ("joaju-refs", "0.1.0"),
    ]
}
