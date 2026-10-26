//! # Joaju Wireframe Engine
//!
//! A native Rust parser and renderer for declarative UI mockups and wireframes,
//! compiling PlantUML Salt and modern wireframe syntax directly into native Typst layout.

use std::fmt::Write;

/// Escapes content for safe embedding in Typst string literals.
pub fn escape_typst_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            _ => out.push(c),
        }
    }
    out
}

#[derive(Debug, Clone, PartialEq)]
pub enum Widget {
    Text(String),
    Button(String),
    TextInput(String),
    Checkbox { label: String, checked: bool },
    Radio { label: String, selected: bool },
    Droplist(String),
    Tabs(Vec<String>),
    Badge(String),
    Toggle(bool),
    Progress { percent: u8, text: Option<String> },
    Separator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WireframeRow {
    Tabs(Vec<String>),
    Separator,
    Cells(Vec<Widget>),
}

/// Parses a single cell's textual representation into a `Widget`.
pub fn parse_widget(cell: &str) -> Widget {
    let trimmed = cell.trim();

    if trimmed == "--" || trimmed == "==" || trimmed == "---" {
        return Widget::Separator;
    }

    // Checkbox: [X] Label or [ ] Label
    if (trimmed.starts_with("[X]") || trimmed.starts_with("[x]") || trimmed.starts_with("[ ]"))
        && trimmed.len() >= 3
    {
        let checked = !trimmed.starts_with("[ ]");
        let label = trimmed[3..].trim().to_string();
        return Widget::Checkbox { label, checked };
    }

    // Radio: (X) Label or ( ) Label
    if (trimmed.starts_with("(X)") || trimmed.starts_with("(x)") || trimmed.starts_with("( )"))
        && trimmed.len() >= 3
    {
        let selected = !trimmed.starts_with("( )");
        let label = trimmed[3..].trim().to_string();
        return Widget::Radio { label, selected };
    }

    // Toggle switch: (O) [on] or (!) [off]
    if trimmed == "(O)" || trimmed == "(o)" {
        return Widget::Toggle(true);
    }
    if trimmed == "(!)" {
        return Widget::Toggle(false);
    }

    // Badge: <Label>
    if trimmed.starts_with('<') && trimmed.ends_with('>') && trimmed.len() >= 2 {
        let label = trimmed[1..trimmed.len() - 1].trim().to_string();
        return Widget::Badge(label);
    }

    // Droplist: ^Label^
    if trimmed.starts_with('^') && trimmed.ends_with('^') && trimmed.len() >= 2 {
        let label = trimmed[1..trimmed.len() - 1].trim().to_string();
        return Widget::Droplist(label);
    }

    // Text Input: "Value" or 'Value'
    if (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2)
        || (trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2)
    {
        let val = trimmed[1..trimmed.len() - 1].trim().to_string();
        return Widget::TextInput(val);
    }

    // Progress Bar: [====... 80%] or [====....]
    if trimmed.starts_with('[')
        && trimmed.ends_with(']')
        && (trimmed.contains('=') || trimmed.contains('-'))
        && (trimmed.contains('.') || trimmed.contains('%'))
    {
        let inner = trimmed[1..trimmed.len() - 1].trim();
        let percent = if let Some(pct_pos) = inner.find('%') {
            let num_part = inner[..pct_pos]
                .split_whitespace()
                .last()
                .unwrap_or("50")
                .trim_matches(|c: char| !c.is_ascii_digit());
            num_part.parse::<u8>().unwrap_or(50).min(100)
        } else {
            let equals = inner.chars().filter(|&c| c == '=').count();
            let total = inner.chars().filter(|&c| c == '=' || c == '.').count();
            if total > 0 {
                ((equals as f32 / total as f32) * 100.0) as u8
            } else {
                50
            }
        };
        return Widget::Progress {
            percent,
            text: Some(inner.to_string()),
        };
    }

    // Button: [ Label ]
    if trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed.len() >= 2 {
        let label = trimmed[1..trimmed.len() - 1].trim().to_string();
        return Widget::Button(label);
    }

    Widget::Text(trimmed.to_string())
}

/// Renders a single widget into its corresponding Typst layout fragment.
pub fn render_widget_typst(widget: &Widget) -> String {
    match widget {
        Widget::Text(text) => {
            format!(
                "[#align(left + horizon)[#text(size: 9.5pt, \"{}\")]]",
                escape_typst_string(text)
            )
        }
        Widget::Button(label) => {
            format!(
                "[#box(radius: 3pt, fill: luma(238), stroke: 0.75pt + luma(140), inset: (x: 8pt, y: 5pt))[#align(center + horizon)[#text(weight: \"bold\", size: 9pt, \"{}\")]]]",
                escape_typst_string(label)
            )
        }
        Widget::TextInput(val) => {
            let content = if val.is_empty() {
                "#text(fill: luma(160), style: \"italic\", size: 9pt, \"...\")".to_string()
            } else {
                format!(
                    "#text(fill: luma(40), size: 9pt, \"{}\")",
                    escape_typst_string(val)
                )
            };
            format!(
                "[#box(radius: 3pt, fill: luma(253), stroke: 0.75pt + luma(175), inset: (x: 7pt, y: 5pt), width: 100%)[#align(left + horizon)[{}]]]",
                content
            )
        }
        Widget::Checkbox { label, checked } => {
            let box_glyph = if *checked {
                "[#box(radius: 2pt, fill: luma(70), stroke: 0.75pt + luma(50), width: 10pt, height: 10pt)[#align(center + horizon)[#text(fill: white, size: 7.5pt, weight: \"bold\", \"✓\")]]]"
            } else {
                "[#box(radius: 2pt, fill: luma(250), stroke: 0.75pt + luma(140), width: 10pt, height: 10pt)[]]"
            };
            format!(
                "[#box[#grid(columns: (auto, auto), gutter: 5pt, align: horizon, {}, [#text(size: 9pt, \"{}\")])]]",
                box_glyph,
                escape_typst_string(label)
            )
        }
        Widget::Radio { label, selected } => {
            let radio_glyph = if *selected {
                "[#circle(radius: 5pt, fill: luma(250), stroke: 1.5pt + luma(70))[#align(center + horizon)[#circle(radius: 2pt, fill: luma(50))]]]"
            } else {
                "[#circle(radius: 5pt, fill: luma(250), stroke: 0.75pt + luma(140))]"
            };
            format!(
                "[#box[#grid(columns: (auto, auto), gutter: 5pt, align: horizon, {}, [#text(size: 9pt, \"{}\")])]]",
                radio_glyph,
                escape_typst_string(label)
            )
        }
        Widget::Droplist(label) => {
            format!(
                "[#box(radius: 3pt, fill: luma(250), stroke: 0.75pt + luma(160), inset: (x: 7pt, y: 5pt), width: 100%)[#grid(columns: (1fr, auto), align: horizon, [#text(size: 9pt, \"{}\")], [#text(size: 7.5pt, fill: luma(100), \"▼\")])]]",
                escape_typst_string(label)
            )
        }
        Widget::Tabs(tabs) => {
            let mut tabs_typst = Vec::new();
            for (idx, tab) in tabs.iter().enumerate() {
                if idx == 0 {
                    tabs_typst.push(format!(
                        "[#box(radius: (top: 3pt), fill: luma(255), stroke: (top: 1.5pt + luma(60), rest: 0.75pt + luma(180)), inset: (x: 8pt, y: 5pt))[#text(weight: \"bold\", size: 9pt, \"{}\")]]",
                        escape_typst_string(tab)
                    ));
                } else {
                    tabs_typst.push(format!(
                        "[#box(radius: (top: 3pt), fill: luma(238), stroke: 0.75pt + luma(195), inset: (x: 8pt, y: 5pt))[#text(fill: luma(100), size: 9pt, \"{}\")]]",
                        escape_typst_string(tab)
                    ));
                }
            }
            format!(
                "[#box(width: 100%, stroke: (bottom: 0.75pt + luma(180)))[#grid(columns: ({}), gutter: 3pt, {})]]",
                vec!["auto"; tabs.len()].join(", "),
                tabs_typst.join(", ")
            )
        }
        Widget::Badge(label) => {
            format!(
                "[#box(radius: 8pt, fill: rgb(\"f1f5f9\"), stroke: 0.5pt + rgb(\"94a3b8\"), inset: (x: 6pt, y: 3pt))[#text(fill: rgb(\"334155\"), size: 8pt, weight: \"bold\", \"{}\")]]",
                escape_typst_string(label)
            )
        }
        Widget::Toggle(on) => {
            if *on {
                "[#box(radius: 7pt, fill: rgb(\"22c55e\"), width: 22pt, height: 12pt, inset: 1.5pt)[#align(right + horizon)[#circle(radius: 4.5pt, fill: white)]]]".to_string()
            } else {
                "[#box(radius: 7pt, fill: luma(200), width: 22pt, height: 12pt, inset: 1.5pt)[#align(left + horizon)[#circle(radius: 4.5pt, fill: white)]]]".to_string()
            }
        }
        Widget::Progress { percent, .. } => {
            let pct = (*percent).min(100);
            format!(
                "[#box(width: 100%)[#grid(columns: (1fr, auto), gutter: 6pt, align: horizon, [#box(radius: 3pt, fill: luma(225), width: 100%, height: 9pt)[#place(top + left)[#box(radius: 3pt, fill: rgb(\"3b82f6\"), width: {}%, height: 9pt)]]], [#text(size: 8pt, fill: luma(80), \"{}%\")])]]",
                pct,
                pct
            )
        }
        Widget::Separator => "[#line(length: 100%, stroke: 0.5pt + luma(200))]".to_string(),
    }
}

/// Parses and compiles a declarative wireframe markup block into pure Typst code.
pub fn render_wireframe(source: &str) -> Result<String, String> {
    let mut clean_lines: Vec<&str> = Vec::new();
    let custom_width = "85%";
    let mut is_window = false;
    let mut window_title: Option<String> = None;

    for line in source.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }

        // Configuration directives
        if t.starts_with("%%") {
            continue;
        }

        // Salt Window container opening
        if let Some(rest) = t.strip_prefix("{+").or_else(|| t.strip_prefix('{')) {
            is_window = true;
            let after_brace = rest.trim();
            if !after_brace.is_empty() && after_brace != "}" {
                // Check if title is on first line
                if after_brace.starts_with('"')
                    && after_brace.ends_with('"')
                    && after_brace.len() >= 2
                {
                    window_title = Some(after_brace[1..after_brace.len() - 1].trim().to_string());
                } else if let Some(inner) = after_brace.strip_prefix('"') {
                    if let Some(end_quote) = inner.find('"') {
                        window_title = Some(inner[..end_quote].to_string());
                        let rest_after = inner[end_quote + 1..].trim();
                        if !rest_after.is_empty() && rest_after != "}" {
                            clean_lines.push(rest_after);
                        }
                    }
                } else {
                    clean_lines.push(after_brace);
                }
            }
            continue;
        }

        if t == "}" || t == "}+" {
            continue;
        }

        clean_lines.push(t);
    }

    if clean_lines.is_empty() && window_title.is_none() {
        return Ok("#box[]".to_string());
    }

    let mut rows: Vec<WireframeRow> = Vec::new();

    for line in clean_lines {
        let trimmed = line.trim();

        // Check if row is a horizontal divider
        if trimmed == "--" || trimmed == "==" || trimmed == "---" {
            rows.push(WireframeRow::Separator);
            continue;
        }

        // Check if row is Tabs declaration: {/ Geral | Segurança | Notificações }
        if trimmed.starts_with("{/") && trimmed.ends_with('}') {
            let inner = trimmed[2..trimmed.len() - 1].trim();
            let tabs: Vec<String> = inner
                .split('|')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !tabs.is_empty() {
                rows.push(WireframeRow::Tabs(tabs));
                continue;
            }
        }

        // Regular grid line: split by |
        let cells: Vec<Widget> = line.split('|').map(parse_widget).collect();

        rows.push(WireframeRow::Cells(cells));
    }

    let mut typst = String::new();
    writeln!(
        typst,
        "#align(center)[#block(breakable: false, width: {})[",
        custom_width
    )
    .unwrap();

    // Outer Window Box
    writeln!(
        typst,
        "  #box(radius: 4pt, fill: luma(252), stroke: 0.85pt + luma(170), inset: 0pt, width: 100%, clip: true)["
    ).unwrap();

    // Window Header (if window container or title exists)
    if is_window || window_title.is_some() {
        let title = window_title.as_deref().unwrap_or("Janela de Interface");
        writeln!(
            typst,
            "    #box(width: 100%, fill: luma(238), stroke: (bottom: 0.75pt + luma(180)), inset: (x: 10pt, y: 7pt))["
        ).unwrap();
        writeln!(
            typst,
            "      #grid(columns: (auto, 1fr, auto), align: horizon, [#box[#grid(columns: (auto, auto, auto), gutter: 4pt, [#circle(radius: 3pt, fill: rgb(\"f87171\"))], [#circle(radius: 3pt, fill: rgb(\"facc15\"))], [#circle(radius: 3pt, fill: rgb(\"4ade80\"))])]], [#align(center)[#text(size: 9.5pt, weight: \"bold\", fill: luma(60), \"{}\")]], [])",
            escape_typst_string(title)
        ).unwrap();
        writeln!(typst, "    ]").unwrap();
    }

    // Window Body
    writeln!(typst, "    #pad(x: 12pt, top: 10pt, bottom: 12pt)[").unwrap();

    for row in rows {
        match row {
            WireframeRow::Separator => {
                writeln!(
                    typst,
                    "      #v(3pt)#line(length: 100%, stroke: 0.5pt + luma(200))#v(3pt)"
                )
                .unwrap();
            }
            WireframeRow::Tabs(tabs) => {
                writeln!(
                    typst,
                    "      {}#v(6pt)",
                    render_widget_typst(&Widget::Tabs(tabs))
                )
                .unwrap();
            }
            WireframeRow::Cells(cells) => {
                let col_widths = vec!["1fr"; cells.len()].join(", ");
                let mut rendered_cells = Vec::new();
                for c in &cells {
                    rendered_cells.push(render_widget_typst(c));
                }
                writeln!(
                    typst,
                    "      #grid(columns: ({}), gutter: 8pt, align: horizon, {})",
                    col_widths,
                    rendered_cells.join(", ")
                )
                .unwrap();
                writeln!(typst, "      #v(3pt)").unwrap();
            }
        }
    }

    writeln!(typst, "    ]").unwrap();
    writeln!(typst, "  ]").unwrap();
    writeln!(typst, "]]").unwrap();

    Ok(typst)
}
