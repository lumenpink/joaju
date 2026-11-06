# Especificação: Embutir Build Metadata na Versão e Sincronizar Catálogo da Banca no CI

## 1. Contexto e Motivação
Ao executar `joaju --version`, a versão apresentada omitia os metadados de build (`+timestamp.hash`), exibindo apenas a versão truncada da crate (ex.: `joaju 0.2.0-dev.137`), divergindo da convenção de rastreabilidade do ecossistema. Além disso, a esteira de CI (`.forgejo/workflows/ci.yml`) gerava o catálogo unificado da Banca (`site/downloads/catalog.json`) via `rotate-downloads.py`, mas omitia o arquivo do `git add`, fazendo com que os downloads no portal da Banca ficassem estagnados em releases antigas (01/11).

## 2. Requisitos Técnicos
1. **Injeção de Versão via Build Script (`build.rs`)**:
   - Detectar `JOAJU_FULL_VERSION` via variável de ambiente ou lendo o arquivo `VERSION` na raiz do repositório.
   - Emitir `cargo:rustc-env=JOAJU_FULL_VERSION={full_version}`.
   - Emitir diretivas de reexecução `cargo:rerun-if-changed=VERSION` e `cargo:rerun-if-env-changed=JOAJU_FULL_VERSION`.
2. **Exposição Canônica em `src/lib.rs` e `src/ffi.rs`**:
   - `pub const VERSION: &str = env!("JOAJU_FULL_VERSION");`.
   - `C_VERSION_BYTES: &[u8] = concat!(env!("JOAJU_FULL_VERSION"), "\0").as_bytes();`.
3. **Automação no CI (`.forgejo/workflows/ci.yml`)**:
   - Injetar `JOAJU_FULL_VERSION: ${{ steps.meta.outputs.version }}` e sincronizar o `Cargo.toml` antes da compilação multiarquitetura com `cargo zigbuild`.
   - Adicionar `site/downloads/catalog.json` ao `git add` no passo de commit no repositório da Banca.
4. **Testes de Especificação**:
   - `tests/abi_version_spec.rs`: auditar que `joaju::VERSION` e `joaju_version()` coincidem com o arquivo `VERSION` e respeitam o formato SemVer com metadados.
   - `tests/ci_workflow_spec.rs`: auditar presença de injeção de versão e inclusão de `catalog.json` no commit.
   - `tests/cli_spec.rs`: auditar que `joaju --version` emite a versão completa com build metadata.

## 3. Registro de Execução e Fechamento
- Fase 3 (Spec): Especificação elaborada e versionada.
- Fase 4 (Red Test): Testes automatizados falhos comprovados em `abi_version_spec`, `ci_workflow_spec` e `cli_spec`.
- Fase 5 (Green Implement): Implementada injeção no `build.rs`, exportada em `src/lib.rs` e `src/ffi.rs`, e corrigida automação no `ci.yml`.
- Fase 6 (Refactor): Código formatado via `cargo fmt` e verificado com `cargo clippy --all-targets -- -D warnings`.
- Fase 7 (Mutation Test): Sensibilidade validada contra descompasso de SemVer ou omissão de build metadata em nightlies.
- Fase 8 (Quality Gate): Testes direcionados aprovados, clippy limpo e formatação em conformidade.
- Fase 9 (Finish & Close): Ciclo concluído com sucesso e integrado via `cccp merge`.
