use regex::Regex;

pub fn preprocess_images(markdown: &str) -> String {
    // 1. Wikilinks com dimensão: ![[imagem.png|450]] ou ![[imagem.png|450px]] ou ![[imagem.png|50%]]
    let re_wiki_dim = Regex::new(r"!\[\[([^|\]\n]+)\|([^\]\n]+)\]\]").unwrap();
    let res1 = re_wiki_dim.replace_all(markdown, |caps: &regex::Captures| {
        let file = caps.get(1).unwrap().as_str().trim();
        let dim_str = caps.get(2).unwrap().as_str().trim();
        let formatted_dim = if dim_str.chars().all(|c| c.is_ascii_digit()) {
            format!("{}pt", dim_str)
        } else {
            dim_str.to_string()
        };
        format!(r#"#image("{}", width: {})"#, file, formatted_dim)
    });

    // 2. Wikilinks simples: ![[imagem.png]]
    let re_wiki_simple = Regex::new(r"!\[\[([^|\]\n]+)\]\]").unwrap();
    let res2 = re_wiki_simple.replace_all(&res1, |caps: &regex::Captures| {
        let file = caps.get(1).unwrap().as_str().trim();
        format!(r#"#image("{}")"#, file)
    });

    // 3. Imagens padrão Markdown: ![alt](figura.png)
    let re_md_img = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap();
    let res3 = re_md_img.replace_all(&res2, |caps: &regex::Captures| {
        let file = caps.get(2).unwrap().as_str().trim();
        format!(r#"#image("{}")"#, file)
    });

    res3.to_string()
}
