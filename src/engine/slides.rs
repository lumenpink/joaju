//! Presentation slides, handouts, and notes engine powered by `joaju-slides`.

pub use joaju_slides::{
    parse_markdown_deck, render_handout_typst, render_notes_typst, render_slides_typst,
    validate_deck, PresentationMode, SlideAspectRatio, SlideBlock, SlideDeck, SlidesError,
};

/// Renders a unified presentation Markdown document to native Typst source code.
pub fn render_slides_markup(markdown: &str, mode: PresentationMode) -> Result<String, String> {
    let deck = parse_markdown_deck(markdown).map_err(|e| e.to_string())?;

    match mode {
        PresentationMode::Slides => render_slides_typst(&deck).map_err(|e| e.to_string()),
        PresentationMode::Handout => render_handout_typst(&deck).map_err(|e| e.to_string()),
        PresentationMode::Notes => render_notes_typst(&deck).map_err(|e| e.to_string()),
    }
}

/// Compiles a unified presentation Markdown document directly to PDF bytes using Joaju's Typst engine.
pub fn compile_slides_to_pdf(markdown: &str, mode: PresentationMode) -> Result<Vec<u8>, String> {
    let typst_source = render_slides_markup(markdown, mode)?;
    crate::compiler::compile_typst_to_pdf(&typst_source)
}
