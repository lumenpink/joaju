#!/usr/bin/env bash
# Red test: Garante que joaju nao possui joaju-csv como dependencia direta
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== [TEST] Auditoria de Dependencias CSV no Cargo.toml ==="

if grep -E "^joaju-csv\b" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'joaju-csv' detectado como dependencia direta em Cargo.toml!" >&2
    exit 1
fi

if grep -E "joaju_csv" "$ROOT_DIR/src/engine/pedigree.rs" >/dev/null 2>&1; then
    echo "FALHA: 'joaju_csv' ainda referenciado diretamente em src/engine/pedigree.rs!" >&2
    exit 1
fi

echo "SUCESSO: joaju-csv desacoplado com sucesso e transferido para joaju-pedigree."
exit 0
