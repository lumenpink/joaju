#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== [TEST] Validando ausência de acoplamento a caminhos externos em testes ==="

# 1. Verificar se audit_registry_dependencies.sh funciona em diretório arbitrário
CARGO_TOML="$ROOT_DIR/Cargo.toml" bash "$ROOT_DIR/tests/audit_registry_dependencies.sh" >/dev/null

# 2. Verificar se compiler_spec.rs trata ausência do arquivo comparativo sem pânico
if grep -q 'read_to_string(path)\.expect("arquivo comparativo deve existir")' "$ROOT_DIR/tests/compiler_spec.rs"; then
    echo "FALHA: compiler_spec.rs ainda possui expect() incondicional em path de host!"
    exit 1
fi

echo "SUCESSO: Verificação de caminhos herméticos concluída."
