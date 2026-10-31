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
exit 0
