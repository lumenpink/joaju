#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML="$ROOT_DIR/Cargo.toml"

echo "=== [TEST] Validando hermeticidade de Cargo.toml ==="

validate_hermetic() {
    local file="$1"
    if grep -E 'path\s*=\s*"\.\./' "$file"; then
        echo "FALHA: $file contém paths relativos a diretórios externos/irmãos!"
        return 1
    fi
    if grep -E '\[patch\.' "$file"; then
        echo "FALHA: $file contém seção [patch] com dependências não herméticas!"
        return 1
    fi
    return 0
}

validate_hermetic "$CARGO_TOML"
echo "SUCESSO: Cargo.toml é estritamente hermético."
