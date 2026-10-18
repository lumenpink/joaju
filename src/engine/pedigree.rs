use pedigreers::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RenderRequest {
    pub code: String,
    pub format: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RenderResponse {
    pub format: String,
    pub r#type: String,
    pub svg: String,
}

#[derive(Debug, Serialize)]
pub struct RenderErrorResponse {
    pub error: String,
}

pub fn render_pedigree(
    raw_code: &str,
    forced_format: Option<&str>,
) -> Result<(String, String), String> {
    let re_fence = Regex::new(r"(?s)```(pedtools|kinship2)\s*\n(.*?)```").unwrap();
    let (detected_format, content) = if let Some(caps) = re_fence.captures(raw_code) {
        let fmt = caps.get(1).map(|m| m.as_str()).unwrap_or("pedtools");
        let body = caps.get(2).map(|m| m.as_str()).unwrap_or(raw_code);
        (fmt, body)
    } else {
        (forced_format.unwrap_or("pedtools"), raw_code)
    };

    let clean = content.trim();
    if clean.is_empty() {
        return Err("Heredograma vazio".to_string());
    }

    // Resolução SSOT (Ticket #21) via ssot-rs
    let (directives, body) = ssot_rs::split_block_content(content);
    let is_ssrot_block = directives.contains_key("src")
        || directives.contains_key("file")
        || directives.contains_key("select")
        || directives.contains_key("where");

    let resolved_ped_text = if is_ssrot_block {
        let dataset = if let Some(src_path) = directives
            .get("src")
            .or_else(|| directives.get("file"))
            .or_else(|| directives.get("arquivo"))
        {
            ssot_rs::Dataset::from_file(src_path).map_err(|e| e.to_string())?
        } else {
            ssot_rs::Dataset::from_csv(&body).map_err(|e| e.to_string())?
        };

        let select_expr = directives.get("select").map(|s| s.as_str());
        let where_expr = directives.get("where").map(|s| s.as_str());
        let queried = dataset
            .apply_query(select_expr, where_expr)
            .map_err(|e| e.to_string())?;

        let csv_output = queried.to_csv().map_err(|e| e.to_string())?;

        // Preserva diretivas de layout, legendas, etc.
        let mut final_text = String::new();
        for (k, v) in &directives {
            if k != "src" && k != "file" && k != "arquivo" && k != "select" && k != "where" {
                final_text.push_str(&format!("%% {}={}\n", k, v));
            }
        }
        final_text.push_str(&csv_output);
        final_text
    } else {
        content.to_string()
    };

    let mut data_lines = 0;
    for line in resolved_ped_text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with("%%") {
            continue;
        }
        let cols: Vec<&str> = if t.contains(',') {
            t.split(',').collect()
        } else {
            t.split_whitespace().collect()
        };
        if cols.len() < 4 {
            return Err(format!(
                "Linha de heredograma malformada (mínimo de 4 colunas): '{}'",
                t
            ));
        }
        data_lines += 1;
    }
    if data_lines == 0 {
        return Err("Nenhum indivíduo encontrado no heredograma".to_string());
    }

    let (ped, parsed_directives) = Ped::from_text(&resolved_ped_text).map_err(|e| e.to_string())?;

    let layout_opts = LayoutOptions::default();
    let layout = ped
        .compute_layout(&layout_opts)
        .map_err(|e| e.to_string())?;

    let mut render_opts = RenderOptions::default();
    if parsed_directives.has_legend() {
        render_opts.legend = Some(parsed_directives.to_legend_config());
    }

    let svg = ped.render_svg(&layout, &render_opts);

    Ok((detected_format.to_string(), svg))
}
