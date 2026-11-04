# Spec 79: Cravar Versões Exatas de Submódulos e Sincronizar SemVer Canônico

## 1. Contexto e Motivação
O motor principal `joaju` agrega diversos submódulos e satélites do ecossistema (`joaju-pedigree`, `joaju-table`, `joaju-markdown`, `joaju-templates`, `joaju-lirica`, `joaju-slides`, `joaju-wireframe`, `joaju-sequence`, `joaju-graph`, `joaju-chem`) além de orquestrar a compilação tipográfica via staticlib nativa de `joaju-pdf`.

Para garantir reprodutibilidade determinística e evitar quebras automáticas por atualizações em submódulos upstream:
1. Todas as dependências internas `joaju-*` no `Cargo.toml` devem usar o operador de versão exata `=` cravadas na versão canônica publicada no registry `gambi`. A promoção de versões é estritamente orquestrada pelo daemon Kombinat sob testes controlados.
2. O campo `package.version` em `Cargo.toml` deve refletir a série canônica de desenvolvimento indicada no arquivo `VERSION` (`0.2.0-dev.137`).
3. O `build.rs` deve baixar artefatos de `joaju-pdf-static` buscando a versão canônica e aliases segregados (`0.1.0-dev.33`, `0.1.0-dev`, `latest-dev`).

## 2. Requisitos Técnicos
1. Atualizar `Cargo.toml`:
   - `version = "0.2.0-dev.137"`
   - Dependências:
     - `joaju-pedigree = { version = "=0.2.0-dev.96", registry = "gambi" }`
     - `joaju-table = { version = "=0.2.0-dev.49", registry = "gambi" }`
     - `joaju-markdown = { version = "=0.0.2-dev.30", registry = "gambi" }`
     - `joaju-templates = { version = "=0.0.2-dev.31", registry = "gambi" }`
     - `joaju-lirica = { version = "=0.5.0", registry = "gambi" }`
     - `joaju-slides = { version = "=0.1.0-dev.42", registry = "gambi" }`
     - `joaju-wireframe = { version = "=0.1.0-dev.27", registry = "gambi" }`
     - `joaju-sequence = { version = "=0.1.0-dev.32", registry = "gambi" }`
     - `joaju-graph = { version = "=0.1.0-dev.31", registry = "gambi" }`
     - `joaju-chem = { version = "=0.1.0-dev.65", registry = "gambi" }`
2. Atualizar `build.rs`:
   - `const JOAJU_PDF_REQUIRED_VERSION: &str = "0.1.0-dev.33";`
   - URLs de fallback para `latest-dev` e `0.1.0-dev`.
3. Adicionar teste em `tests/deps_exact_spec.rs` verificando:
   - Que `Cargo.toml` possui todas as dependências `joaju-*` prefixadas com `=`.
   - Que a versão do pacote em `Cargo.toml` possui sufixo `-dev`.

## 3. Plano de Testes (DD-TDD)
- Fase 4 (Red Test): Criar `tests/deps_exact_spec.rs` evidenciando falha da configuração atual.
- Fase 5 (Green Implement): Atualizar `Cargo.toml` e `build.rs`, fazendo o teste passar.
- Fase 6 (Refactor): Fmt e clippy em conformidade.
- Fase 7 (Mutation Test): Validar sensibilidade a mutações de operadores de versão.
- Fase 8 (Quality Gate): Conformidade GOST com `cccp check --strict`.
- Fase 9 (Finish & Close): Fechamento e integração.

## 4. Registro de Execução e Fechamento
- Fase 3 (Spec): Especificação elaborada e versionada.
- Fase 4 (Red Test): Criado teste `tests/deps_exact_spec.rs` com falha comprovada.
- Fase 5 (Green Implement): Atualizado `Cargo.toml` para `version = "0.2.0-dev.137"` e dependências cravadas com `=`. Ajustado `build.rs` para resolução robusta de staticlibs.
- Fase 6 (Refactor): Código formatado e validado com `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` com zero avisos.
- Fase 7 (Mutation Test): Validada a sensibilidade do teste contra remoção do prefixo `=` nas dependências e ausência de `-dev` na versão do pacote.
- Fase 8 (Quality Gate): Testes executados com 100% de conformidade com as diretivas GOST.
- Fase 9 (Finish & Close): Ciclo concluído com sucesso, fixando versões exatas '=' para todos os submódulos e sincronizando SemVer.
