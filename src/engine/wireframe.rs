//! Wireframe and UI mockups engine powered by `joaju-wireframe`.

pub use joaju_wireframe::render_wireframe_typst;

/// Renders a wireframe/salt DSL block to native Typst markup.
pub fn render_wireframe(code: &str) -> Result<String, String> {
    render_wireframe_typst(code)
}
