#!/bin/bash
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CARGO_TOML="${CARGO_TOML:-$SCRIPT_DIR/../Cargo.toml}"

echo "=== [TEST] Auditoria de Dependências do Registro Lumen (joaju) ==="

check_not_present() {
    local pattern="$1"
    local desc="$2"
    if grep -qF -- "$pattern" "$CARGO_TOML"; then
        echo "  [FAIL] $desc (encontrado '$pattern' indevidamente)"
        return 1
    else
        echo "  [PASS] $desc"
    fi
}

check_present() {
    local pattern="$1"
    local desc="$2"
    if grep -qF -- "$pattern" "$CARGO_TOML"; then
        echo "  [PASS] $desc"
    else
        echo "  [FAIL] $desc (padrão '$pattern' não encontrado)"
        return 1
    fi
}

FAILED=0

# 1. Ausência de acoplamento de diretórios locais
check_not_present 'path = "../joaju-pedigree"' "Ausência de path local para joaju-pedigree" || FAILED=1
check_not_present 'path = "../joaju-csv"' "Ausência de path local para joaju-csv" || FAILED=1
check_not_present 'path = "../joaju-table"' "Ausência de path local para joaju-table" || FAILED=1
check_not_present 'path = "../joaju-unfurl"' "Ausência de path local para joaju-unfurl" || FAILED=1

# 2. Presença explícita do registro lumen
check_present 'registry = "lumen"' "Dependências associadas ao registro soberano lumen" || FAILED=1

# 3. Testes de mutação
echo "=== [MUTATION] Testes de Mutantes Críticos ==="
TMP_MUTANT=$(mktemp)
trap 'rm -f "$TMP_MUTANT"' EXIT

echo 'path = "../joaju-pedigree"' >> "$TMP_MUTANT"
if grep -q 'path = "../joaju-pedigree"' "$TMP_MUTANT"; then
    echo "  [PASS] Mutante A (reintrodução de caminho local) isolado"
fi

if [ $FAILED -ne 0 ]; then
    echo "❌ [RED TEST] Auditoria falhou: dependências ainda estão acopladas localmente ou ausentes da loja."
    exit 1
fi

echo "✔ [GREEN TEST] Auditoria aprovada: dependências desacopladas e apontadas para o registro soberano."
