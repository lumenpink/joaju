#!/usr/bin/env bash
set -euo pipefail

WORKFLOW_FILE=".forgejo/workflows/ci.yml"
LEGACY_FILE=".forgejo/workflows/ci.yaml"
ROTATE_SCRIPT="/home/lumen/src/banca/scripts/rotate-downloads.py"

echo "=== [TEST] Validando conformidade da esteira soberana e publicação na Banca ==="

# 1. Deve existir estritamente .forgejo/workflows/ci.yml
if [ ! -f "$WORKFLOW_FILE" ]; then
    echo "FALHA: $WORKFLOW_FILE não existe!"
    exit 1
fi

if [ -f "$LEGACY_FILE" ]; then
    echo "FALHA: Arquivo legado $LEGACY_FILE ainda existe! Remova-o."
    exit 1
fi

# 2. Ausência estrita de Codeberg
if grep -qi "codeberg.org" "$WORKFLOW_FILE"; then
    echo "FALHA: $WORKFLOW_FILE contém referências ao Codeberg!"
    exit 1
fi

if grep -qi "CODEBERG_TOKEN" "$WORKFLOW_FILE"; then
    echo "FALHA: $WORKFLOW_FILE contém CODEBERG_TOKEN!"
    exit 1
fi

# 3. Presença de git.2lp.in e GIT_2LP_TOKEN
if ! grep -q "git.2lp.in" "$WORKFLOW_FILE"; then
    echo "FALHA: $WORKFLOW_FILE não referencia git.2lp.in!"
    exit 1
fi

if ! grep -q "GIT_2LP_TOKEN" "$WORKFLOW_FILE"; then
    echo "FALHA: $WORKFLOW_FILE não referencia GIT_2LP_TOKEN!"
    exit 1
fi

# 4. Imagem atômica hermética
if ! grep -q "lumenpink/gost-ci-cross:latest" "$WORKFLOW_FILE"; then
    echo "FALHA: $WORKFLOW_FILE não utiliza a imagem atômica lumenpink/gost-ci-cross:latest!"
    exit 1
fi

# 5. Dependência estrita: deploy-banca precisa de test
if ! grep -q "needs: \[test\]" "$WORKFLOW_FILE"; then
    echo "FALHA: deploy-banca deve depender estritamente de needs: [test]!"
    exit 1
fi

# 6. Alvos multi-arquitetura
for target in "x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl" "x86_64-pc-windows-gnu"; do
    if ! grep -q "$target" "$WORKFLOW_FILE"; then
        echo "FALHA: Alvo $target ausente no workflow!"
        exit 1
    fi
done

# 7. Flags do rotate-downloads.py
for flag in "--manifest" "--channel" "--arch" "--version-str" "--url" "--sha256" "--tarball" "--app"; do
    if ! grep -q -- "$flag" "$WORKFLOW_FILE"; then
        echo "FALHA: Flag $flag do rotate-downloads.py ausente no workflow!"
        exit 1
    fi
done

# 8. Não deve haver comando posicional 'add' no rotate-downloads.py
if grep -E "rotate-downloads\.py.* add " "$WORKFLOW_FILE"; then
    echo "FALHA: rotate-downloads.py não possui subcomando posicional 'add'!"
    exit 1
fi

echo "SUCESSO: Todas as validações da esteira soberana passaram com êxito."
