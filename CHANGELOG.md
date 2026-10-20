# Changelog

## [Unreleased]

### Features
  - (registry) migracao das dependencias satelites para o registro soberano lumen e publicacao da crate v0.1.0 (closes #43)
  - (pipeline) fonte unica de dados SSOT via CSV com projecoes e filtros (closes #21)
  - (modularizacao) desacoplamento soberano em crates ssot-rs, table-rs e unfurl-rs (#21)
  - add canonical engines, zip workspace compilation and eliminate legacy tags
  - (render) adicionar motores openchemlib, openbabel e tabelas academicas booktabs
  - (render) adicionar motor pedtools, canonicalizar tags e suportar fontes remotas
  - (render) adicionar motores leves d2, graphviz, plantuml e padronizar kinship
  - (pedigree) adicionar modal com guia de campos e corrigir tratamento de erro 422
  - (branding) reposicionar foco em diagramas academicos e fundo branco obrigatorio
  - (ci) implement biome linter, stryker, renv.lock, no_color, and ci quality gates (closes #16)
  - (i18n) implement 10 canonical languages and rtl support conforming to gost (closes #15)
  - (docs) implement public documentation site, security headers, and kokorosim banner (closes #14)
  - (api) implement cccp serve, dev, and demo interfaces and health endpoint (closes #13)
  - (ui) implement interactive web playground and preview endpoint (closes #10)
  - (docker) update runtimes to latest and apply multi-stage non-root oci standards (closes #8)
  - (ci) add CI/CD pipeline, Docker compose setup, and Node.js project configuration (closes #7)
  - (api) implement compiler orchestrator and regex preprocessors (closes #5)
  - (templates) add typst abnt template and csl citation styles (closes #4)
  - (pedigree) add r kinship2 pedigree generator (closes #3)
  - (docker) setup multi-engine container orchestration (closes #2)

### Bug Fixes
  - (compiler) improve typst 0.11 compatibility, kinship sex mapping and pandoc rules
  - (kroki) aumentar timeout para 20s em instancias de cpu compartilhada
  - (resilience) adicionar trava anti-empilhamento, debounce 800ms e status 422
  - (pedigree) ajustar proporcoes compactas de geracoes e ampliar simbolos

## Previous Releases

