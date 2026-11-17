# Especificação Técnica — Integração do joaju-refs e Resolução de Citações em ./refs (Ticket #88)

## 1. Contexto e Motivação
A geração de artigos e documentos acadêmicos e técnicos no Joaju necessita de resolução de citações bibliográficas (estilos ABNT autor-data e numérico) a partir de notas individuais em Markdown geradas pelo Tatupeba/Zotero.
A crate `joaju-refs` centraliza a lógica de formatação de citações e referências. O compilador do Joaju precisa integrar essa funcionalidade no pipeline de compilação, resolvendo as citações contra notas armazenadas localmente em `./refs` (e subpastas).

## 2. Decisões de Arquitetura e Árvore de Escolhas

### 2.1 Ordem de Precedência do Diretório de Referências
1. `refs_dir` explicitado no YAML frontmatter do documento (ex.: `refs_dir: "./bibliografia"` ou `refs_dir: "../refs"`).
2. Padrão canônico: `./refs` relativo ao documento sendo compilado (ou relativo ao diretório corrente de trabalho na ausência de documento físico).

### 2.2 Estratégia de Indexação e Resolução de Arquivos
- **Indexador de Nomes em Memória:**
  Na presença de citações (`@citekey`, `[@citekey]`, `[[citekey]]`, etc.), o resolvedor inspeciona o diretório `refs_dir` de forma recursiva, indexando apenas os nomes de arquivo (sem extensão `.md`) mapeados para seus respectivos caminhos em disco (`HashMap<String, PathBuf>`).
  Isso suporta subpastas livres (ex.: `refs/artigos/silva2024.md`, `refs/livros/kollontai1920.md`) sem custo perceptível de I/O.
- **Lookup Direto O(1):**
  Para cada citação encontrada, o compilador consulta o mapa de nomes. Encontrando o arquivo, realiza a leitura e parse do YAML frontmatter via `joaju_refs::parse_reference_note` e armazena em cache de notas (`HashMap<String, ReferenceNote>`).
- **Fallback Tolerante (Varredura de Resgate):**
  Se uma citação não corresponder a nenhum nome de arquivo no mapa, o compilador varre os arquivos `.md` restantes do diretório em busca do campo `citekey:` no YAML frontmatter.

### 2.3 Integração no Pipeline do Compilador HTML
No módulo `src/compiler/html.rs`, antes da chamada ao `pulldown-cmark`:
1. O texto Markdown com frontmatter é inspecionado para determinar `refs_dir` e estilo (`estilo: autor-data` ou `estilo: numerico`).
2. Se houver citações presentes no corpo, o resolvedor localiza e parseia as notas referenciadas.
3. Executa `joaju_refs::process_references(markdown, &references, style)`.
4. O resultado com citações in-text formatadas e a seção `# Referências` anexada segue para os pré-processadores do `joaju-markdown` e o `cmark`.

## 3. Preservação de Manutenibilidade (Para Natasha)
- **Por que `./refs` e não caminhos globais do usuário?** Evita acoplamento implícito com `$HOME` ou máquinas específicas, permitindo compilação reproduzível em CI ou entre colegas de equipe.
- **Por que mapa de nomes em vez de ler todos os arquivos?** Abrir centenas de notas para ler YAML consome I/O e CPU desnecessariamente; indexar apenas nomes de arquivos mantém o pipeline quase instantâneo.
- **Alternativas descartadas:**
  - *BibTeX legada (`referencias.bib`)*: Descartada pela preferência de Option B (notas Markdown do Zotero/Tatupeba).
  - *Hardcoded `~/obsidian/03_referencia`*: Descartada por quebrar portabilidade e gerar dívida técnica invisível.
