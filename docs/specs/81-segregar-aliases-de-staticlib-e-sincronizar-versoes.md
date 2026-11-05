# Especificação: Segregação de Aliases de Staticlib e Sincronização Canônica de Versões

## 1. Contexto e Motivação
A esteira CI do motor central `joaju` (Run 1795) compilou com sucesso os testes e binários multiarch e enviou a staticlib `libjoaju-x86_64-linux.a` para a versão exata `0.2.0-dev.137+20261104.f4439ec`. Entretanto, falhou na etapa de publicação do alias base por tentar realizar um `PUT` direto sem `DELETE` prévio sob um alias preexistente, incorrendo em `HTTP 409 Conflict`.

Adicionalmente, os módulos satélites `joaju-table` (`0.2.0-dev.56`), `joaju-lirica` (`0.5.1-dev.31`) e `joaju-pdf` (`0.1.0-dev.47`) tiveram novas versões canônicas publicadas no registro Gambi, exigindo atualização de amarra exata (`=`) no motor central e na staticlib requerida pelo `build.rs`.

## 2. Requisitos Técnicos

1. **Workflow CI (`.forgejo/workflows/ci.yml`)**:
   - Detectar o canal de lançamento (`nightly` para branch `main`, `stable` para tags `v*`).
   - Definir `PRIMARY_ALIAS="latest-dev"` e `NAMED_ALIAS="${BASE_VERSION}-dev"` para builds `nightly`.
   - Definir `PRIMARY_ALIAS="latest"` e `NAMED_ALIAS="${BASE_VERSION}"` para tags `v*`.
   - Implementar deleção prévia idempotente (`curl -s -X DELETE`) antes de cada envio PUT para os aliases.
2. **Dependências em `Cargo.toml`**:
   - Atualizar `joaju-table = { version = "=0.2.0-dev.56", registry = "gambi" }`.
   - Atualizar `joaju-lirica = { version = "=0.5.1-dev.31", registry = "gambi" }`.
   - Manter as demais 8 dependências cravadas com `=`.
3. **Resolução de Staticlib em `build.rs`**:
   - Atualizar `const JOAJU_PDF_REQUIRED_VERSION: &str = "0.1.0-dev.47";`.
   - Manter cadeia de fallback `latest-dev` -> `0.1.0-dev` -> `0.1.0`.
4. **Testes Automatizados**:
   - Atualizar `tests/deps_exact_spec.rs` para auditar as novas versões exatas.
   - Criar `tests/ci_aliases_spec.rs` para auditar a segregação de aliases e o DELETE prévio no CI do motor central.

## 3. Plano de Testes (DD-TDD)
- Fase 4 (Red Test): Criar `tests/ci_aliases_spec.rs` e expandir `tests/deps_exact_spec.rs` evidenciando as falhas da configuração anterior.
- Fase 5 (Green Implement): Atualizar `Cargo.toml`, `build.rs` e `.forgejo/workflows/ci.yml`, garantindo a passagem dos testes.
- Fase 6 (Refactor): Formatação de código e testes com `cargo fmt` e validação com `cargo clippy`.
- Fase 7 (Mutation Test): Validação de sensibilidade a mutações nos aliases do CI e versões de dependências.
- Fase 8 (Quality Gate): Auditoria GOST com `cccp check --strict` e verificação de testes unitários.
- Fase 9 (Finish & Close): Integração à branch `main` e disparo da esteira automatizada.

## 4. Registro de Execução e Fechamento
- Fase 3 (Spec): Especificação elaborada e versionada.
- Fase 4 (Red Test): Testes `tests/ci_aliases_spec.rs` e `tests/deps_exact_spec.rs` criados e falhas comprovadas.
- Fase 5 (Green Implement): Implementada segregação de aliases em `.forgejo/workflows/ci.yml`, dependências atualizadas no `Cargo.toml` e staticlib atualizada no `build.rs`.
- Fase 6 (Refactor): Código e testes formatados via `cargo fmt` e validados com `cargo clippy --all-targets -- -D warnings`.
- Fase 7 (Mutation Test): Sensibilidade do teste confirmada contra ausência de DELETE prévio ou alteração nos nomes canônicos de aliases.
- Fase 8 (Quality Gate): Conformidade estrita validada com `cargo fmt --check`, clippy e `cccp check --strict`.
- Fase 9 (Finish & Close): Ticket concluído com sucesso.

