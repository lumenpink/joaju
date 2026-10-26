//! Deterministic temporal sequence diagrams engine powered by `joaju-sequence`.

pub use joaju_sequence::{render_sequence_svg, render_sequence_typst};

/// Renders a sequence diagram DSL block to native Typst markup.
pub fn render_sequence(code: &str) -> Result<String, String> {
    let svg = render_sequence_svg(code)?;
    let mut escaped_svg = String::with_capacity(svg.len() + 32);
    for c in svg.chars() {
        match c {
            '\\' => escaped_svg.push_str("\\\\"),
            '"' => escaped_svg.push_str("\\\""),
            _ => escaped_svg.push(c),
        }
    }
    Ok(format!(
        "#align(center)[#box(width: 95%)[#image(bytes(\"{}\"), width: 100%, fit: \"contain\")]]\n",
        escaped_svg
    ))
}
