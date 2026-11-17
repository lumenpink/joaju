use crate::engine;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

/// Compila texto Markdown para HTML semântico com enriquecimento de blocos Joaju.
pub fn compile_markdown_to_html(markdown: &str) -> Result<String, String> {
    let s = joaju_markdown::preprocess_comments(markdown);
    let s = joaju_markdown::preprocess_callouts(&s);
    let s = joaju_markdown::preprocess_wikilinks(&s);
    let preprocessed = joaju_markdown::preprocess_highlights(&s);
    let mut replacements: Vec<(String, String)> = Vec::new();
    let mut counter = 0;

    // 1. Tabelas Joaju: ```table ... ```
    let re_table = Regex::new(r"(?s)```table\s*\n(.*?)```").map_err(|e| e.to_string())?;
    let mut intermediate = re_table
        .replace_all(&preprocessed, |caps: &regex::Captures| {
            let code = caps.get(1).map(|m| m.as_str()).unwrap_or("").trim();
            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let rendered = match crate::table::render_html_table(code) {
                Ok(html) => html,
                Err(_) => caps.get(0).unwrap().as_str().to_string(),
            };
            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 2. Pedigrees: ```pedtools ... ``` ou ```kinship2 ... ```
    let re_ped =
        Regex::new(r"(?s)```(pedtools|kinship2)\s*\n(.*?)```").map_err(|e| e.to_string())?;
    intermediate = re_ped
        .replace_all(&intermediate, |caps: &regex::Captures| {
            let fmt = caps.get(1).map(|m| m.as_str());
            let code = caps.get(2).map(|m| m.as_str()).unwrap_or("").trim();
            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let rendered = match engine::pedigree::render_pedigree(code, fmt) {
                Ok((_, svg)) => svg,
                Err(_) => caps.get(0).unwrap().as_str().to_string(),
            };
            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 3. Química: ```chem ... ``` ou ```openchemlib ... ``` ou ```openbabel ... ```
    let re_chem = Regex::new(r"(?s)```(chem|openchemlib|openbabel)\s*\n(.*?)```")
        .map_err(|e| e.to_string())?;
    intermediate = re_chem
        .replace_all(&intermediate, |caps: &regex::Captures| {
            let fmt = caps.get(1).map(|m| m.as_str());
            let code = caps.get(2).map(|m| m.as_str()).unwrap_or("").trim();
            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let rendered = match engine::chem::render_chem(code, fmt) {
                Ok((_, svg)) => svg,
                Err(_) => caps.get(0).unwrap().as_str().to_string(),
            };
            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 4. Lírica: ```lirica ... ``` ou ```poema ... ``` ou ```poesia ... ```
    let re_lirica =
        Regex::new(r"(?s)```(lirica|poema|poesia)\s*\n(.*?)```").map_err(|e| e.to_string())?;
    intermediate = re_lirica
        .replace_all(&intermediate, |caps: &regex::Captures| {
            let code = caps.get(2).map(|m| m.as_str()).unwrap_or("").trim();
            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let rendered = match engine::lirica::render_lirica_html(code) {
                Ok(html) => html,
                Err(_) => caps.get(0).unwrap().as_str().to_string(),
            };
            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 5. Wireframe: ```wireframe ... ``` ou ```salt ... ``` ou ```ui ... ```
    let re_wireframe =
        Regex::new(r"(?s)```(wireframe|salt|ui)\s*\n(.*?)```").map_err(|e| e.to_string())?;
    intermediate = re_wireframe
        .replace_all(&intermediate, |caps: &regex::Captures| {
            let code = caps.get(2).map(|m| m.as_str()).unwrap_or("").trim();
            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let rendered = match joaju_wireframe::render_wireframe_svg(code) {
                Ok(svg) => format!(
                    "<div class=\"joaju-diagram-block\" data-engine=\"wireframe\">\n{svg}\n</div>"
                ),
                Err(_) => caps.get(0).unwrap().as_str().to_string(),
            };
            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 6. Sequence: ```sequence ... ``` ou ```sequencediagram ... ```
    let re_sequence =
        Regex::new(r"(?s)```(sequence|sequencediagram)\s*\n(.*?)```").map_err(|e| e.to_string())?;
    intermediate = re_sequence
        .replace_all(&intermediate, |caps: &regex::Captures| {
            let code = caps.get(2).map(|m| m.as_str()).unwrap_or("").trim();
            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let rendered = match joaju_sequence::render_sequence_svg(code) {
                Ok(svg) => format!(
                    "<div class=\"joaju-diagram-block\" data-engine=\"sequence\">\n{svg}\n</div>"
                ),
                Err(_) => caps.get(0).unwrap().as_str().to_string(),
            };
            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 7. Graph / Flowchart: ```graph ... ``` ou ```flowchart ... ```
    let re_graph =
        Regex::new(r"(?s)```(graph|flowchart)\s*\n(.*?)```").map_err(|e| e.to_string())?;
    intermediate = re_graph
        .replace_all(&intermediate, |caps: &regex::Captures| {
            let code = caps.get(2).map(|m| m.as_str()).unwrap_or("").trim();
            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let rendered = match joaju_graph::render_graph_svg(code) {
                Ok(svg) => format!(
                    "<div class=\"joaju-diagram-block\" data-engine=\"graph\">\n{svg}\n</div>"
                ),
                Err(_) => caps.get(0).unwrap().as_str().to_string(),
            };
            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 8. Typst / Joaju: ```typst ... ``` ou ```joaju ... ```
    let re_typst = Regex::new(r"(?s)```(typst|joaju)\s*\n(.*?)```").map_err(|e| e.to_string())?;
    intermediate = re_typst
        .replace_all(&intermediate, |caps: &regex::Captures| {
            let lang = caps.get(1).map(|m| m.as_str()).unwrap_or("typst").trim();
            let code = caps.get(2).map(|m| m.as_str()).unwrap_or("").trim();
            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let escaped = escape_html(code);
            let rendered = format!(
                "<div class=\"joaju-code-block joaju-typst-block\" data-language=\"{lang}\">\n  <div class=\"joaju-code-header\"><span class=\"joaju-code-badge\">{lang}</span></div>\n  <pre class=\"language-{lang}\"><code class=\"language-{lang}\">{escaped}</code></pre>\n</div>"
            );
            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 9. Blocos de código genéricos demarcados com linguagem: ```lang ... ```
    let re_code =
        Regex::new(r"(?s)```([a-zA-Z0-9_\-\+]+)\s*\n(.*?)```").map_err(|e| e.to_string())?;
    intermediate = re_code
        .replace_all(&intermediate, |caps: &regex::Captures| {
            let lang = caps.get(1).map(|m| m.as_str()).unwrap_or("code").trim();
            let code = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let escaped = escape_html(code.trim_end());
            let rendered = format!(
                "<div class=\"joaju-code-block\" data-language=\"{lang}\">\n  <div class=\"joaju-code-header\"><span class=\"joaju-code-badge\">{lang}</span></div>\n  <pre class=\"language-{lang}\"><code class=\"language-{lang}\">{escaped}</code></pre>\n</div>"
            );
            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 10. Figuras semânticas: ![alt](url) ou ![alt](url "legenda")
    let re_fig = Regex::new(r#"!\[([^\]]*)\]\(\s*([^\s\)\"]+)(?:\s+["']([^"']*)["'])?\s*\)"#)
        .map_err(|e| e.to_string())?;
    intermediate = re_fig
        .replace_all(&intermediate, |caps: &regex::Captures| {
            let alt = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            let url = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");
            let title = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("");

            let caption = if !title.is_empty() {
                title
            } else if !alt.is_empty() {
                alt
            } else {
                ""
            };

            let placeholder = format!("<!--JOAJU_REPLACEMENT_{}-->", counter);
            counter += 1;
            let escaped_alt = escape_html(alt);
            let escaped_url = escape_html(url);
            let escaped_caption = escape_html(caption);

            let rendered = if !caption.is_empty() {
                format!(
                    "<figure class=\"joaju-figure\"><img src=\"{escaped_url}\" alt=\"{escaped_alt}\" /><figcaption class=\"joaju-figcaption\">{escaped_caption}</figcaption></figure>"
                )
            } else {
                format!("<img src=\"{escaped_url}\" alt=\"{escaped_alt}\" />")
            };

            replacements.push((placeholder.clone(), rendered));
            format!("\n\n{}\n\n", placeholder)
        })
        .to_string();

    // 11. Parser CommonMark + GFM para o corpo Markdown restante
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&intermediate, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 12. Restaura os blocos HTML/SVG gerados pelo Joaju
    for (placeholder, rendered) in replacements {
        let wrapped_in_p = format!("<p>{placeholder}</p>");
        let wrapped_in_p_nl = format!("<p>{placeholder}\n</p>");
        if html_output.contains(&wrapped_in_p_nl) {
            html_output = html_output.replace(&wrapped_in_p_nl, &rendered);
        } else if html_output.contains(&wrapped_in_p) {
            html_output = html_output.replace(&wrapped_in_p, &rendered);
        } else {
            html_output = html_output.replace(&placeholder, &rendered);
        }
    }

    Ok(html_output)
}
