use regex::Regex;
use std::collections::HashMap;

pub fn render_single_table(raw_content: &str) -> Result<String, String> {
    let mut directives = HashMap::new();
    let mut clean_lines = Vec::new();

    for line in raw_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("%%") {
            let body = trimmed.trim_start_matches('%').trim();
            if let Some((k, v)) = body.split_once('=') {
                directives.insert(k.trim().to_lowercase(), v.trim().to_string());
            }
        } else if !trimmed.is_empty() {
            // Ignora réguas de markdown |---|---|
            if trimmed.contains("---") && (trimmed.contains('|') || trimmed.contains('+')) {
                continue;
            }
            clean_lines.push(trimmed);
        }
    }

    if clean_lines.is_empty() {
        return Err("Tabela vazia".to_string());
    }

    // Detecta delimitador: '|', '\t' ou ','
    let delimiter = if clean_lines[0].contains('|') {
        '|'
    } else if clean_lines[0].contains('\t') {
        '\t'
    } else {
        ','
    };

    let mut rows: Vec<Vec<String>> = Vec::new();
    for line in clean_lines {
        let cells: Vec<String> = line
            .split(delimiter)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if !cells.is_empty() {
            rows.push(cells);
        }
    }

    if rows.is_empty() {
        return Err("Nenhuma célula encontrada na tabela".to_string());
    }

    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(1);
    let header_cells = &rows[0];
    let data_rows = &rows[1..];

    let caption = directives
        .get("caption")
        .or_else(|| directives.get("titulo"))
        .cloned();
    let source = directives
        .get("source")
        .or_else(|| directives.get("fonte"))
        .cloned();
    let note = directives
        .get("note")
        .or_else(|| directives.get("nota"))
        .cloned();

    let mut out = String::new();

    if let Some(ref cap) = caption {
        out.push_str(&format!(
            "#figure(\n  caption: [{}],\n  supplement: [Tabela],\n)[\n",
            cap
        ));
    }

    out.push_str("  #align(center)[\n");
    out.push_str(&format!(
        "    #table(\n      columns: {},\n      stroke: none,\n      table.hline(stroke: 1.2pt),\n      table.header(\n",
        num_cols
    ));

    // Cabeçalho
    out.push_str("        ");
    for (i, cell) in header_cells.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("[{}]", cell));
    }
    out.push_str("\n      ),\n      table.hline(stroke: 0.6pt),\n");

    // Linhas de dados
    for row in data_rows {
        out.push_str("      ");
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&format!("[{}]", cell));
        }
        out.push_str(",\n");
    }

    out.push_str("      table.hline(stroke: 1.2pt),\n    )\n  ]\n");

    if source.is_some() || note.is_some() {
        out.push_str("  #v(-4pt)\n  #align(left)[\n    #set text(size: 8.5pt)\n");
        if let Some(ref s) = source {
            out.push_str(&format!("    Fonte: {}\n", s));
        }
        if let Some(ref n) = note {
            out.push_str(&format!("    Nota: {}\n", n));
        }
        out.push_str("  ]\n");
    }

    if caption.is_some() {
        out.push_str("]\n");
    }

    Ok(out)
}

pub fn preprocess_tables(markdown: &str) -> String {
    let re = Regex::new(r"(?s)```table\s*\n(.*?)```").unwrap();
    re.replace_all(markdown, |caps: &regex::Captures| {
        let content = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        match render_single_table(content) {
            Ok(typst) => format!("\n{}\n", typst.trim()),
            Err(_) => caps.get(0).unwrap().as_str().to_string(),
        }
    })
    .to_string()
}
