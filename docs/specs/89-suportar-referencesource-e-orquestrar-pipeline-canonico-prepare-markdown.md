# Especificação Técnica — Suportar ReferenceSource e Orquestrar Pipeline Canônico prepare_markdown (Ticket #89)

## 1. Contexto e Motivação
O compilador Joaju atua como a fonte única da verdade para processamento e tipografia de documentos científicos e notas.
Anteriormente, a resolução de referências bibliográficas dependia exclusivamente de arquivos físicos no sistema de arquivos local (`refs/`).
Isso forçou ecossistemas clientes (como o Mandu) a implementar pipelines manuais e paralelos para resolver referências mantidas em bancos de dados (ex.: SQLite em memória) ou ambientes sem filesystem convencional (ex.: WebAssembly, Mobile).
Este ticket introduz a enumeração `ReferenceSource` e a função pública canônica `prepare_markdown`, unificando a esteira de pré-processamento textual do Joaju.

## 2. Decisões de Arquitetura e Árvore de Escolhas

### 2.1 Modelo `ReferenceSource`
Para suportar fontes variadas de referências sem explosão combinatória de funções:
```rust
pub enum ReferenceSource<'a> {
    FileSystem(Option<&'a Path>),
    Memory(&'a HashMap<String, joaju_refs::ReferenceNote>),
    Hybrid {
        memory: &'a HashMap<String, joaju_refs::ReferenceNote>,
        base_dir: Option<&'a Path>,
    },
}
```
Implementa-se `From<&'a HashMap<String, joaju_refs::ReferenceNote>>` e `From<&'a Path>` para ergonomia.

### 2.2 Pipeline Canônico `prepare_markdown`
A função pública `joaju::prepare_markdown(markdown: &str, references: Option<ReferenceSource>) -> String` orquestra:
1. Resolução de citações e anexação bibliográfica (via `joaju-refs`):
   - Se `Memory(map)`: resolve diretamente contra o mapa em memória.
   - Se `FileSystem(dir)`: resolve contra o diretório via `resolve_and_process_references`.
   - Se `Hybrid { memory, base_dir }`: prioriza o mapa em memória e busca chaves não resolvidas no sistema de arquivos.
2. Pré-processadores do `joaju-markdown`:
   - Remoção de comentários `%%...%%` (`preprocess_comments`).
   - Conversão de callouts `> [!type]` (`preprocess_callouts`).
   - Conversão de realces `==...==` (`preprocess_highlights`).
   - Normalização de wikilinks `[[...]]` (`preprocess_wikilinks`).

### 2.3 Integração com Compilação HTML e Typst/PDF
- `compile_markdown_to_html_with_options` e a implementação interna de `compile_markdown_to_html_with_base_dir` passam a delegar a fase de pré-processamento de Markdown diretamente para `prepare_markdown`.
- Adiciona-se `compile_markdown_to_html_with_source(markdown: &str, source: Option<ReferenceSource>) -> Result<String, String>`.

## 3. Preservação de Manutenibilidade (Para Natasha)
- **Por que `ReferenceSource` e não várias funções separadas?** Para evitar duplicar 9 a 12 variações de compiladores (`compile_html_with_refs`, `compile_pdf_with_refs`, etc.) e permitir que novos backends de dados (ex.: SQLite embutido, Zotero API) sejam adicionados apenas como novas variantes do enum.
- **Por que unificar o pré-processamento em `prepare_markdown`?** Garante que o preview em ferramentas clientes (TUI, GUI, Web) e a compilação final (HTML, Typst/PDF) usem exatamente o mesmo algoritmo e ordem de execução de transformações sintáticas.
- **Alternativas descartadas:**
  - *Manter a orquestração no `mandu-core`*: Descartada por inverter a hierarquia arquitetural e desalinhar o preview da compilação real do Joaju.
  - *Clonar referências para passar por valor*: Descartada por gerar alocações desnecessárias em documentos volumosos com centenas de citações.
