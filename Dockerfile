FROM node:20-bookworm-slim

# Evita prompts interativos durante a instalação do sistema
ENV DEBIAN_FRONTEND=noninteractive

# Instala dependências básicas, R, kinship2, Pandoc e fontes Liberation
RUN apt-get update && apt-get install -y --no-install-recommends \
    pandoc \
    curl \
    ca-certificates \
    xz-utils \
    fonts-liberation \
    r-base-core \
    r-cran-kinship2 \
    && rm -rf /var/lib/apt/lists/*

# Instala o binário oficial do Typst (arquitetura x86_64 estática)
RUN TYPST_VERSION="0.11.1" && \
    curl -fsSL "https://github.com/typst/typst/releases/download/v${TYPST_VERSION}/typst-x86_64-unknown-linux-musl.tar.xz" | \
    tar -xJ -C /tmp && \
    mv /tmp/typst-x86_64-unknown-linux-musl/typst /usr/local/bin/typst && \
    chmod +x /usr/local/bin/typst && \
    rm -rf /tmp/typst*

WORKDIR /app

# Inicializa o projeto Node.js e instala o Express (servidor web)
RUN npm init -y && npm install express uuid fetch

# Copia os dois scripts cruciais da aplicação para dentro da imagem
COPY server.js /app/server.js
COPY plot_pedigree.R /app/plot_pedigree.R

# Cria os diretórios necessários (data para volumes e tmp para compilação)
RUN mkdir -p /data /tmp && chmod 777 /tmp

EXPOSE 3000

# Inicia o servidor Node.js orquestrador
CMD ["node", "server.js"]