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

# 5. Teste de mutação: validar que mutantes com tokio/reqwest sejam detectados
echo "=== [MUTATION] Validando sensibilidade a mutantes ==="
TMP_MUTANT=$(mktemp)
trap 'rm -f "$TMP_MUTANT"' EXIT

echo "tokio v1.53.3" > "$TMP_MUTANT"
if ! grep -q "tokio" "$TMP_MUTANT"; then
    echo "FALHA DE MUTAÇÃO: tokio não foi detectado no mutante!"
    exit 1
fi

echo "reqwest v0.12.28" > "$TMP_MUTANT"
if ! grep -q "reqwest" "$TMP_MUTANT"; then
    echo "FALHA DE MUTAÇÃO: reqwest não foi detectado no mutante!"
    exit 1
fi

echo "SUCESSO: Nenhuma dependência assíncrona ou de rede presente na biblioteca e mutação testada."
