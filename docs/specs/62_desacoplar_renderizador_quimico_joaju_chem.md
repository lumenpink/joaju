# Especificação 62: Desacoplamento do Renderizador Químico para a Crate joaju-chem

- **ID do Ticket:** #62
- **Status:** Concluído (DD-TDD Fase 9)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Motivação
O compilador acadêmico Joaju (`joaju`) renderiza estruturas químicas e diagramas moleculares 2D a partir de blocos Markdown com notação SMILES (blocos ````openchemlib`` e ````openbabel``).

Historicamente, o Joaju manteve o código de renderização molecular incorporado diretamente em `src/engine/chem.rs`, puxando diretamente as crates `chematic-smiles` e `chematic-depict` no `Cargo.toml`. Todas as demais engines do Joaju (heredogramas clínicos em `joaju-pedigree`, apresentações em `joaju-slides`, poesia e escansão em `joaju-lirica`, tabelas ABNT em `joaju-table`, e metadados web em `joaju-unfurl`) já foram extraídas para crates soberanas e independentes.

Esta especificação definiu o desacoplamento do motor químico para a nova crate `joaju-chem`, eliminando `chematic-smiles` e `chematic-depict` como dependências diretas da raiz do Joaju e preservando total compatibilidade com a API pública `joaju::render_chem` e o pipeline do compilador.

## 2. Requisitos Técnicos
1. **Criação da Crate Soberana `joaju-chem`:**
   - Repositório `lumen/joaju-chem` na forja soberana `gambi.2lp.in`.
   - Exportação da função `render_chem(raw_code: &str, forced_format: Option<&str>) -> Result<(String, String), String>`.
   - Suporte a `rlib` e `staticlib` e CLI isolado via feature `cli`.
   - Dependências especializadas contidas em `joaju-chem`: `chematic-smiles`, `chematic-depict`, `regex`.
   - Crate publicada com sucesso no registro `gambi` (`v0.1.0`).

2. **Desacoplamento em `joaju`:**
   - Remoção das dependências diretas `chematic-smiles` e `chematic-depict` de `Cargo.toml`.
   - Inclusão de `joaju-chem = { version = "0.1.0", registry = "gambi" }`.
   - Reexportação limpa de `render_chem` em `src/engine/chem.rs`:
     ```rust
     pub use joaju_chem::render_chem;
     ```
   - Manutenção da interface pública `pub use engine::chem::render_chem;` em `src/lib.rs` 100% inalterada.

3. **Garantia de Não-Regressão:**
   - Teste Red `tests/test_no_chematic_direct_dependency.sh` garantindo a ausência de `chematic-*` no core.
   - Validação da suíte de testes: `tests/engine_chem_spec.rs`, `tests/audit_registry_dependencies.sh`, testes de CLI e compilador.

## 3. Critérios de Aceitação
1. `Cargo.toml` não contém `chematic-smiles` nem `chematic-depict`.
2. `tests/test_no_chematic_direct_dependency.sh` passa com sucesso.
3. `tests/engine_chem_spec.rs` compila e passa 100% dos testes.
4. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` executam sem qualquer violação.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #62
- [x] Fase 2 (Branch): `feat/62-desacoplar-renderizador-quimico-joaju-chem`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Criado teste red `tests/test_no_chematic_direct_dependency.sh`
- [x] Fase 5 (Green Implement): Publicado joaju-chem no gambi e integrado com sucesso
- [x] Fase 6 (Refactor): `src/engine/chem.rs` simplificado para reexportação pura
- [x] Fase 7 (Mutation Test): Testes de mutação e validação de regressão concluídos
- [x] Fase 8 (Quality Gate): fmt e clippy sem warnings, testes unitários e de integração verdes
- [x] Fase 9 (Finish & Close): Concluído e pronto para merge (closes #62)
