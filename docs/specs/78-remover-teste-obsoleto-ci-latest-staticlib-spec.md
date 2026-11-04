# Spec 78: Remover Teste Obsoleto ci_latest_staticlib_spec

## 1. Contexto e Motivação
Os tickets #76 e #77 consolidaram a política canônica de compilações herméticas e rastreáveis no ecossistema `joaju`, expurgando o uso de tags mutáveis (como `latest`) no `build.rs` e nos workflows de publicação do CI.

Entretanto, o arquivo de teste `tests/ci_latest_staticlib_spec.rs`, originado do antigo ticket #73, permaneceu na árvore de testes contendo asserções contraditórias que exigiam o retorno de `latest` no `build.rs` e na publicação de artefatos.

## 2. Requisitos Técnicos
- Remover o arquivo obsoleto `tests/ci_latest_staticlib_spec.rs`.
- Garantir que a suíte canônica de testes seja regida exclusivamente por `tests/deps_semver_spec.rs` e `tests/ci_workflow_spec.rs`.
- Executar `cargo test` confirmando que 100% dos testes passam.

## 3. Plano de Testes (DD-TDD)
- Fase 3: Especificação técnica formal.
- Fase 4: Validar falha de `tests/ci_latest_staticlib_spec.rs` frente ao código canônico atual.
- Fase 5: Expurgar `tests/ci_latest_staticlib_spec.rs`.
- Fase 8: Suíte de testes 100% verde e auditoria `cccp check --strict`.


## 4. Status de Fechamento
- Teste legado `tests/ci_latest_staticlib_spec.rs` removido com sucesso.
- Suíte integral aprovada (68 testes verdes, 0 falhas).
- Gosplan e Clippy 100% conformes.
