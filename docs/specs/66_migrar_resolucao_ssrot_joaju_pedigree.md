# Especificação 66: Migração da Resolução Tabular SSROT para joaju-pedigree

- **ID do Ticket:** #66
- **Status:** Concluído (DD-TDD Fase 9)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Motivação
O compilador acadêmico Joaju (`joaju`) renderiza heredogramas clínicos conformes aos padrões NSGC/ACMG delegando para a biblioteca `joaju-pedigree`.

Originalmente, blocos de código com diretivas tabulares SSROT (`src`, `file`, `select`, `where`) eram processados por código embutido diretamente no Joaju (`src/engine/pedigree.rs`), forçando o Joaju a manter uma dependência direta de `joaju-csv` na raiz do [Cargo.toml](file:///home/lumen/src/joaju/Cargo.toml).

Como a própria crate `joaju-pedigree` já possui suporte tabular através de sua dependência interna de `joaju-csv`, a responsabilidade de interpretar diretivas de dados e renderizar blocos de heredograma pertence conceitualmente à `joaju-pedigree`.

Esta especificação concluiu a remoção completa da dependência `joaju-csv` da raiz do Joaju, atualizando `joaju-pedigree` para a versão `0.2.1` e delegando a função `render_pedigree` diretamente para a crate especializada.

## 2. Requisitos Técnicos
1. **Atualização de joaju-pedigree:**
   - Dependência elevada para `0.2.1` no registro `gambi`.
   - Removida dependência `joaju-csv = { version = "0.1.0", registry = "gambi" }` de `Cargo.toml`.

2. **Refatoração de `src/engine/pedigree.rs`:**
   - Reexportação de `render_pedigree`, `RenderRequest`, `RenderResponse` diretamente de `joaju_pedigree`:
     ```rust
     pub use joaju_pedigree::{render_pedigree, RenderRequest, RenderResponse};
     ```
   - Eliminado código legado de split e filtragem tabular em `src/engine/pedigree.rs`.

3. **Garantia de Não-Regressão:**
   - Teste Red `tests/test_no_joaju_csv_dependency.sh` aprovado.
   - Suíte de testes de heredograma e SSROT passando: `engine_pedigree_spec`, `exhaustive_spec`, `test_banca_pipeline_canonical.sh`.

## 3. Critérios de Aceitação
1. `Cargo.toml` não contém `joaju-csv`.
2. `tests/test_no_joaju_csv_dependency.sh` passa com sucesso.
3. Todos os testes de heredograma passam 100%.
4. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem avisos.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #66
- [x] Fase 2 (Branch): `feat/66-migrar-resolucao-ssrot-joaju-pedigree`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Criado teste red `tests/test_no_joaju_csv_dependency.sh`
- [x] Fase 5 (Green Implement): Atualizado joaju-pedigree para 0.2.1 e removido joaju-csv
- [x] Fase 6 (Refactor): Reexportação direta de render_pedigree
- [x] Fase 7 (Mutation Test): Testes de mutação e regressão aprovados
- [x] Fase 8 (Quality Gate): fmt e clippy sem warnings, testes verdes
- [x] Fase 9 (Finish & Close): Concluído e pronto para merge (closes #66)
