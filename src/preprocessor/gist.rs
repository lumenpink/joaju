use regex::Regex;

pub fn preprocess_gists(markdown: &str) -> String {
    let re = Regex::new(r"(?s)```gist\s*\n(.*?)```").unwrap();
    re.replace_all(markdown, |caps: &regex::Captures| {
        let content = caps.get(1).map(|m| m.as_str()).unwrap_or("").trim();
        format!("```\n{}\n```", content)
    })
    .to_string()
}
