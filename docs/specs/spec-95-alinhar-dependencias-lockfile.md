# Spec #95: Alinhar Dependências Internas e Eliminar Duplicatas no Lockfile

## 1. Contexto e Motivação
O repositório agregador `joaju` travou o motor de bumps automáticos em cascata (`kn cascade`) devido a duas causas combinadas:
1. Convivência de duas versões de `joaju-table` (`0.1.0` puxada pelo antigo `joaju-markdown`, e `0.2.0-dev.56` declarada diretamente no `Cargo.toml`), gerando `error: specification 'joaju-table' is ambiguous` no Cargo.
2. Defasagem das dependências internas (`joaju-table = =0.2.0-dev.56` vs `joaju-pedigree = =0.2.0-dev.100`), impedindo a resolução de versões compartilhadas como `joaju-csv`.

Com o `joaju-markdown` agora publicado na linha `0.2.0-dev` de `joaju-table` (`v0.1.0-dev.47`), é necessário atualizar as dependências do `joaju` para as versões canônicas e regenerar o `Cargo.lock` unificado.

## 2. Escopo
- Atualizar dependências em `Cargo.toml`:
  - `joaju-table = "=0.2.0-dev.66"`
  - `joaju-pedigree = "=0.2.0-dev.108"`
  - `joaju-markdown = "=0.1.0-dev.51"`
- Atualizar teste `tests/deps_exact_spec.rs` para asserir as novas versões canônicas.
- Regenerar `Cargo.lock` eliminando a versão órfã `0.1.0` de `joaju-table`.
- Validar conformidade normativa (`cccp check --strict`) e integridade funcional (`cargo test`).

## 3. Plano de Implementação (GOST DD-TDD)
1. **Fase 3 (Spec):** Este documento técnico com justificativas explícitas.
2. **Fase 4 (Red Test):** Atualização de `tests/deps_exact_spec.rs` com as novas versões antes de alterar `Cargo.toml` (falha comprovada).
3. **Fase 5 (Green Implement):** Atualização do `Cargo.toml` e `cargo update` das crates alvo.
4. **Fase 6 (Refactor):** Verificação de zero avisos no clippy e formatação.
5. **Fase 7 (Mutation):** N/A (manutenção de dependências).
6. **Fase 8 (Quality Gate):** `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cccp check --strict`.
7. **Fase 9 (Finish):** Laudo e fechamento do ciclo.

## 4. Árvore de Decisão e Guardiã da Natasha
- **Por que atualizar `table`, `pedigree` e `markdown` em conjunto?** Porque eles formam um grafo com dependências compartilhadas (`joaju-csv`). No início do ticket #95, `joaju-markdown 0.1.0-dev.47` ainda puxava `joaju-table 0.2.0-dev.62`. Para evitar conflitos de lockfile, publicamos primeiro `joaju-table 0.2.0-dev.66` (com `joaju-csv 0.2.0-dev.49`) e `joaju-markdown 0.1.0-dev.51` (com `joaju-table 0.2.0-dev.66`). Assim, todas as dependências transitivas convergem em `joaju-csv 0.2.0-dev.49`. Natasha precisa encontrar o grafo 100% coeso e documentado.
- **Alternativa descartada:** Remover a pinagem estrita `=` das dependências internas. Descartado porque nas versões `-dev`, a pinagem estrita é convenção GOST para garantir determinismo absoluto em builds cross e C-ABI.

## 5. Laudo de Validação da Fase 8 (Quality Gate)
- **cargo test:** 100% aprovado em todas as suítes (unittests, `deps_exact_spec`, integrações).
- **cargo clippy:** Zero warnings com flags estritas (`--all-targets -- -D warnings`).
- **cargo fmt:** Em conformidade com o estilo oficial do projeto.
- **cccp check --strict:** 24/24 PASS (100% de compliance normativa).
- **Lockfile unificado:** Versão ambígua `0.1.0` de `joaju-table` eliminada. Unificação estrita em `joaju-table = =0.2.0-dev.66`, `joaju-pedigree = =0.2.0-dev.108`, `joaju-markdown = =0.1.0-dev.51` e `joaju-csv = =0.2.0-dev.49`.
