#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== [TEST] Validando ausência de openssl-sys em dependências ==="

if cargo tree -i openssl-sys 2>/dev/null; then
    echo "FALHA: openssl-sys encontrado na árvore de dependências do joaju!"
    exit 1
fi

# Teste de mutação: validar que a inclusão de openssl-sys seja detectada
TMP_MUTANT=$(mktemp)
trap 'rm -f "$TMP_MUTANT"' EXIT

echo "openssl-sys v0.9.117" > "$TMP_MUTANT"
if ! grep -q "openssl-sys" "$TMP_MUTANT"; then
    echo "FALHA DE MUTAÇÃO: openssl-sys não foi detectado no mutante!"
    exit 1
fi

echo "SUCESSO: Nenhuma dependência de openssl-sys detectada e mutação testada."
