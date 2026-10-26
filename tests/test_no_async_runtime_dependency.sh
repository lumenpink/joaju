#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== [TEST] Validando ausência de dependências assíncronas e de rede no núcleo do joaju ==="

# 1. Validar que tokio não está na árvore da biblioteca
if cargo tree -p joaju --no-default-features -i tokio 2>/dev/null; then
    echo "FALHA: tokio encontrado na biblioteca joaju!"
    exit 1
fi

# 2. Validar que reqwest não está na árvore da biblioteca
if cargo tree -p joaju --no-default-features -i reqwest 2>/dev/null; then
    echo "FALHA: reqwest encontrado na biblioteca joaju!"
    exit 1
fi

# 3. Validar que scraper não está na árvore da biblioteca
if cargo tree -p joaju --no-default-features -i scraper 2>/dev/null; then
    echo "FALHA: scraper encontrado na biblioteca joaju!"
    exit 1
fi

# 4. Validar que tracing não está na árvore da biblioteca
if cargo tree -p joaju --no-default-features -i tracing 2>/dev/null; then
    echo "FALHA: tracing encontrado na biblioteca joaju!"
    exit 1
fi

echo "SUCESSO: Nenhuma dependência assíncrona ou de rede presente na biblioteca joaju."
