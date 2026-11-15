//! Poetic scansion and Typst typography engine powered by `joaju-lirica`.

pub use joaju_lirica::{parse_lirica_block, render_painel, render_typst, Poema, PoemaConfig};

/// Renders a `lirica` code block into native Typst markup.
pub fn render_lirica(code: &str) -> Result<String, String> {
    let (config, poem) = parse_lirica_block(code).map_err(|e| e.to_string())?;
    Ok(render_typst(&config, &poem))
}

/// Renders a `lirica` code block into structured HTML with scansion panel.
pub fn render_lirica_html(code: &str) -> Result<String, String> {
    let (config, poem) = parse_lirica_block(code).map_err(|e| e.to_string())?;
    let painel = render_painel(&poem);
    let title_badge = config.titulo.as_deref().unwrap_or("Lírica");

    let mut out = String::new();
    out.push_str("<div class=\"joaju-lirica-card\" data-engine=\"lirica\">\n");
    out.push_str(&format!(
        "  <div class=\"joaju-card-header\"><span class=\"joaju-card-badge\">✒ {title_badge}</span></div>\n"
    ));
    out.push_str("  <pre class=\"joaju-scansion\"><code>");
    for c in painel.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out.push_str("</code></pre>\n</div>");
    Ok(out)
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
