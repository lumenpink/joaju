# Especificação 63: Extração do Pré-processador e Conversor de Markdown para joaju-markdown

- **ID do Ticket:** #63
- **Status:** Em Progresso (DD-TDD)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Motivação
O compilador acadêmico Joaju (`joaju`) realiza operações intensivas de pré-processamento de texto sobre documentos Markdown:
- Resolução de wikilinks de imagens com dimensões (`![[figura.png|450]]`) e imagens Markdown para sintaxe Typst `#image(...)`.
- Normalização de blocos ````gist``.
- Normalização de expressões matemáticas justapostas (`$ii$` -> `$i i$`).
- Conversão de cabeçalhos (`#` -> `=`) e quebras de página (`<!-- pagebreak -->` -> `#pagebreak()`).
- Pré-processamento e renderização de tabelas ABNT/IBGE.

Projetos clientes como `lauda`, `papeli` e `mandu` necessitam frequentemente dessas transformações de Markdown sem precisar do compilador pesado do Typst (`typst-as-lib`, `typst`, `typst-pdf`).

Esta especificação define a extração completa dessa lógica para a crate desacoplada `joaju-markdown`, reexportando-a no `joaju` para manter compatibilidade absoluta com a API pública.

## 2. Requisitos Técnicos
1. **Crate Soberana `joaju-markdown`:**
   - Publicada no registro `gambi` (`v0.1.0`).
   - Exporta:
     - `preprocess_images`
     - `preprocess_gists`
     - `normalize_typst_math_expressions`
     - `convert_markdown_to_typst`
     - `preprocess_markdown`
     - `markdown_to_typst`
     - `table` / `joaju_table` reexports

2. **Integração no `joaju`:**
   - Adicionar `joaju-markdown = { version = "0.1.0", registry = "gambi" }` em `Cargo.toml`.
   - Simplificar [src/preprocessor/](file:///home/lumen/src/joaju/src/preprocessor) para reexportar a partir de `joaju_markdown`.
   - Simplificar `convert_markdown_to_typst` e `normalize_typst_math_expressions` em [src/compiler/mod.rs](file:///home/lumen/src/joaju/src/compiler/mod.rs) para delegar diretamente à `joaju_markdown`.
   - Remover arquivos redundantes [src/preprocessor/image.rs](file:///home/lumen/src/joaju/src/preprocessor/image.rs) e [src/preprocessor/gist.rs](file:///home/lumen/src/joaju/src/preprocessor/gist.rs).

3. **Garantia de Não-Regressão:**
   - Teste Red para verificar presença de `joaju-markdown` e ausência de código duplicado de regex de imagens/gists.
   - Suíte de testes existente: `tests/preprocessor_spec.rs`, `tests/compiler_spec.rs`, `tests/cli_spec.rs`.

## 3. Critérios de Aceitação
1. `joaju-markdown` listado nas dependências soberanas de `Cargo.toml`.
2. Todos os testes de pré-processamento e compilação passam com 100% de sucesso.
3. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem qualquer advertência.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #63
- [x] Fase 2 (Branch): `feat/63-extrair-preprocessador-joaju-markdown`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [ ] Fase 4 (Red Test): Criar teste red para dependência e integridade do joaju-markdown
- [ ] Fase 5 (Green Implement): Integrar joaju-markdown e limpar preprocessor interno
- [ ] Fase 6 (Refactor): Reexportar de forma idiomática
- [ ] Fase 7 (Mutation Test): Testar mutantes no pipeline de pré-processamento
- [ ] Fase 8 (Quality Gate): fmt, clippy sem warnings e testes verdes
- [ ] Fase 9 (Finish & Close): Concluído e pronto para merge
