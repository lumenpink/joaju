# Especificação 91: Bump Joaju para 0.2.0-dev.139

- **ID do Ticket:** #91
- **Status:** Concluído (DD-TDD Fase 9)
- **Autora:** Lumen
- **Alinhamento:** GOST 3.0.0 (Rust)

## 1. Contexto & Motivação
A publicação do `joaju` no registry `gambi` durante a CI falhou porque o prefixo SemVer `0.2.0-dev.138` já havia sido publicado anteriormente. O registry ignora o build metadata (`+hash`) para efeitos de duplicidade de versão. Como consequência, os repositórios consumidores (`mandu-core`, `mandu-mobile`) estavam recorrendo a caminhos locais `path = "../joaju"`, o que quebrou a CI deles em ambientes de contêiner isolados (onde os repositórios irmãos não existem no mesmo diretório).

## 2. Decisão Arquitetural (Para Natasha)
Incrementar formalmente o SemVer pré-release para `0.2.0-dev.139` nos arquivos `Cargo.toml` e `VERSION`. Isso permite que a CI do `joaju` publique com sucesso o novo crate contendo `ReferenceSource` no registry `gambi`, liberando os repositórios dependentes para remover as referências a `path = ...`.

## 3. Requisitos Técnicos
1. Atualizar `Cargo.toml` para `version = "0.2.0-dev.139"`.
2. Atualizar `VERSION` para `0.2.0-dev.139+20261120.69acc9e`.
3. Validar qualidade (`cargo fmt --check`, `cargo clippy`, `cargo test`).
