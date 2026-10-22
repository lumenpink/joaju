#!/usr/bin/env bash
set -euo pipefail

WORKFLOW_FILE=".forgejo/workflows/ci.yml"
LEGACY_FILE=".forgejo/workflows/ci.yaml"
ROTATE_SCRIPT="/home/lumen/src/banca/scripts/rotate-downloads.py"

echo "=== [TEST] Validando conformidade da esteira soberana e publicação na Banca ==="

validate_workflow() {
    local file="$1"

    # 1. Deve existir estritamente .forgejo/workflows/ci.yml
    if [ ! -f "$file" ]; then
        echo "FALHA: $file não existe!"
        return 1
    fi

    if [ -f "$LEGACY_FILE" ]; then
        echo "FALHA: Arquivo legado $LEGACY_FILE ainda existe! Remova-o."
        return 1
    fi

    # 2. Ausência estrita de Codeberg
    if grep -qi "codeberg.org" "$file"; then
        echo "FALHA: $file contém referências ao Codeberg!"
        return 1
    fi

    if grep -qi "CODEBERG_TOKEN" "$file"; then
        echo "FALHA: $file contém CODEBERG_TOKEN!"
        return 1
    fi

    # 3. Presença de git.2lp.in e GIT_2LP_TOKEN
    if ! grep -q "git.2lp.in" "$file"; then
        echo "FALHA: $file não referencia git.2lp.in!"
        return 1
    fi

    if ! grep -q "GIT_2LP_TOKEN" "$file"; then
        echo "FALHA: $file não referencia GIT_2LP_TOKEN!"
        return 1
    fi

    # 4. Imagem atômica hermética
    if ! grep -q "lumenpink/gost-ci-cross:latest" "$file"; then
        echo "FALHA: $file não utiliza a imagem atômica lumenpink/gost-ci-cross:latest!"
        return 1
    fi

    # 5. Dependência estrita: deploy-banca precisa de test
    if ! grep -q "needs: \[test\]" "$file"; then
        echo "FALHA: deploy-banca deve depender estritamente de needs: [test]!"
        return 1
    fi

    # 6. Alvos multi-arquitetura
    for target in "x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl" "x86_64-pc-windows-gnu"; do
        if ! grep -q "$target" "$file"; then
            echo "FALHA: Alvo $target ausente no workflow!"
            return 1
        fi
    done

    # 7. Flags do rotate-downloads.py
    for flag in "--manifest" "--channel" "--arch" "--version-str" "--url" "--sha256" "--tarball" "--app"; do
        if ! grep -q -- "$flag" "$file"; then
            echo "FALHA: Flag $flag do rotate-downloads.py ausente no workflow!"
            return 1
        fi
    done

    # 8. Não deve haver comando posicional 'add' no rotate-downloads.py
    if grep -E "rotate-downloads\.py.* add " "$file"; then
        echo "FALHA: rotate-downloads.py não possui subcomando posicional 'add'!"
        return 1
    fi

    return 0
}

# Validação do arquivo canônico
validate_workflow "$WORKFLOW_FILE"
echo "✔ Validação canônica passou."

# === Teste de Mutação (Fase 7 DD-TDD) ===
echo "=== [MUTATION TEST] Verificando sensibilidade a mutantes ==="
TMP_MUTANT=$(mktemp)
trap 'rm -f "$TMP_MUTANT"' EXIT

# Mutante 1: Injeção de Codeberg
sed 's/git\.2lp\.in/codeberg.org/g' "$WORKFLOW_FILE" > "$TMP_MUTANT"
if validate_workflow "$TMP_MUTANT" >/dev/null 2>&1; then
    echo "FALHA DE MUTAÇÃO: Mutante com codeberg.org não foi detectado!"
    exit 1
fi
echo "✔ Mutante 1 (codeberg.org) detectado e neutralizado."

# Mutante 2: Remoção do needs: [test]
grep -v "needs: \[test\]" "$WORKFLOW_FILE" > "$TMP_MUTANT"
if validate_workflow "$TMP_MUTANT" >/dev/null 2>&1; then
    echo "FALHA DE MUTAÇÃO: Mutante sem needs: [test] não foi detectado!"
    exit 1
fi
echo "✔ Mutante 2 (ausência de needs: [test]) detectado e neutralizado."

# Mutante 3: Inserção do subcomando posicional 'add' no rotate-downloads.py
sed 's/rotate-downloads\.py/rotate-downloads.py add/g' "$WORKFLOW_FILE" > "$TMP_MUTANT"
if validate_workflow "$TMP_MUTANT" >/dev/null 2>&1; then
    echo "FALHA DE MUTAÇÃO: Mutante com comando posicional 'add' não foi detectado!"
    exit 1
fi
echo "✔ Mutante 3 (comando posicional 'add') detectado e neutralizado."

# Mutante 4: Remoção da flag --app
grep -v -- "--app" "$WORKFLOW_FILE" > "$TMP_MUTANT"
if validate_workflow "$TMP_MUTANT" >/dev/null 2>&1; then
    echo "FALHA DE MUTAÇÃO: Mutante sem flag --app não foi detectado!"
    exit 1
fi
echo "✔ Mutante 4 (ausência de flag --app) detectado e neutralizado."

echo "SUCESSO: Todas as mutações foram eliminadas com 100% de precisão."
