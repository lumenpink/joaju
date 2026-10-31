# Joaju (djo-a-DJÚ)

[![crates.io](https://img.shields.io/crates/v/joaju.svg)](https://crates.io/crates/joaju)
[![docs.rs](https://docs.rs/joaju/badge.svg)](https://docs.rs/joaju)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.5.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

> **Joaju** (*Guarani*): Enlace, vínculo recíproco, conexão.  
> Orquestrador e compilador acadêmico nativo em Rust para publicações tipográficas Typst, tabelas e heredogramas clínicos.

## 📌 Visão Geral

Joaju é uma esteira de diagramação acadêmica e compilação técnica com foco em privacidade estrita, execução determinística e conformidade LGPD:

- **joaju-csv**: Processamento tabular de alto desempenho e geração de tabelas acadêmicas (ABNT, IBGE, APA).
- **joaju-pedigree**: Renderização determinística de diagramas e heredogramas clínicos conformes NSGC/ACMG.
- **joaju-unfurl**: Resolução de metadados e geração de cartões visuais ricos (OpenGraph, Twitter Cards).
- **joaju-pdf**: Motor isolado de compilação Typst para PDF via biblioteca estática pura musl.

## 🚀 Instalação e Uso Rápido

```bash
# Compilação nativa com suporte estático
cargo build --release
```

Para a documentação completa de arquitetura, pipeline e especificações, consulte [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).
