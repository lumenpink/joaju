#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== [TEST] Validando ausência de openssl-sys em dependências ==="

if cargo tree -i openssl-sys 2>/dev/null; then
    echo "FALHA: openssl-sys encontrado na árvore de dependências do joaju!"
    exit 1
fi

echo "SUCESSO: Nenhuma dependência de openssl-sys detectada."
