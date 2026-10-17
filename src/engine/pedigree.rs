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

    let (ped, directives) = Ped::from_text(content).map_err(|e| e.to_string())?;

    let layout_opts = LayoutOptions::default();
    let layout = ped
        .compute_layout(&layout_opts)
        .map_err(|e| e.to_string())?;

    let mut render_opts = RenderOptions::default();
    if directives.has_legend() {
        render_opts.legend = Some(directives.to_legend_config());
    }

    let svg = ped.render_svg(&layout, &render_opts);

    Ok((detected_format.to_string(), svg))
}
