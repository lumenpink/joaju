//! Poetic scansion and Typst typography engine powered by `joaju-lirica`.

use joaju_lirica::{parse_lirica_block, render_typst};

/// Renders a `lirica` code block into native Typst markup.
pub fn render_lirica(code: &str) -> Result<String, String> {
    let (config, poem) = parse_lirica_block(code).map_err(|e| e.to_string())?;
    Ok(render_typst(&config, &poem))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_lirica() {
        let code = "%% titulo = Teste\nAmor é fogo que arde sem se ver";
        let res = render_lirica(code).expect("Rendering failed");
        assert!(res.contains("== Teste"));
        assert!(res.contains("Decassílabo Heroico"));
    }
}
