# Especificação 40: Esteira Soberana Multi-arquitetura e Publicação na Banca para Joaju

## 1. Contexto e Objetivo
O `joaju` é o compilador e orquestrador tipográfico soberano central do ecossistema Lumen.
Atualmente, sua esteira de CI/CD utilizava um arquivo legado (`ci.yaml`), instalava ferramentas dinamicamente em runtime e continha dependências e referências ao Codeberg.

O objetivo desta especificação é padronizar a automação de CI/CD no arquivo atômico canônico `.forgejo/workflows/ci.yml`, eliminando qualquer menção ou dependência do Codeberg e integrando a compilação multiarquitetura estática com publicação soberana direta no catálogo da Banca (`https://banca.2lp.in/downloads/joaju/`).

## 2. Requisitos Técnicos

### 2.1 Forja e Autenticação Soberana
- O repositório canônico remoto é exclusivamente `git.2lp.in:lumen/joaju.git`.
- Toda autenticação interna e chamadas de API ou git clones utilizam `GIT_2LP_TOKEN` e `https://git.2lp.in/`.
- Fica estritamente vedada qualquer referência ao `codeberg.org` ou `CODEBERG_TOKEN`.

### 2.2 Imagem OCI Atômica e Hermética
- Todos os jobs da esteira executam exclusivamente no contêiner `docker.io/lumenpink/gost-ci-cross:latest`.
- Não é permitida a instalação de ferramentas em tempo de execução (`curl | sh`, `apt-get`, etc.).

### 2.3 Estrutura de Jobs
1. **`test` ("Checks, Tests & Quality Gates")**:
   - `cargo fmt -- --check`
   - `cargo clippy --all-targets -- -D warnings`
   - `cargo test --all-targets`
   - `cccp check --strict` (quando disponível)
2. **`deploy-banca` ("Build & Publish Joaju Multiarch to Banca")**:
   - Condição estrita: `needs: [test]`, `if: success() && (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/tags/v')) && (github.event_name == 'push' || github.event_name == 'workflow_dispatch')`.
   - Compilação dos 3 alvos:
     - `x86_64-unknown-linux-musl` (Linux AMD64)
     - `aarch64-unknown-linux-musl` (Linux ARM64)
     - `x86_64-pc-windows-gnu` (Windows AMD64)
   - Empacotamento em `dist/packages/`:
     - `joaju-${VERSION}-linux-amd64.tar.gz`
     - `joaju-${VERSION}-linux-arm64.tar.gz`
     - `joaju-${VERSION}-windows-amd64.zip`
   - Geração de `SHA256SUMS`.
   - Upload para o Cloudflare R2 (`twolp-bucket/downloads/joaju/${CHANNEL}/...`).
   - Clone raso de `https://lumen:${GIT_2LP_TOKEN}@git.2lp.in/lumen/banca.git`.
   - Invocação do `rotate-downloads.py` com rotação de manifesto:
     - `--manifest site/downloads/joaju/index.json`
     - `--channel ${CHANNEL}`
     - `--arch ${ARCH}`
     - `--version-str ${VERSION}`
     - `--url ${DOWNLOAD_URL}`
     - `--sha256 ${HASH}`
     - `--tarball ${PKG_FILE}`
     - `--app "joaju"`
   - Commit e push soberano para o repositório da Banca.

### 2.4 Crate Registry Soberano
- Publicação da crate no registry soberano Lumen (`sparse+https://git.2lp.in/api/packages/lumen/cargo/`) mantida no workflow de release.
