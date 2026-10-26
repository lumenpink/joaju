#!/usr/bin/env bash
# Red test: Garante que joaju integra joaju-templates e desacoplou templates estaticos
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== [TEST] Auditoria de Dependencias de Templates no Cargo.toml ==="

if ! grep -E "joaju-templates\b" "$ROOT_DIR/Cargo.toml" >/dev/null 2>&1; then
    echo "FALHA: 'joaju-templates' deve estar declarado no Cargo.toml do joaju!" >&2
    exit 1
fi

if ! grep -E "joaju_templates" "$ROOT_DIR/src/compiler/mod.rs" >/dev/null 2>&1; then
    echo "FALHA: 'src/compiler/mod.rs' deve referenciar joaju_templates!" >&2
    exit 1
fi

echo "SUCESSO: joaju-templates integrado e templates desacoplados."
exit 0
