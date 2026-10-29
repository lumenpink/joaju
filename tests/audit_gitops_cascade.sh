#!/usr/bin/env bash
# ==============================================================================
# tests/audit_gitops_cascade.sh
# Valida integracao de workflow GitOps e cascata em joaju (Ticket #44)
# ==============================================================================
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if [ -f ".forgejo/workflows/ci.yaml" ]; then
  WORKFLOW=".forgejo/workflows/ci.yaml"
elif [ -f ".forgejo/workflows/ci.yml" ]; then
  WORKFLOW=".forgejo/workflows/ci.yml"
else
  echo "✘ [FALHA] Nenhum workflow ci.yaml ou ci.yml encontrado!"
  exit 1
fi

python3 -c "
try:
    import yaml
    yaml.safe_load(open('$WORKFLOW'))
except ImportError:
    pass
" || {
  echo "✘ [FALHA] $WORKFLOW possui sintaxe YAML invalida!"
  exit 1
}

if ! grep -q "rustup component add" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao adiciona componentes rustfmt/clippy!"
  exit 1
fi

if ! grep -q "workflow_dispatch:" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao declara 'workflow_dispatch:'!"
  exit 1
fi

if ! grep -q "sparse+https://gambi.2lp.in/api/packages/lumen/cargo/" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao configura endpoint sparse do registry lumen!"
  exit 1
fi

if ! grep -q 'token = "Bearer %s"' "$WORKFLOW" && ! grep -q 'token = "Bearer \${{ secrets.LUMEN_REGISTRY_TOKEN }}"' "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao configura autenticacao Bearer para cargo publish!"
  exit 1
fi

if grep -q "Publish Sovereign Crate & Cascade Downstream" "$WORKFLOW"; then
  if ! grep -q "CANONICAL_VERSION" "$WORKFLOW"; then
    echo "✘ [FALHA] $WORKFLOW nao sincroniza a versao canonica!"
    exit 1
  fi

  if ! grep -q "repos/lumenpink/joaju-web/actions/workflows/ci.yaml/dispatches" "$WORKFLOW"; then
    echo "✘ [FALHA] $WORKFLOW nao dispara a esteira de joaju-web!"
    exit 1
  fi

  if ! grep -q "repos/lumenpink/mandu/actions/workflows/release-banca.yaml/dispatches" "$WORKFLOW"; then
    echo "✘ [FALHA] $WORKFLOW nao dispara a esteira de mandu!"
    exit 1
  fi
fi

echo "✔ [PASS] Auditoria GitOps cascade em joaju validada com sucesso."
