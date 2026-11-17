# Especificação Técnica — Integração do joaju-markdown com Paridade Obsidian (Ticket #87)

## 1. Escopo e Propósito

Esta especificação define a integração da versão `0.1.0-dev.34` da crate `joaju-markdown`
no compilador HTML do `joaju`, habilitando o processamento semântico nativo de:
- Callouts do Obsidian (`> [!tipo] [Título]`) com ícones SVG embutidos
- Destaques / Marca-texto (`==termo==` -> `<mark>termo</mark>`)
- Wikilinks textuais (`[[Nota]]`, `[[Nota|Apelido]]` -> `<a class="internal-link" data-href="..." href="...">`)
- Comentários Obsidian (`%%...%%` -> suprimidos da saída final)

## 2. Requisitos Técnicos

1. `Cargo.toml`: Atualizar a dependência `joaju-markdown` para a versão exata publicada no registro `gambi`.
2. `src/compiler/html.rs`: Integrar o pré-processamento de Markdown com `joaju_markdown::preprocess_markdown_html`
   antes da extração dos blocos customizados e do parser pulldown-cmark.
3. Testes de Especificação: Validar que `compile_markdown_to_html` converte callouts com ícones SVG,
   wikilinks com `data-href`, tags `<mark>` e remove comentários `%%...%%`.
