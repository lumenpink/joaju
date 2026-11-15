# Especificação 85: Integração de Motores Lírica, Diagramas SVG, Figuras e Envelopes de Código no Compilador HTML

- **ID do Ticket:** #85
- **Status:** Em Progresso (DD-TDD Fase 3)
- **Autora:** Lumen
- **Alinhamento:** GOST 3.0.0 (Rust)

## 1. Contexto & Motivação
O Joaju é o compilador e orquestrador tipográfico unificado (SSOT) do ecossistema. Na especificação #83, introduziu-se a função pública `compile_markdown_to_html` em `src/compiler/html.rs`, contemplando tabelas tipográficas (`joaju-table`), heredogramas clínicos (`joaju-pedigree`) e estruturas químicas (`joaju-chem`).

Entretanto, motores essenciais do ecossistema ficaram restritos ao modo Typst/PDF e ausentes do modo HTML:
1. `lirica`: blocos poéticos e de escansão métrica vazavam para CommonMark cru sem a análise de sílabas, métrica e rimas.
2. `wireframe` / `salt`: diagramas de interfaces mockups suportados pelo `joaju-wireframe` não emitiam SVG no HTML.
3. `sequence`: diagramas de sequência suportados pelo `joaju-sequence` não emitiam SVG no HTML.
4. `graph` / `flowchart`: grafos e fluxogramas suportados pelo `joaju-graph` não emitiam SVG no HTML.
5. Blocos de código genéricos: emitiam tags `<pre><code>` sem envelope semântico, distintivo de linguagem ou cabeçalho de cartão.
6. Figuras: imagens em Markdown (`![legenda](url)`) não eram transformadas em elementos semânticos estruturados `<figure>` e `<figcaption>`.

Para que clientes consumidores (como o Mandu Mobile, Mandu Desktop e Tachinha) tenham paridade visual absoluta e atualização contínua sem reimplementar parsers locais, o Joaju deve tratar todos esses blocos nativamente em `compile_markdown_to_html`.

## 2. Requisitos Técnicos

### 2.1 Motor de Lírica (`lirica`, `poema`, `poesia`)
- Interceptar blocos com delimitadores ````lirica````, ````poema```` e ````poesia````.
- Executar a análise com `joaju_lirica::parse_lirica_block` e renderizar o painel de escansão métrica (`render_painel`).
- Emitir contêiner HTML estilizado:
  ```html
  <div class="joaju-lirica-card" data-engine="lirica">
    <div class="joaju-card-header"><span class="joaju-card-badge">✒ Lírica</span></div>
    <pre class="joaju-scansion"><code>...</code></pre>
  </div>
  ```
- Em caso de falha de parsing, manter fallback seguro sem interromper a compilação do restante do documento.

### 2.2 Motores de Diagramas Nativos em SVG
- **Wireframes:** interceptar ````(wireframe|salt|ui)```` e renderizar SVG via `joaju_wireframe::render_wireframe_svg`.
- **Sequência:** interceptar ````(sequence|sequencediagram)```` e renderizar SVG via `joaju_sequence::render_sequence_svg`.
- **Grafos:** interceptar ````(graph|flowchart)```` e renderizar SVG via `joaju_graph::render_graph_svg`.
- Envelopar os SVGs em contêineres identificados:
  ```html
  <div class="joaju-diagram-block" data-engine="...">...svg...</div>
  ```

### 2.3 Cartões Semânticos de Código e Typst
- Interceptar blocos de código ````(typst|joaju)```` identificando-os como código tipográfico especializado:
  ```html
  <div class="joaju-code-block joaju-typst-block" data-language="typst">
    <div class="joaju-code-header"><span class="joaju-code-badge">typst</span></div>
    <pre class="language-typst"><code class="language-typst">...</code></pre>
  </div>
  ```
- Blocos de código genéricos demarcados com linguagem ````{lang}```` devem receber contêiner estruturado:
  ```html
  <div class="joaju-code-block" data-language="{lang}">
    <div class="joaju-code-header"><span class="joaju-code-badge">{lang}</span></div>
    <pre class="language-{lang}"><code class="language-{lang}">...</code></pre>
  </div>
  ```

### 2.4 Figuras Semânticas
- Converter tags de imagem Markdown `![legenda](url)` para HTML acadêmico com legenda:
  ```html
  <figure class="joaju-figure">
    <img src="url" alt="legenda" />
    <figcaption class="joaju-figcaption">legenda</figcaption>
  </figure>
  ```
- Se a legenda estiver vazia `![](url)`, renderizar a imagem padrão sem `<figcaption>` vazio.

## 3. Critérios de Aceitação
1. Suíte de testes em `tests/html_engines_spec.rs` cobrindo:
   - Renderização de bloco `lirica` com painel de escansão.
   - Renderização de `wireframe` em SVG inline.
   - Renderização de `sequence` em SVG inline.
   - Renderização de `graph`/`flowchart` em SVG inline.
   - Renderização de bloco `typst` e código genérico em cartões estruturados.
   - Renderização de imagem em `<figure>` e `<figcaption>`.
2. Todos os testes existentes em `tests/html_spec.rs` continuam passando sem regressões.
3. CLI `joaju render-html` reflete imediatamente todos os novos blocos.
4. Exportação C-ABI `joaju_compile_markdown_to_html` reflete os novos blocos.
5. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem avisos.
