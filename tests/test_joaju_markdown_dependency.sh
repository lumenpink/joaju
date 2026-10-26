#!/usr/bin/env bash
# Red test: Garante que joaju integra joaju-markdown e desacoplou preprocessor interno
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== [TEST] Auditoria de Dependencias Markdown no Cargo.toml ==="

if ! grep -E "joaju-markdown\b" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'joaju-markdown' deve estar declarado no Cargo.toml do joaju!" >&2
    exit 1
fi

if [ -f "$ROOT_DIR/src/preprocessor/image.rs" ]; then
    echo "FALHA: 'src/preprocessor/image.rs' ainda presente no core (deve ser extraido para joaju-markdown)!" >&2
    exit 1
fi

if [ -f "$ROOT_DIR/src/preprocessor/gist.rs" ]; then
    echo "FALHA: 'src/preprocessor/gist.rs' ainda presente no core (deve ser extraido para joaju-markdown)!" >&2
    exit 1
fi

echo "SUCESSO: joaju-markdown integrado e preprocessor interno limpo."
exit 0
