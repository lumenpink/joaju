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
