# Especificação Técnica 48: Hermeticidade e Independência de Host em Testes

## Objetivo
Garantir que a suíte de testes de `joaju` e scripts auxiliares executem de forma 100% hermética em qualquer ambiente (local, container CI no runner, etc.), sem dependência de arquivos inexistentes no host (como diretórios Obsidian locais).

## Requisitos
1. Em `tests/compiler_spec.rs`, o teste `test_spec_compile_comparativo_pedtools_vs_kinship2_document` não deve falhar com `panic` quando o caminho absoluto externo `/home/lumen/obsidian/...` não existir. Caso ausente, deve registrar log informativo e finalizar com sucesso (skip graceful).
2. Em `tests/audit_registry_dependencies.sh`, a variável `CARGO_TOML` deve resolver dinamicamente relativa ao repositório (`"${CARGO_TOML:-$SCRIPT_DIR/../Cargo.toml}"`) em vez de caminho estático `/home/lumen/...`.
3. Todos os testes devem rodar com sucesso tanto no ambiente de desenvolvimento local quanto no container hermético do runner.
