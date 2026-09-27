#!/usr/bin/env bash
# ==============================================================================
# ☭ [ГОСТ / GOSPLAN] JOAJU VDNKh DEMO PREPARATION & RUNNER
# Interface de demonstração integrada ao 'cccp serve --demo'
# ==============================================================================

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PORT="${PORT:-3000}"
HOST="${HOST:-0.0.0.0}"
START_SERVER=0
OPEN_BROWSER=0

# Cores ANSI (observando NO_COLOR e detecção de TTY - Seção 13.4)
if [ -z "${NO_COLOR:-}" ] && [ -t 1 ]; then
    BOLD='\033[1m'
    CYAN='\033[0;36m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    GOLD='\033[38;5;220m'
    NC='\033[0m'
else
    BOLD=''
    CYAN=''
    GREEN=''
    YELLOW=''
    GOLD=''
    NC=''
fi

print_help() {
    cat << EOF
★ JOAJU - Pavilhão de Demonstração VDNKh ★

Uso:
  ./scripts/demo.sh [opções]

Opções:
  -p, --port <N>   Porta para executar o servidor (padrão: 3000)
  -s, --start      Iniciar o servidor Joaju após preparar dados de demonstração
  -o, --open       Abrir navegador automaticamente (requer --start)
  -h, --help       Exibir esta mensagem de ajuda

Compatibilidade CCCP:
  Quando executado por 'cccp serve --demo', semeia os artefatos de demonstração
  em data/demo/ e valida a integridade do ambiente.
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        -p|--port)
            shift
            [ $# -gt 0 ] || { echo "Erro: --port requer número." >&2; exit 1; }
            PORT="$1"
            shift
            ;;
        -s|--start)
            START_SERVER=1
            shift
            ;;
        -o|--open)
            OPEN_BROWSER=1
            START_SERVER=1
            shift
            ;;
        -h|--help)
            print_help
            exit 0
            ;;
        *)
            # Ignora argumentos desconhecidos para compatibilidade com cccp
            shift
            ;;
    esac
done

echo -e "${GOLD}${BOLD}★════════════════════════════════════════════════════════════════════════════★${NC}"
echo -e "${GOLD}${BOLD}  ☭ ВДНХ — JOAJU: COMPILADOR ACADÊMICO & PIPELINE DE HEREDOGRAMAS ☭${NC}"
echo -e "${GOLD}${BOLD}  Pavilhão de Demonstração & Prontidão Operacional${NC}"
echo -e "${GOLD}${BOLD}★════════════════════════════════════════════════════════════════════════════★${NC}"

# 1. Semear fixtures de demonstração em data/demo/
DEMO_DIR="${ROOT_DIR}/data/demo"
mkdir -p "$DEMO_DIR"

# 1.1 Heredograma clínico de amostra
cat << 'EOF' > "${DEMO_DIR}/heredograma_clinico.csv"
id,pai,mae,sex,fenotipo,label
1,0,0,M,wt,I.1 (Avô)
2,0,0,F,wt,I.2 (Avó)
3,1,2,M,het,II.1 (Pai Portador)
4,0,0,F,wt,II.2 (Mãe Não-Afetada)
5,3,4,F,hom,III.1 (Probanda Afetada)
6,3,4,M,wt,III.2 (Irmão Hígido)
EOF

# 1.2 Artigo Markdown com heredograma e fluxo Mermaid integrados
cat << 'EOF' > "${DEMO_DIR}/artigo_clinico.md"
# Investigação Genômica de Variante Familiar Rara

## Resumo
Apresentamos a investigação de segregação fenotípica de variante candidata
em uma linhagem familiar com três gerações documentadas.

## Heredograma Clínico Familiar
```heredograma
%% layout.width=90% layout.max_width=550pt cex=0.85
id,pai,mae,sex,fenotipo,label
1,0,0,M,wt,Avô
2,0,0,F,wt,Avó
3,1,2,M,het,Pai
4,0,0,F,wt,Mãe
5,3,4,F,hom,Probanda
6,3,4,M,wt,Irmão
```

## Fluxo Laboratorial e Conduta
```mermaid
%% width=85% max-width=600pt
graph TD
    A[Coleta Amostra Sanguínea] --> B[Sequenciamento NGS Painel Clínico]
    B --> C{Variante Patogênica Detectada?}
    C -->|Sim| D[Aconselhamento Genético Pré e Pós-Teste]
    C -->|Não| E[Expansão para WES / Trio Familiar]
    D --> F[Laudo Molecular Definitivo ABNT/NSGC]
```

## Conclusão
A identificação precoce através do pipeline reprodutível permitiu direcionamento
terapêutico preciso conforme as diretrizes do CFM e NSGC.
EOF

echo -e "  ${GREEN}✔ Amostras de demonstração geradas em:${NC} ${CYAN}${DEMO_DIR}/${NC}"
echo -e "    - ${YELLOW}data/demo/heredograma_clinico.csv${NC}"
echo -e "    - ${YELLOW}data/demo/artigo_clinico.md${NC}"
echo -e "  ${GREEN}✔ Web Playground pronto em:${NC} ${CYAN}public/index.html${NC}"

# Se disparado com --start, executa o servidor
if [ "$START_SERVER" -eq 1 ]; then
    SERVER_ARGS=("-p" "$PORT" "--host" "$HOST")
    [ "$OPEN_BROWSER" -eq 1 ] && SERVER_ARGS+=("-o")
    echo -e "\n${CYAN}🚀 Iniciando servidor Joaju com fixtures de demonstração...${NC}\n"
    exec bash "${ROOT_DIR}/scripts/dev.sh" "${SERVER_ARGS[@]}"
fi

echo -e "\n${GREEN}✔ Preparação VDNKh concluída com sucesso.${NC}\n"
exit 0
