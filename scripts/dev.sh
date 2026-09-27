#!/usr/bin/env bash
# ==============================================================================
# ☭ [ГОСТ / GOSPLAN] JOAJU DEV WATCHER & PIPELINE RUNNER
# Interface de desenvolvimento local integrada ao 'cccp serve --dev'
# ==============================================================================

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PORT="${PORT:-3000}"
HOST="${HOST:-0.0.0.0}"
OPEN_BROWSER=0

# Cores ANSI para padronização Gosplan
BOLD='\033[1m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
GOLD='\033[38;5;220m'
NC='\033[0m'

print_help() {
    cat << EOF
★ JOAJU - Servidor de Desenvolvimento e Observador Local ★

Uso:
  ./scripts/dev.sh [opções]

Opções:
  -p, --port <N>   Porta para o servidor (padrão: 3000 ou variável PORT)
  -l, --localhost  Vincular exclusivamente em 127.0.0.1
  --host <ip>      IP customizado para bind
  -o, --open       Abrir navegador automaticamente após inicialização
  -h, --help       Exibir esta mensagem de ajuda

Compatibilidade CCCP:
  Disparado automaticamente por: cccp serve --dev
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        -p|--port)
            shift
            [ $# -gt 0 ] || { echo -e "${RED}Erro: --port requer um número.${NC}" >&2; exit 1; }
            PORT="$1"
            shift
            ;;
        -l|--localhost)
            HOST="127.0.0.1"
            shift
            ;;
        --host)
            shift
            [ $# -gt 0 ] || { echo -e "${RED}Erro: --host requer um endereço IP.${NC}" >&2; exit 1; }
            HOST="$1"
            shift
            ;;
        -o|--open)
            OPEN_BROWSER=1
            shift
            ;;
        -h|--help)
            print_help
            exit 0
            ;;
        *)
            echo -e "${RED}Erro: Argumento desconhecido: $1${NC}" >&2
            print_help
            exit 1
            ;;
    esac
done

# Verificação de dependência do Node.js
if ! command -v node >/dev/null 2>&1; then
    echo -e "${RED}❌ Erro: Node.js não foi encontrado no PATH.${NC}" >&2
    exit 1
fi

# Verificação do serviço de diagramação Kroki (porta 8000)
KROKI_HOST="${KROKI_HOST:-http://localhost:8000}"
if command -v curl >/dev/null 2>&1; then
    if ! curl -sf "${KROKI_HOST}/health" >/dev/null 2>&1 && ! curl -sf "${KROKI_HOST}" >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  Aviso: Serviço Kroki não detectado em ${KROKI_HOST}.${NC}"
        echo -e "${YELLOW}   Diagramas Mermaid/PlantUML remotos exigem o Kroki ativo.${NC}"
        echo -e "${YELLOW}   Para iniciar: ${BOLD}docker compose up -d kroki${NC}\n"
    fi
fi

# Verificação e incremento de porta caso já esteja ocupada
is_port_in_use() {
    local check_port="$1"
    if command -v nc >/dev/null 2>&1; then
        nc -z -w 1 127.0.0.1 "$check_port" >/dev/null 2>&1
    elif command -v lsof >/dev/null 2>&1; then
        lsof -i :"$check_port" >/dev/null 2>&1
    else
        return 1
    fi
}

ORIGINAL_PORT="$PORT"
while is_port_in_use "$PORT"; do
    echo -e "${YELLOW}ℹ️  Porta ${PORT} ocupada. Tentando $((PORT + 1))...${NC}"
    PORT=$((PORT + 1))
done

# Banner visual Gosplan
echo -e "${GOLD}${BOLD}╔═══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GOLD}${BOLD}║  ☭ [ГОСТ / GOSPLAN] JOAJU DEV WATCHER & PIPELINE COMPILER         ║${NC}"
echo -e "${GOLD}${BOLD}╚═══════════════════════════════════════════════════════════════════╝${NC}"
echo -e "  📡 Web Playground:  ${CYAN}${BOLD}http://${HOST}:${PORT}/${NC}"
echo -e "  🩺 Health Probe:    ${CYAN}${BOLD}http://${HOST}:${PORT}/health${NC}"
echo -e "  🧬 Kroki Backend:   ${GREEN}${KROKI_HOST}${NC}"
echo -e "  👀 Hot Watcher:     ${YELLOW}node --watch server.js${NC}"
echo -e "  ⌨️  Encerrar:        ${BOLD}Ctrl+C${NC}"
echo ""

SERVER_PID=""

cleanup() {
    trap - SIGINT SIGTERM EXIT
    echo ""
    if [ -n "$SERVER_PID" ] && kill -0 "$SERVER_PID" 2>/dev/null; then
        echo -e "${YELLOW}🛑 Encerrando servidor de desenvolvimento (PID: ${SERVER_PID})...${NC}"
        kill -TERM "$SERVER_PID" 2>/dev/null || true
        wait "$SERVER_PID" 2>/dev/null || true
    fi
    echo -e "${GREEN}✔ Servidor Joaju finalizado com sucesso.${NC}"
    exit 0
}
trap cleanup SIGINT SIGTERM EXIT

# Disparo do servidor com recarga automática nativa do Node 22
PORT="$PORT" HOST="$HOST" node --watch server.js &
SERVER_PID=$!

# Health probe e abertura opcional de navegador
if [ "$OPEN_BROWSER" -eq 1 ]; then
    PROBE_HOST="$HOST"
    [ "$PROBE_HOST" = "0.0.0.0" ] && PROBE_HOST="127.0.0.1"
    HEALTH_URL="http://${PROBE_HOST}:${PORT}/health"
    
    echo -e "${CYAN}⏳ Aguardando prontidão do servidor para abrir o navegador...${NC}"
    ATTEMPTS=0
    READY=0
    while [ "$ATTEMPTS" -lt 30 ]; do
        if command -v curl >/dev/null 2>&1 && curl -sf "$HEALTH_URL" >/dev/null 2>&1; then
            READY=1
            break
        fi
        sleep 0.2
        ATTEMPTS=$((ATTEMPTS + 1))
    done

    if [ "$READY" -eq 1 ]; then
        TARGET_URL="http://localhost:${PORT}/"
        echo -e "${GREEN}🚀 Abrindo navegador em ${TARGET_URL}...${NC}"
        if command -v xdg-open >/dev/null 2>&1; then
            xdg-open "$TARGET_URL" >/dev/null 2>&1 &
        fi
    fi
fi

wait "$SERVER_PID"
