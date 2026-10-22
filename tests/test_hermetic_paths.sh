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

# 3. Teste de mutação: garantir que reintroduzir expect() incondicional falhe na validação
validate_compiler_spec() {
    local file="$1"
    if grep -q 'read_to_string(path)\.expect("arquivo comparativo deve existir")' "$file"; then
        return 1
    fi
    return 0
}

TMP_MUTANT=$(mktemp)
trap 'rm -f "$TMP_MUTANT"' EXIT

cat << 'EOF' > "$TMP_MUTANT"
let md = std::fs::read_to_string(path).expect("arquivo comparativo deve existir");
EOF

if validate_compiler_spec "$TMP_MUTANT" >/dev/null 2>&1; then
    echo "FALHA DE MUTAÇÃO: Mutante com expect incondicional não foi detectado!"
    exit 1
fi

echo "SUCESSO: Verificação de caminhos herméticos concluída."
