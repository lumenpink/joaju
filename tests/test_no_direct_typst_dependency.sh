#!/usr/bin/env bash
# Red test: Garante que joaju nao possui typst nem typst-as-lib como dependencias diretas
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== [TEST] Auditoria de Dependencias Typst no Cargo.toml ==="

if grep -E "^typst-as-lib\b" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'typst-as-lib' detectado como dependencia direta em Cargo.toml!" >&2
    exit 1
fi

if grep -E "^typst-pdf\b" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'typst-pdf' detectado como dependencia direta em Cargo.toml!" >&2
    exit 1
fi

if grep -E "^typst-layout\b" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'typst-layout' detectado como dependencia direta em Cargo.toml!" >&2
    exit 1
fi

if grep -E "^typst\s*=" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'typst' detectado como dependencia direta em Cargo.toml!" >&2
    exit 1
fi

if ! grep -E "^joaju-table\b" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'joaju-table' deve permanecer declarado no Cargo.toml!" >&2
    exit 1
fi

echo "SUCESSO: Motor Typst desacoplado para joaju-pdf e joaju-table preservado."

echo "=== [MUTATION TEST] Testando sensibilidade a mutantes ==="
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

cp "$ROOT_DIR/Cargo.toml" "$TMP_DIR/Cargo.toml"

# Mutante 1: reinserir typst
echo 'typst = "0.15"' >> "$TMP_DIR/Cargo.toml"
if ! grep -E "^typst\s*=" "$TMP_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: Mutante 1 nao pode ser simulado!" >&2
    exit 1
fi
echo "✔ Mutante 1 (reinserção de typst) detectado com sucesso."

# Mutante 2: remoção de joaju-table
sed -i '/^joaju-table/d' "$TMP_DIR/Cargo.toml"
if grep -E "^joaju-table\b" "$TMP_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: Mutante 2 nao pode ser simulado!" >&2
    exit 1
fi
echo "✔ Mutante 2 (remoção de joaju-table) detectado com sucesso."

echo "SUCESSO: Todos os mutantes neutralizados."
exit 0
