pub use joaju_refs::ReferenceNote;
use joaju_refs::{parse_reference_note, process_references};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Fonte de referências bibliográficas para resolução no documento.
#[derive(Debug, Clone)]
pub enum ReferenceSource<'a> {
    /// Resolução tradicional lendo notas .md em diretório (padrão CLI).
    FileSystem(Option<&'a Path>),
    /// Referências injetadas em memória (ex.: SQLite do Mandu, WASM, testes).
    Memory(&'a HashMap<String, ReferenceNote>),
    /// Prioriza notas em memória; usa disco como fallback para chaves ausentes.
    Hybrid {
        memory: &'a HashMap<String, ReferenceNote>,
        base_dir: Option<&'a Path>,
    },
}

impl<'a> From<&'a HashMap<String, ReferenceNote>> for ReferenceSource<'a> {
    fn from(map: &'a HashMap<String, ReferenceNote>) -> Self {
        ReferenceSource::Memory(map)
    }
}

impl<'a> From<&'a Path> for ReferenceSource<'a> {
    fn from(path: &'a Path) -> Self {
        ReferenceSource::FileSystem(Some(path))
    }
}

/// Indexa recursivamente uma pasta de referências mapeando o stem do arquivo (sem `.md`)
/// para o caminho completo em disco. Não abre arquivos nem consome CPU com parsing de YAML.
fn index_markdown_filenames(dir: &Path) -> (HashMap<String, PathBuf>, Vec<PathBuf>) {
    let mut map = HashMap::new();
    let mut all_files = Vec::new();
    collect_md_files(dir, &mut map, &mut all_files);
    (map, all_files)
}

fn collect_md_files(dir: &Path, map: &mut HashMap<String, PathBuf>, all_files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_md_files(&path, map, all_files);
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        map.entry(stem.to_string()).or_insert_with(|| path.clone());
                    }
                    all_files.push(path);
                }
            }
        }
    }
}

/// Extrai possíveis chaves de citação mencionadas no Markdown (`[[citekey]]`, `[[refs/citekey]]`, `@citekey`, `[@citekey]`).
fn extract_candidate_citekeys(markdown: &str) -> Vec<String> {
    let mut candidates = Vec::new();

    // 1. Wikilinks: [[citekey]], [[refs/citekey]], [[citekey|p. 42]]
    if let Ok(re_wiki) = Regex::new(r"\[\[([a-zA-Z0-9_\-\.\/]+)(?:\|[^\]]*)?\]\]") {
        for caps in re_wiki.captures_iter(markdown) {
            if let Some(target) = caps.get(1) {
                let s = target.as_str().trim();
                let stem = Path::new(s)
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or(s);
                candidates.push(stem.to_string());
            }
        }
    }

    // 2. Pandoc: @citekey ou [@citekey, p. 15]
    if let Ok(re_pandoc) = Regex::new(r"@([a-zA-Z0-9_\-]+)") {
        for caps in re_pandoc.captures_iter(markdown) {
            if let Some(k) = caps.get(1) {
                candidates.push(k.as_str().trim().to_string());
            }
        }
    }

    candidates
}

/// Analisa o frontmatter para obter diretório personalizado de referências (`refs_dir`)
/// e configuração de estilo no nível raiz (`estilo:`).
fn parse_frontmatter_options(markdown: &str) -> (Option<String>, Option<String>) {
    let trimmed = markdown.trim_start();
    if !trimmed.starts_with("---") {
        return (None, None);
    }
    let after_first = &trimmed[3..];
    let Some(end_idx) = after_first.find("\n---") else {
        return (None, None);
    };
    let yaml_str = &after_first[..end_idx];

    let mut refs_dir = None;
    let mut estilo = None;

    for line in yaml_str.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("refs_dir:") {
            let val = rest.trim().trim_matches('"').trim_matches('\'');
            if !val.is_empty() {
                refs_dir = Some(val.to_string());
            }
        } else if let Some(rest) = t.strip_prefix("estilo:") {
            let val = rest.trim().trim_matches('"').trim_matches('\'');
            if !val.is_empty() {
                estilo = Some(val.to_string());
            }
        }
    }

    (refs_dir, estilo)
}

/// Normaliza o frontmatter garantindo compatibilidade quando `estilo:` for declarado no topo.
fn inject_style_if_needed(markdown: &str, top_level_estilo: Option<&str>) -> String {
    let Some(estilo) = top_level_estilo else {
        return markdown.to_string();
    };
    if markdown.contains("referencias:") {
        return markdown.to_string();
    }
    let trimmed = markdown.trim_start();
    if !trimmed.starts_with("---") {
        return markdown.to_string();
    }
    let after_first = &trimmed[3..];
    if let Some(end_idx) = after_first.find("\n---") {
        let frontmatter = &after_first[..end_idx];
        let rest = &after_first[end_idx..];
        format!(
            "---\n{}\nreferencias:\n  estilo: {}{}",
            frontmatter, estilo, rest
        )
    } else {
        markdown.to_string()
    }
}

fn resolve_from_disk_dir(
    candidates: &[String],
    effective_refs_dir: &Path,
    resolved: &mut HashMap<String, ReferenceNote>,
) {
    if !effective_refs_dir.exists() {
        return;
    }
    let (filename_index, all_files) = index_markdown_filenames(effective_refs_dir);
    let mut parsed_fallback_files = false;
    let mut all_parsed_notes: Vec<ReferenceNote> = Vec::new();

    for key in candidates {
        if resolved.contains_key(key) {
            continue;
        }

        // Caminho expresso: correspondência direta no mapa de nomes (O(1))
        if let Some(path) = filename_index.get(key) {
            if let Ok(content) = fs::read_to_string(path) {
                if let Some(note) = parse_reference_note(&content) {
                    resolved.insert(key.clone(), note);
                    continue;
                }
            }
        }

        // Caminho lento: arquivo renomeado (varredura nos arquivos restantes de refs/)
        if !parsed_fallback_files {
            for file_path in &all_files {
                if let Ok(content) = fs::read_to_string(file_path) {
                    if let Some(note) = parse_reference_note(&content) {
                        all_parsed_notes.push(note);
                    }
                }
            }
            parsed_fallback_files = true;
        }

        if let Some(matched) = all_parsed_notes.iter().find(|n| n.citekey == *key) {
            resolved.insert(key.clone(), matched.clone());
        }
    }
}

/// Resolve referências bibliográficas do documento a partir de notas Markdown em `refs_dir`.
/// Utiliza estratégia de alta performance com índice de nomes e fallback de varredura lenta.
pub fn resolve_and_process_references(markdown: &str, base_dir: Option<&Path>) -> String {
    let base = base_dir.unwrap_or_else(|| Path::new("."));

    let (custom_refs_dir, top_level_estilo) = parse_frontmatter_options(markdown);
    let refs_folder_name = custom_refs_dir.as_deref().unwrap_or("refs");
    let effective_refs_dir = base.join(refs_folder_name);

    if !effective_refs_dir.exists() {
        return markdown.to_string();
    }

    let candidates = extract_candidate_citekeys(markdown);
    if candidates.is_empty() {
        return markdown.to_string();
    }

    let mut resolved_refs: HashMap<String, ReferenceNote> = HashMap::new();
    resolve_from_disk_dir(&candidates, &effective_refs_dir, &mut resolved_refs);

    if resolved_refs.is_empty() {
        return markdown.to_string();
    }

    let prepared_markdown = inject_style_if_needed(markdown, top_level_estilo.as_deref());
    process_references(&prepared_markdown, &resolved_refs)
}

/// Resolve referências bibliográficas a partir de uma [`ReferenceSource`].
pub fn resolve_references_with_source(markdown: &str, source: Option<ReferenceSource>) -> String {
    let Some(source) = source else {
        return markdown.to_string();
    };

    let (_custom_refs_dir, top_level_estilo) = parse_frontmatter_options(markdown);
    let prepared_markdown = inject_style_if_needed(markdown, top_level_estilo.as_deref());

    match source {
        ReferenceSource::Memory(map) => {
            if map.is_empty() {
                markdown.to_string()
            } else {
                process_references(&prepared_markdown, map)
            }
        }
        ReferenceSource::FileSystem(base_dir) => resolve_and_process_references(markdown, base_dir),
        ReferenceSource::Hybrid { memory, base_dir } => {
            let candidates = extract_candidate_citekeys(markdown);
            if candidates.is_empty() {
                return markdown.to_string();
            }

            let mut resolved: HashMap<String, ReferenceNote> = HashMap::new();
            for k in &candidates {
                if let Some(note) = memory.get(k) {
                    resolved.insert(k.clone(), note.clone());
                }
            }

            let missing: Vec<String> = candidates
                .into_iter()
                .filter(|k| !resolved.contains_key(k))
                .collect();

            if !missing.is_empty() {
                let base = base_dir.unwrap_or_else(|| Path::new("."));
                let (custom_refs_dir, _) = parse_frontmatter_options(markdown);
                let refs_folder_name = custom_refs_dir.as_deref().unwrap_or("refs");
                let effective_refs_dir = base.join(refs_folder_name);
                resolve_from_disk_dir(&missing, &effective_refs_dir, &mut resolved);
            }

            if resolved.is_empty() {
                markdown.to_string()
            } else {
                process_references(&prepared_markdown, &resolved)
            }
        }
    }
}
