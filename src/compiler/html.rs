use crate::engine;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;

/// Compila texto Markdown para HTML semântico com enriquecimento de blocos Joaju.
pub fn compile_markdown_to_html(markdown: &str) -> Result<String, String> {
    let mut replacements: Vec<(String, String)> = Vec::new();
    let mut counter = 0;

    // 1. Tabelas Joaju: ```table ... ```
    let re_table = Regex::new(r"(?s)```table\s*\n(.*?)```").map_err(|e| e.to_string())?;
    let mut intermediate = re_table
        .replace_all(markdown, |caps: &regex::Captures| {
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

    // 4. Parser CommonMark + GFM para o corpo Markdown restante
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&intermediate, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 5. Restaura os blocos HTML/SVG gerados pelo Joaju
    for (placeholder, rendered) in replacements {
        html_output = html_output.replace(&placeholder, &rendered);
    }

    Ok(html_output)
}
