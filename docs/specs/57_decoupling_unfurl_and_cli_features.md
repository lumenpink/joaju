# Especificação 57: Desacoplamento de Dependências de Rede, Otimização do CLI e Suporte a staticlib

- **ID do Ticket:** #57
- **Status:** Aprovado
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Motivação
O Joaju (`joaju`) é o compilador acadêmico e orquestrador tipográfico em Rust integrando Typst, tabelas, heredogramas clínicos e fórmulas químicas.

Historicamente, dependências de rede e web scraping (`reqwest`, `scraper`, `base64`) e um runtime assíncrono (`tokio = { features = ["full"] }`) foram introduzidas unicamente para suportar a funcionalidade de `unfurl` no CLI. Além disso, dependências fantasmas não utilizadas (`tracing`, `tracing-subscriber`, `joaju-unfurl` duplicada) permaneceram no `Cargo.toml`.

Como o `joaju` é consumido como biblioteca por projetos como `mandu`, `lauda` e `joaju-web`, a presença dessas dependências infla excessivamente o grafo de compilação, trazendo dezenas de crates transitivos (Hyper, Mio, Socket2, Cssparser, etc.) e prejudicando drasticamente o tempo de build e consumo de memória na esteira de CI.

## 2. Requisitos Técnicos
1. **Expurgo de Dependências Fantasmas:**
   - Remover `tracing` e `tracing-subscriber` de `Cargo.toml` (zero uso no código-fonte).
   - Remover `joaju-unfurl` duplicada de `[dependencies]` (a crate autônoma `joaju-unfurl` é gerenciada independentemente).

2. **Desacoplamento de Rede e Runtime Assíncrono:**
   - Remover `tokio`, `reqwest`, `scraper` e `base64` da biblioteca `joaju`.
   - Remover o módulo `src/unfurl/` do crate principal `joaju`, delegando a responsabilidade de metadados web e unfurl ao pacote `joaju-unfurl` e ao daemon `joaju-web`.

3. **Isolamento de Dependências de CLI:**
   - Tornar `clap` e `serde_json` dependências opcionais sob a feature `cli`:
     ```toml
     [dependencies]
     clap = { version = "4.5", features = ["derive"], optional = true }
     serde_json = { version = "1", optional = true }

     [features]
     default = ["cli"]
     cli = ["dep:clap", "dep:serde_json"]
     ```
   - Configurar `required-features = ["cli"]` para a seção `[[bin]] name = "joaju"`.
   - Tornar a função `main()` do `src/main.rs` síncrona pura (`fn main() -> Result<...>`), eliminando a anotação `#[tokio::main]`.

4. **Habilitação de C-ABI Staticlib:**
   - Adicionar `crate-type = ["rlib", "staticlib"]` na seção `[lib]`.

## 3. Critérios de Aceitação
1. `cargo tree -p joaju` não contém referências a `tokio`, `reqwest`, `scraper` nem `tracing`.
2. A biblioteca compila com sucesso com `--no-default-features`.
3. O executável de terminal compila e executa normalmente com `cargo run --bin joaju -- --help`.
4. Todos os testes unitários e de integração existentes passam com sucesso.
5. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem qualquer violação.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #57
- [x] Fase 2 (Branch): `feat/57-decoupling-unfurl-and-cli-features`
- [x] Fase 3 (Spec): Especificação técnica aprovada
- [x] Fase 4 (Red Test): `tests/test_no_async_runtime_dependency.sh` falhando com tokio/reqwest
- [x] Fase 5 (Green Implement): Expurgo de tokio, reqwest, scraper, tracing e isolamento do CLI
- [x] Fase 6 (Refactor): Remoção de código morto e alinhamento de módulos
- [x] Fase 7 (Mutation Test): Testes de mutantes integrados
- [x] Fase 8 (Quality Gate): fmt, clippy sem warnings e testes passando
- [x] Fase 9 (Finish & Close): Concluído e pronto para merge (closes #57)
