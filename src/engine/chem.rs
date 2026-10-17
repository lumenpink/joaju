use regex::Regex;

pub fn render_chem(
    raw_code: &str,
    forced_format: Option<&str>,
) -> Result<(String, String), String> {
    let re_fence = Regex::new(r"(?s)```(openchemlib|openbabel)\s*\n(.*?)```").unwrap();
    let (detected_format, content) = if let Some(caps) = re_fence.captures(raw_code) {
        let fmt = caps.get(1).map(|m| m.as_str()).unwrap_or("openchemlib");
        let body = caps.get(2).map(|m| m.as_str()).unwrap_or(raw_code);
        (fmt, body)
    } else {
        (forced_format.unwrap_or("openchemlib"), raw_code)
    };

    let clean_smiles: String = content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.starts_with("%%") && !l.is_empty())
        .collect::<Vec<_>>()
        .join("");

    if clean_smiles.is_empty() {
        return Err("Código SMILES vazio ou não fornecido".to_string());
    }

    let mol = chematic_smiles::parse(&clean_smiles)
        .map_err(|e| format!("Erro no parsing SMILES: {:?}", e))?;
    let layout = chematic_depict::compute_layout(&mol);
    let svg = chematic_depict::render_svg(&mol, &layout);

    Ok((detected_format.to_string(), svg))
}
