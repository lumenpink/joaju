# ==============================================================================
# Estágio 1: Download de binários estáticos (Typst e D2)
# ==============================================================================
FROM node:22-bookworm-slim AS binaries-downloader

ARG TYPST_VERSION="0.15.1"
ARG D2_VERSION="0.9.0"
ARG PANDOC_VERSION="3.6.3"

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    xz-utils \
    && rm -rf /var/lib/apt/lists/* \
    && curl -fsSL "https://github.com/typst/typst/releases/download/v${TYPST_VERSION}/typst-x86_64-unknown-linux-musl.tar.xz" | \
       tar -xJ -C /tmp \
    && mv /tmp/typst-x86_64-unknown-linux-musl/typst /usr/local/bin/typst \
    && chmod +x /usr/local/bin/typst \
    && curl -fsSL "https://github.com/d2lang/d2/releases/download/v${D2_VERSION}/d2-v${D2_VERSION}-linux-amd64.tar.gz" | \
       tar -xz -C /tmp \
    && mv /tmp/d2-v${D2_VERSION}/bin/d2 /usr/local/bin/d2 \
    && chmod +x /usr/local/bin/d2 \
    && curl -fsSL "https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-linux-amd64.tar.gz" | \
       tar -xz -C /tmp \
    && mv /tmp/pandoc-${PANDOC_VERSION}/bin/pandoc /usr/local/bin/pandoc \
    && chmod +x /usr/local/bin/pandoc

# ==============================================================================
# Estágio 2: Instalação hermética de dependências Node.js
# ==============================================================================
FROM node:22-bookworm-slim AS node-builder

WORKDIR /app
COPY package*.json ./
RUN npm ci --omit=dev

# ==============================================================================
# Estágio 3: Imagem final de runtime (Multi-stage, Non-root)
# ==============================================================================
FROM node:22-bookworm-slim AS runtime

ENV DEBIAN_FRONTEND=noninteractive \
    NODE_ENV=production \
    PORT=3000

# Dependências de sistema: R, quadprog, Matrix, knitr e fontes Liberation
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    fonts-liberation \
    r-base-core \
    r-cran-matrix \
    r-cran-quadprog \
    r-cran-lpsolve \
    r-cran-knitr \
    && rm -rf /var/lib/apt/lists/* \
    && Rscript -e "install.packages(c('kinship2', 'pedtools'), repos='https://cloud.r-project.org')"

# Copia binários estáticos compilados
COPY --from=binaries-downloader /usr/local/bin/typst /usr/local/bin/typst
COPY --from=binaries-downloader /usr/local/bin/d2 /usr/local/bin/d2
COPY --from=binaries-downloader /usr/local/bin/pandoc /usr/local/bin/pandoc

WORKDIR /app

# Cria pastas necessárias e define permissões para o usuário node (UID 1000)
RUN mkdir -p /data /tmp /app \
    && chown -R node:node /app /data /tmp \
    && chmod 1777 /tmp

# Copia dependências e artefatos de código com ownership do usuário node
COPY --chown=node:node --from=node-builder /app/node_modules /app/node_modules
COPY --chown=node:node package.json /app/package.json
COPY --chown=node:node server.js /app/server.js
COPY --chown=node:node unfurl.js /app/unfurl.js
COPY --chown=node:node plot_pedigree.R /app/plot_pedigree.R
COPY --chown=node:node plot_pedtools.R /app/plot_pedtools.R
COPY --chown=node:node public /app/public
COPY --chown=node:node locales /app/locales

USER node

EXPOSE 3000

CMD ["node", "server.js"]