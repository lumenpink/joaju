use crate::engine;
use crate::preprocessor::{preprocess_gists, preprocess_images, preprocess_tables};
use regex::Regex;
use serde::Deserialize;
use typst_as_lib::TypstEngine;

static TEMPLATE_UFSC: &str = include_str!("../../data/trabalho-ufsc.typ");
static TEMPLATE_ABNT: &str = include_str!("../../data/template.typ");
static REFERENCIAS_BIB: &str = include_str!("../../data/referencias.bib");
static TRABALHO_UFSC_CSL: &str = include_str!("../../data/trabalho-ufsc.csl");

#[derive(Debug, Deserialize)]
pub struct CompileRequest {
    pub markdown: Option<String>,
    pub template: Option<String>,
}

pub fn preprocess_code_blocks(md: &str) -> (String, Vec<(String, Vec<u8>)>) {
    let mut assets = Vec::new();
    let mut counter = 0;

    let re_ped = Regex::new(r"(?s)```(pedtools|kinship2)\s*\n(.*?)```").unwrap();
    let res1 = re_ped.replace_all(md, |caps: &regex::Captures| {
        let code = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let fmt = caps.get(1).map(|m| m.as_str());

        let mut custom_height: Option<String> = None;
        let mut custom_width: Option<String> = None;

        for line in code.lines() {
            let t = line.trim();
            if t.starts_with("%%") {
                let directive_str = t.trim_start_matches('%').trim();
                for token in directive_str.split_whitespace() {
                    if let Some((k, v)) = token.split_once('=') {
                        let k = k.trim().to_lowercase();
                        let v = v.trim().trim_matches('"').trim_matches('\'');
                        if k == "height"
                            || k == "layout.height"
                            || k == "max_height"
                            || k == "layout.max_height"
                        {
                            custom_height = Some(v.to_string());
                        } else if (k == "scale" || k == "width" || k == "layout.width")
                            && !v.is_empty()
                        {
                            custom_width = Some(v.to_string());
                        }
                    }
                }
            }
        }

        match engine::pedigree::render_pedigree(code, fmt) {
            Ok((_, svg)) => {
                let filename = format!("asset_ped_{}.svg", counter);
                counter += 1;
                assets.push((filename.clone(), svg.into_bytes()));

                let width_arg = custom_width.unwrap_or_else(|| "100%".to_string());
                let img_call = match custom_height {
                    Some(h) => format!(
                        "#image(\"{}\", width: {}, height: {}, fit: \"contain\")",
                        filename, width_arg, h
                    ),
                    None => {
                        format!(
                            "#image(\"{}\", width: {}, fit: \"contain\")",
                            filename, width_arg
                        )
                    }
                };

                format!(
                    "\n#block(breakable: false, width: 100%)[#align(center)[{}]]\n",
                    img_call
                )
            }
            Err(_) => caps.get(0).unwrap().as_str().to_string(),
        }
    });

    let re_chem = Regex::new(r"(?s)```(openchemlib|openbabel)\s*\n(.*?)```").unwrap();
    let res2 = re_chem.replace_all(&res1, |caps: &regex::Captures| {
        let code = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let fmt = caps.get(1).map(|m| m.as_str());
        match engine::chem::render_chem(code, fmt) {
            Ok((_, svg)) => {
                let filename = format!("asset_chem_{}.svg", counter);
                counter += 1;
                assets.push((filename.clone(), svg.into_bytes()));
                format!("\n#block(breakable: false, width: 100%)[#align(center)[#image(\"{}\", width: 100%, fit: \"contain\")]]\n", filename)
            }
            Err(_) => caps.get(0).unwrap().as_str().to_string(),
        }
    });

    let re_lirica = Regex::new(r"(?s)```lirica\s*\n(.*?)```").unwrap();
    let res3 = re_lirica.replace_all(&res2, |caps: &regex::Captures| {
        let code = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        match engine::lirica::render_lirica(code) {
            Ok(typst) => format!("\n{}\n", typst),
            Err(_) => caps.get(0).unwrap().as_str().to_string(),
        }
    });

    let re_wireframe = Regex::new(r"(?s)```(wireframe|salt)\s*\n(.*?)```").unwrap();
    let res4 = re_wireframe.replace_all(&res3, |caps: &regex::Captures| {
        let code = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        match engine::wireframe::render_wireframe(code) {
            Ok(typst) => format!("\n{}\n", typst),
            Err(_) => caps.get(0).unwrap().as_str().to_string(),
        }
    });

    (res4.to_string(), assets)
}

fn normalize_typst_math_expressions(content: &str) -> String {
    let re = Regex::new(r"\$([^\$]+)\$").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let inner = &caps[1];
        let mut fixed = inner.to_string();
        fixed = fixed.replace("ii", "i i");
        fixed = fixed.replace("Bb", "B b");
        fixed = fixed.replace("bb", "b b");
        fixed = fixed.replace("BB", "B B");
        fixed = fixed.replace(r"B\_", "B \"_\"");
        fixed = fixed.replace(r"B_", "B \"_\"");
        format!("${}$", fixed)
    })
    .to_string()
}

pub fn convert_markdown_to_typst(md: &str) -> String {
    let mut result = Vec::new();
    let mut in_code_block = false;

    for line in md.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            result.push(line.to_string());
            continue;
        }

        if in_code_block {
            result.push(line.to_string());
            continue;
        }

        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            continue;
        }

        let trimmed_all = line.trim();
        if trimmed_all == "<pagebreak>"
            || trimmed_all == "<pagebreak/>"
            || trimmed_all == "<pagebreak />"
            || trimmed_all == "<!-- pagebreak -->"
            || trimmed_all == "<!-- newpage -->"
            || trimmed_all == "\\pagebreak"
            || trimmed_all == "\\newpage"
        {
            result.push("#pagebreak()".to_string());
            continue;
        }

        if let Some(rest) = line.strip_prefix("###### ") {
            result.push(format!("====== {}", rest));
        } else if let Some(rest) = line.strip_prefix("##### ") {
            result.push(format!("===== {}", rest));
        } else if let Some(rest) = line.strip_prefix("#### ") {
            result.push(format!("==== {}", rest));
        } else if let Some(rest) = line.strip_prefix("### ") {
            result.push(format!("=== {}", rest));
        } else if let Some(rest) = line.strip_prefix("## ") {
            result.push(format!("== {}", rest));
        } else if let Some(rest) = line.strip_prefix("# ") {
            result.push(format!("= {}", rest));
        } else {
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

pub fn compile_markdown_to_pdf(markdown: &str) -> Result<Vec<u8>, String> {
    compile_markdown_to_pdf_with_options(markdown, None)
}

pub fn compile_markdown_to_pdf_with_options(
    markdown: &str,
    template_override: Option<&str>,
) -> Result<Vec<u8>, String> {
    let clean_md = markdown.trim();
    if clean_md.is_empty() {
        return Err("Conteúdo Markdown vazio".to_string());
    }

    let mut title = "Documento Acadêmico".to_string();
    let mut subtitle: Option<String> = None;
    let mut author = "".to_string();
    let mut institution = "".to_string();
    let mut city = "".to_string();
    let mut year = "".to_string();
    let mut template_name = template_override.map(|s| s.to_string());

    let mut body_md = clean_md;
    if let Some(stripped) = clean_md.strip_prefix("---") {
        if let Some(end_idx) = stripped.find("---") {
            let frontmatter = &stripped[..end_idx];
            body_md = stripped[end_idx + 3..].trim();
            let mut lines = frontmatter.lines().peekable();
            while let Some(line) = lines.next() {
                if let Some((k, v)) = line.split_once(':') {
                    let key = k.trim().to_lowercase();
                    let v_trimmed = v.trim();
                    let val = if v_trimmed == "|-"
                        || v_trimmed == "|"
                        || v_trimmed == ">"
                        || v_trimmed == ">-"
                        || v_trimmed.is_empty()
                    {
                        let mut block_lines = Vec::new();
                        while let Some(next_line) = lines.peek() {
                            if next_line.starts_with("  ") || next_line.starts_with('\t') {
                                block_lines.push(next_line.trim());
                                lines.next();
                            } else if next_line.trim().is_empty() {
                                lines.next();
                            } else {
                                break;
                            }
                        }
                        if !block_lines.is_empty() {
                            block_lines.join("\n")
                        } else {
                            v_trimmed.trim_matches('"').trim_matches('\'').to_string()
                        }
                    } else {
                        v_trimmed.trim_matches('"').trim_matches('\'').to_string()
                    };

                    if key == "titulo" || key == "title" {
                        title = val;
                    } else if key == "subtitulo" || key == "subtitle" {
                        subtitle = Some(val);
                    } else if key == "autor" || key == "author" {
                        author = val;
                    } else if key == "instituicao" || key == "institution" {
                        institution = val;
                    } else if key == "local" || key == "city" {
                        city = val;
                    } else if key == "ano" || key == "year" {
                        year = val;
                    } else if (key == "template" || key == "modelo" || key == "perfil")
                        && template_name.is_none()
                    {
                        template_name = Some(val);
                    }
                }
            }
        }
    }

    let (code_blocks_processed, assets) = preprocess_code_blocks(body_md);
    let mut processed = preprocess_tables(&code_blocks_processed);
    processed = preprocess_images(&processed);
    processed = preprocess_gists(&processed);
    processed = normalize_typst_math_expressions(&processed);
    processed = convert_markdown_to_typst(&processed);

    let mut typst_code = String::new();

    match template_name.as_deref() {
        Some("trabalho-ufsc") | Some("ufsc") => {
            typst_code.push_str("#import \"trabalho-ufsc.typ\": trabalho_ufsc\n\n");
            typst_code.push_str("#show: doc => trabalho_ufsc(\n  doc,\n");
            if !institution.is_empty() {
                typst_code.push_str(&format!(
                    "  instituicao: \"{}\",\n",
                    institution.replace('\n', "\\n").replace('"', "\\\"")
                ));
            }
            if !author.is_empty() {
                typst_code.push_str(&format!("  autor: \"{}\",\n", author.replace('"', "\\\"")));
            }
            typst_code.push_str(&format!("  titulo: \"{}\",\n", title.replace('"', "\\\"")));
            if let Some(ref sub) = subtitle {
                typst_code.push_str(&format!("  subtitulo: \"{}\",\n", sub.replace('"', "\\\"")));
            }
            if !city.is_empty() {
                typst_code.push_str(&format!("  local: \"{}\",\n", city.replace('"', "\\\"")));
            }
            if !year.is_empty() {
                typst_code.push_str(&format!("  ano: \"{}\",\n", year.replace('"', "\\\"")));
            }
            typst_code.push_str(")\n\n");
            let clean_body = processed.replace("= REFERÊNCIAS BIBLIOGRÁFICAS", "");
            typst_code.push_str(&clean_body);
            typst_code.push_str("\n\n#bibliography(\"referencias.bib\", title: \"REFERÊNCIAS BIBLIOGRÁFICAS\", style: \"trabalho-ufsc.csl\")\n");
        }
        Some("abnt") | Some("artigo") => {
            typst_code.push_str("#import \"template.typ\": abnt\n\n");
            typst_code.push_str("#show: doc => abnt(doc)\n\n");
            typst_code.push_str(&format!("= {}\n\n", title));
            if !author.is_empty() {
                typst_code.push_str(&format!("*{}*\n\n", author));
            }
            typst_code.push_str("#v(0.5cm)\n\n");
            typst_code.push_str(&processed);
        }
        _ => {
            typst_code.push_str(
                "#set page(paper: \"a4\", margin: (top: 2.5cm, bottom: 2.5cm, left: 3cm, right: 2cm))\n",
            );
            typst_code.push_str("#set text(size: 11pt, lang: \"pt\")\n");
            typst_code.push_str("#set par(justify: true, leading: 0.75em)\n\n");

            typst_code.push_str(&format!("= {}\n\n", title));
            if !author.is_empty() {
                typst_code.push_str(&format!("*{}*\n\n", author));
            }
            typst_code.push_str("#v(0.5cm)\n\n");
            typst_code.push_str(&processed);
        }
    }

    let mut file_resolver_assets: Vec<(&str, &[u8])> = assets
        .iter()
        .map(|(n, b)| (n.as_str(), b.as_slice()))
        .collect();
    file_resolver_assets.push(("referencias.bib", REFERENCIAS_BIB.as_bytes()));
    file_resolver_assets.push(("trabalho-ufsc.csl", TRABALHO_UFSC_CSL.as_bytes()));

    compile_typst_to_pdf_with_assets(&typst_code, file_resolver_assets)
}

pub fn compile_typst_to_pdf(typst_code: &str) -> Result<Vec<u8>, String> {
    compile_typst_to_pdf_with_assets(typst_code, vec![])
}

pub fn compile_typst_to_pdf_with_assets(
    typst_code: &str,
    assets: Vec<(&str, &[u8])>,
) -> Result<Vec<u8>, String> {
    let engine = TypstEngine::builder()
        .search_fonts_with(typst_as_lib::typst_kit_options::TypstKitFontOptions::default())
        .with_static_source_file_resolver([
            ("main.typ", typst_code),
            ("trabalho-ufsc.typ", TEMPLATE_UFSC),
            ("template.typ", TEMPLATE_ABNT),
        ])
        .with_static_file_resolver(assets)
        .build();

    let doc: typst_layout::PagedDocument = engine
        .compile("main.typ")
        .output
        .map_err(|e| format!("Erro de compilação Typst: {:?}", e))?;

    let pdf = typst_pdf::pdf(&doc, &Default::default())
        .map_err(|e| format!("Erro ao gerar PDF: {:?}", e))?;

    Ok(pdf)
}
