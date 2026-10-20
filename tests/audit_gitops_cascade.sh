#!/usr/bin/env bash
# ==============================================================================
# tests/audit_gitops_cascade.sh
# Valida integracao de workflow GitOps e cascata em joaju (Ticket #44)
# ==============================================================================
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

WORKFLOW=".forgejo/workflows/ci.yaml"

if [ -f ".forgejo/workflows/ci.yml" ]; then
  echo "✘ [FALHA] Arquivo ci.yml legado ainda presente!"
  exit 1
fi

if [ ! -f "$WORKFLOW" ]; then
  echo "✘ [FALHA] $WORKFLOW inexistente!"
  exit 1
fi

if ! grep -q "workflow_dispatch:" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao declara 'workflow_dispatch:'!"
  exit 1
fi

if ! grep -q "sparse+https://git.2lp.in/api/packages/lumen/cargo/" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao configura endpoint sparse do registry lumen!"
  exit 1
fi

if ! grep -q "cargo publish --registry lumen" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao publica no registry soberano lumen!"
  exit 1
fi

if ! grep -q "repos/lumenpink/joaju-web/actions/workflows/ci.yaml/dispatches" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao dispara a esteira de joaju-web!"
  exit 1
fi

echo "✔ [PASS] Auditoria GitOps cascade em joaju validada com sucesso."
