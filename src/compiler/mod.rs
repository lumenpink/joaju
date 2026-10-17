use crate::engine;
use crate::preprocessor::{preprocess_gists, preprocess_images, preprocess_tables};
use regex::Regex;
use serde::Deserialize;
use typst_as_lib::TypstEngine;

#[derive(Debug, Deserialize)]
pub struct CompileRequest {
    pub markdown: Option<String>,
}

pub fn preprocess_code_blocks(md: &str) -> (String, Vec<(String, Vec<u8>)>) {
    let mut assets = Vec::new();
    let mut counter = 0;

    let re_ped = Regex::new(r"(?s)```(pedtools|kinship2)\s*\n(.*?)```").unwrap();
    let res1 = re_ped.replace_all(md, |caps: &regex::Captures| {
        let code = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let fmt = caps.get(1).map(|m| m.as_str());
        match engine::pedigree::render_pedigree(code, fmt) {
            Ok((_, svg)) => {
                let filename = format!("asset_ped_{}.svg", counter);
                counter += 1;
                assets.push((filename.clone(), svg.into_bytes()));
                format!("\n#align(center)[#image(\"{}\")]\n", filename)
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
                format!("\n#align(center)[#image(\"{}\")]\n", filename)
            }
            Err(_) => caps.get(0).unwrap().as_str().to_string(),
        }
    });

    (res2.to_string(), assets)
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
    let clean_md = markdown.trim();
    if clean_md.is_empty() {
        return Err("Conteúdo Markdown vazio".to_string());
    }

    let mut title = "Documento Acadêmico".to_string();
    let mut author = "".to_string();

    let mut body_md = clean_md;
    if let Some(stripped) = clean_md.strip_prefix("---") {
        if let Some(end_idx) = stripped.find("---") {
            let frontmatter = &stripped[..end_idx];
            body_md = stripped[end_idx + 3..].trim();
            for line in frontmatter.lines() {
                if let Some((k, v)) = line.split_once(':') {
                    let key = k.trim().to_lowercase();
                    let val = v.trim().trim_matches('"').trim_matches('\'').to_string();
                    if key == "titulo" || key == "title" {
                        title = val;
                    } else if key == "autor" || key == "author" {
                        author = val;
                    }
                }
            }
        }
    }

    let (code_blocks_processed, assets) = preprocess_code_blocks(body_md);
    let mut processed = preprocess_tables(&code_blocks_processed);
    processed = preprocess_images(&processed);
    processed = preprocess_gists(&processed);
    processed = convert_markdown_to_typst(&processed);

    let mut typst_code = String::new();
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

    let engine = TypstEngine::builder()
        .search_fonts_with(typst_as_lib::typst_kit_options::TypstKitFontOptions::default())
        .with_static_source_file_resolver([("main.typ", typst_code.as_str())])
        .with_static_file_resolver(assets.iter().map(|(n, b)| (n.as_str(), b.as_slice())))
        .build();

    let doc: typst_layout::PagedDocument = engine
        .compile("main.typ")
        .output
        .map_err(|e| format!("Erro de compilação Typst: {:?}", e))?;

    let pdf = typst_pdf::pdf(&doc, &Default::default())
        .map_err(|e| format!("Erro ao gerar PDF: {:?}", e))?;

    Ok(pdf)
}
