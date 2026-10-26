#!/usr/bin/env bash
# Red test: Garante que joaju nao possui chematic-smiles ou chematic-depict como dependencias diretas
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== [TEST] Auditoria de Dependencias Quimicas no Cargo.toml ==="

if grep -E "^chematic-smiles\b" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'chematic-smiles' detectado como dependencia direta em Cargo.toml!" >&2
    exit 1
fi

if grep -E "^chematic-depict\b" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'chematic-depict' detectado como dependencia direta em Cargo.toml!" >&2
    exit 1
fi

if ! grep -E "joaju-chem" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'joaju-chem' deve estar presente no Cargo.toml do joaju!" >&2
    exit 1
fi

echo "SUCESSO: chematic-* desacoplado com sucesso para a crate joaju-chem."
exit 0
