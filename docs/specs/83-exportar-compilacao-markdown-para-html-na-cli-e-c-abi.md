# Especificação 83: Exportação de Compilação Markdown para HTML na CLI e C-ABI

- **ID do Ticket:** #83
- **Status:** Em Progresso (DD-TDD Fase 3)
- **Autora:** Lumen
- **Alinhamento:** GOST 3.0.0 (Rust)

## 1. Contexto & Motivação
O compilador acadêmico e tipográfico Joaju (`joaju`) dispõe de submódulos especializados para tabelas ABNT/IBGE (`joaju-table`), estruturas genealógicas (`joaju-pedigree`), notações químicas (`joaju-chem`) e transformações textuais (`joaju-markdown`).

Entretanto, para servir como renderizador externo do Forgejo (`[markup.markdown]` no `app.ini`) e como motor de renderização do gerador estático Tachinha sem forçar recompilação integral de dependências Typst nos clientes, o Joaju precisa:
1. Oferecer uma API Rust pública e canônica de compilação de Markdown completo para HTML (`compile_markdown_to_html`).
2. Disponibilizar o subcomando na CLI `joaju render-html [INPUT] [-o OUTPUT]` com suporte pleno a leitura via `stdin` e escrita em `stdout`.
3. Exportar a função C-ABI `joaju_compile_markdown_to_html` em `src/ffi.rs` para linkagem estática (`libjoaju.a`) ou dinâmica por outros consumidores do ecossistema.

## 2. Requisitos Técnicos
1. **API Rust Pública:**
   - `pub fn compile_markdown_to_html(markdown: &str) -> Result<String, String>` exposta na raiz de `joaju`.
   - Processamento determinístico de blocos de código:
     - ````table```` com dados CSV/TSV: converte para tabela HTML tipográfica estilizada via `joaju_table::render_html_table`.
     - ````pedtools```` / ````kinship2````: converte para SVG inline via `joaju::render_pedigree`.
     - ````chem```` / ````openchemlib````: converte para SVG inline via `joaju::render_chem`.
     - Tabelas e elementos padrão Markdown: compilados para HTML semântico seguro.

2. **Subcomando CLI:**
   - `joaju render-html [INPUT] [-o OUTPUT]`
   - Se `INPUT` omitido ou `-`, lê de `stdin`.
   - Se `OUTPUT` omitido ou `-`, emite para `stdout`.
   - Compatibilidade direta com o backend de renderizadores externos do Forgejo.

3. **Interface C-ABI Estável:**
   - Em `src/ffi.rs`:
     ```c
     int32_t joaju_compile_markdown_to_html(
         const char* markdown,
         char** out_html,
         char** out_error
     );
     ```
   - Liberação de memória gerada via `joaju_free_string`.

## 3. Critérios de Aceitação
1. Compilação de documento Markdown contendo tabelas e blocos ricos para HTML validada em testes unitários.
2. CLI `joaju render-html` lendo de `stdin` e escrevendo no `stdout`.
3. Função C-ABI `joaju_compile_markdown_to_html` funcional e testada contra vazamentos de memória e ponteiros nulos.
4. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem advertências.
