# Spec 75: Implementar Inspeção Detalhada com joaju --version --all

## 1. Contexto e Motivação
Para fins de auditoria em produção e diagnóstico em esteiras de integração contínua, o CLI do Joaju deve permitir inspecionar não apenas a versão do motor principal, mas o inventário completo de todos os submódulos e bibliotecas vinculadas.

## 2. Requisitos Técnicos
- Adicionar flags `--all` e `--json` ao CLI `joaju` ou subcomando correspondente.
- Quando executado com `--all`, imprimir uma árvore hierárquica contendo a versão de cada submódulo ativo do ecossistema:
  - `joaju-pdf`
  - `joaju-chem`
  - `joaju-pedigree`
  - `joaju-table`
  - `joaju-markdown`
  - `joaju-templates`
  - `joaju-lirica`
  - `joaju-slides`
  - `joaju-wireframe`
  - `joaju-sequence`
  - `joaju-graph`
- Quando executado com `--all --json`, emitir JSON estruturado para consumo automatizado.
- Expor função `pub fn ecosystem_inventory() -> Vec<(&'static str, &'static str)>` em `src/lib.rs`.

## 3. Plano de Testes (DD-TDD)
- Fase 4: Teste comprovadamente falho `tests/cli_version_all_spec.rs` chamando `ecosystem_inventory()`.
- Fase 5: Implementação funcional.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict`.
