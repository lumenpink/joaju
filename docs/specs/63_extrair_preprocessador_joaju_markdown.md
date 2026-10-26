# Especificação 63: Extração do Pré-processador e Conversor de Markdown para joaju-markdown

- **ID do Ticket:** #63
- **Status:** Concluído (DD-TDD Fase 9)
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

Esta especificação concluiu a extração completa dessa lógica para a crate desacoplada `joaju-markdown`, reexportando-a no `joaju` mantendo compatibilidade absoluta com a API pública.

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
   - Adicionada `joaju-markdown = { version = "0.1.0", registry = "gambi" }` em `Cargo.toml`.
   - Simplificado `src/preprocessor/mod.rs` para reexportar a partir de `joaju_markdown`.
   - Simplificado `convert_markdown_to_typst` e `normalize_typst_math_expressions` em `src/compiler/mod.rs` delegando à `joaju_markdown`.
   - Removidos arquivos redundantes `src/preprocessor/image.rs` e `src/preprocessor/gist.rs`.

3. **Garantia de Não-Regressão:**
   - Teste Red `tests/test_joaju_markdown_dependency.sh` garantindo a presença de `joaju-markdown` e ausência de código duplicado.
   - Suíte completa de testes aprovada: `preprocessor_spec`, `exhaustive_spec`, `cli_spec`, `compiler_spec`.

## 3. Critérios de Aceitação
1. `joaju-markdown` listado nas dependências soberanas de `Cargo.toml`.
2. Todos os testes de pré-processamento e compilação passam com 100% de sucesso.
3. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem qualquer advertência.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #63
- [x] Fase 2 (Branch): `feat/63-extrair-preprocessador-joaju-markdown`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Criado teste red `tests/test_joaju_markdown_dependency.sh`
- [x] Fase 5 (Green Implement): Publicado joaju-markdown e integrado com sucesso
- [x] Fase 6 (Refactor): Removidos image.rs e gist.rs, unificadas reexportações
- [x] Fase 7 (Mutation Test): Testes de mutantes e suíte de regressão aprovados
- [x] Fase 8 (Quality Gate): fmt e clippy sem warnings, 100% dos testes verdes
- [x] Fase 9 (Finish & Close): Concluído e pronto para merge (closes #63)
