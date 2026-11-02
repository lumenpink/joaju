# Spec 76: Fixar Versões Estritas de Todos os Submódulos joaju-*

## 1. Contexto e Motivação
Para assegurar compilações herméticas, reprodutíveis e determinísticas, o motor principal `joaju` não deve depender de aliases mutáveis (como `latest`) em nenhuma etapa de compilação ou linkedição.

No `build.rs`, a constante `JOAJU_PDF_REQUIRED_VERSION` deve substituir o fallback `latest`, e no `Cargo.toml`, as versões das dependências do ecossistema devem ser estritas e rastreáveis.

## 2. Requisitos Técnicos
- Eliminar referências a `"latest"` no `build.rs`.
- Declarar a versão canônica exigida `JOAJU_PDF_REQUIRED_VERSION` no `build.rs`.
- Auditar e cravar as dependências de submódulos `joaju-*` no `Cargo.toml`.
- Criar teste automatizado `tests/deps_semver_spec.rs` que audite `Cargo.toml` e `build.rs` contra o uso de `latest` ou dependências não versionadas.

## 3. Plano de Testes (DD-TDD)
- Fase 4: Criar `tests/deps_semver_spec.rs` comprovando falha devido à presença de `"latest"` no `build.rs`.
- Fase 5: Implementação funcional removendo `"latest"` e cravando constantes estritas.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict`.

## 4. Status de Conclusão
- [x] Tag mutável `"latest"` removida de `build.rs`.
- [x] Constante `JOAJU_PDF_REQUIRED_VERSION` cravada em `build.rs`.
- [x] Dependências auditadas em `Cargo.toml`.
- [x] Suíte `tests/deps_semver_spec.rs` 100% verde.
- [x] 100% de conformidade Gosplan (`cccp check --strict`).
