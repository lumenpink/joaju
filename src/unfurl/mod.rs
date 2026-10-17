use crate::compiler::compile_markdown_to_pdf;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Url;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct UnfurlRequest {
    pub url: Option<String>,
    #[serde(default)]
    pub archive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfurlResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "siteName", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "faviconUrl", skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "readingTimeMin", skip_serializing_if = "Option::is_none")]
    pub reading_time_min: Option<u32>,
    #[serde(rename = "hasArticle", skip_serializing_if = "Option::is_none")]
    pub has_article: Option<bool>,
    #[serde(rename = "pdfBase64", skip_serializing_if = "Option::is_none")]
    pub pdf_base64: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfurlErrorResponse {
    pub ok: bool,
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedMetadata {
    pub domain: String,
    pub site_name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub favicon_url: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub reading_time_min: u32,
    pub has_article: bool,
    pub pdf_base64: Option<String>,
}

fn resolve_url(base: &Url, relative_or_absolute: &str) -> String {
    if let Ok(resolved) = base.join(relative_or_absolute) {
        resolved.to_string()
    } else {
        relative_or_absolute.to_string()
    }
}

pub fn extract_metadata_from_html(
    html: &str,
    target_url: &str,
    archive: bool,
) -> Result<ExtractedMetadata, String> {
    let parsed_url = Url::parse(target_url).map_err(|e| format!("URL inválida: {}", e))?;
    let domain = parsed_url
        .host_str()
        .map(|h| h.trim_start_matches("www.").to_string())
        .unwrap_or_else(|| "desconhecido".to_string());

    let document = Html::parse_document(html);

    // Helpers para seleção
    let get_meta = |property_or_name: &str, val: &str| -> Option<String> {
        let selector_str = format!("meta[{}='{}']", property_or_name, val);
        let selector = Selector::parse(&selector_str).ok()?;
        document
            .select(&selector)
            .next()?
            .value()
            .attr("content")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    };

    let title = get_meta("property", "og:title")
        .or_else(|| get_meta("name", "twitter:title"))
        .or_else(|| {
            let selector = Selector::parse("title").ok()?;
            document
                .select(&selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .filter(|s| !s.is_empty())
        });

    let description = get_meta("property", "og:description")
        .or_else(|| get_meta("name", "twitter:description"))
        .or_else(|| get_meta("name", "description"));

    let image_url = get_meta("property", "og:image")
        .or_else(|| get_meta("name", "twitter:image"))
        .map(|img| resolve_url(&parsed_url, &img));

    let site_name = get_meta("property", "og:site_name").or_else(|| Some(domain.clone()));

    let author = get_meta("property", "article:author")
        .or_else(|| get_meta("name", "author"))
        .or_else(|| get_meta("name", "byl"));

    let date = get_meta("property", "article:published_time")
        .or_else(|| get_meta("name", "date"))
        .or_else(|| get_meta("name", "pubdate"));

    let favicon_url = {
        let mut fav = None;
        if let Ok(sel) = Selector::parse("link[rel='icon'], link[rel='shortcut icon']") {
            if let Some(el) = document.select(&sel).next() {
                if let Some(href) = el.value().attr("href") {
                    fav = Some(resolve_url(&parsed_url, href));
                }
            }
        }
        fav.or_else(|| parsed_url.join("/favicon.ico").ok().map(|u| u.to_string()))
    };

    // Artigo e texto
    let mut article_text = String::new();
    let mut has_article = false;

    if let Ok(sel) = Selector::parse("article") {
        if let Some(art) = document.select(&sel).next() {
            has_article = true;
            article_text = art.text().collect::<Vec<_>>().join(" ");
        }
    }

    if article_text.is_empty() {
        if let Ok(sel) = Selector::parse("body") {
            if let Some(body) = document.select(&sel).next() {
                article_text = body.text().collect::<Vec<_>>().join(" ");
            }
        }
    }

    let word_count = article_text.split_whitespace().count();
    let reading_time_min = if word_count > 0 {
        std::cmp::max(1, word_count.div_ceil(200) as u32)
    } else {
        1
    };

    let mut pdf_base64 = None;
    if archive {
        let doc_title = title.as_deref().unwrap_or("Artigo Arquivado");
        let doc_author = author.as_deref().unwrap_or(&domain);
        let content_paragraphs = article_text
            .split("\n\n")
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n");

        let markdown = format!(
            "---\ntitulo: {}\nautor: {}\n---\n\n# {}\n\n{}\n",
            doc_title, doc_author, doc_title, content_paragraphs
        );

        if let Ok(pdf_bytes) = compile_markdown_to_pdf(&markdown) {
            pdf_base64 = Some(general_purpose::STANDARD.encode(pdf_bytes));
        }
    }

    Ok(ExtractedMetadata {
        domain,
        site_name,
        title,
        description,
        image_url,
        favicon_url,
        author,
        date,
        reading_time_min,
        has_article,
        pdf_base64,
    })
}

pub async fn unfurl_url(target_url: &str, archive: bool) -> Result<UnfurlResponse, String> {
    let parsed_url = Url::parse(target_url).map_err(|e| format!("URL inválida: {}", e))?;

    if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
        return Err("Protocolo inválido. Apenas http e https são permitidos.".to_string());
    }

    if let Some(host) = parsed_url.host_str() {
        let host_lower = host.to_lowercase();
        if host_lower == "localhost"
            || host_lower == "127.0.0.1"
            || host_lower == "::1"
            || host_lower.starts_with("192.168.")
            || host_lower.starts_with("10.")
            || host_lower.starts_with("172.16.")
            || host_lower.ends_with(".local")
        {
            return Err("Acesso a endereços locais ou privados não é permitido.".to_string());
        }
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(|e| format!("Erro ao criar cliente HTTP: {}", e))?;

    let response = client
        .get(parsed_url.as_str())
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
        )
        .header(
            reqwest::header::ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .send()
        .await
        .map_err(|e| format!("Erro ao consultar URL: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Servidor retornou HTTP {}", response.status()));
    }

    let html = response
        .text()
        .await
        .map_err(|e| format!("Erro ao ler corpo da resposta: {}", e))?;

    let meta = extract_metadata_from_html(&html, target_url, archive)?;

    Ok(UnfurlResponse {
        ok: true,
        error: None,
        url: Some(target_url.to_string()),
        domain: Some(meta.domain),
        site_name: meta.site_name,
        title: meta.title,
        description: meta.description,
        image_url: meta.image_url,
        favicon_url: meta.favicon_url,
        author: meta.author,
        date: meta.date,
        reading_time_min: Some(meta.reading_time_min),
        has_article: Some(meta.has_article),
        pdf_base64: meta.pdf_base64,
    })
}
