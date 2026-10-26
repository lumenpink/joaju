//! Connected graphs and flowcharts engine powered by `joaju-graph`.

pub use joaju_graph::{render_graph_svg, render_graph_typst};

/// Renders a flowchart or connected graph DSL block to native Typst markup.
pub fn render_graph(code: &str) -> Result<String, String> {
    let svg = render_graph_svg(code)?;
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
