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

# Teste de mutação: garantir que arquivo mutado com path relativo seja rejeitado
TEMP_MUTANT=$(mktemp)
trap 'rm -f "$TEMP_MUTANT"' EXIT

cat << 'EOF' > "$TEMP_MUTANT"
[package]
name = "mutant"
version = "0.1.0"
[dependencies]
joaju-pedigree = { path = "../joaju-pedigree" }
EOF

if validate_hermetic "$TEMP_MUTANT" >/dev/null 2>&1; then
    echo "FALHA: Teste de mutação permitiu dependência relativa!"
    exit 1
fi

echo "SUCESSO: Cargo.toml e testes de mutação hermética validados."
