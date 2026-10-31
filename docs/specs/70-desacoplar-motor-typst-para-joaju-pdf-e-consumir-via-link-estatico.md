# Especificação 70: Desacoplar Motor Typst para joaju-pdf e Consumir via Link Estático

- **ID do Ticket:** #70
- **Status:** Em Andamento (DD-TDD Fase 3)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Motivação
A crate `joaju` atua como o orquestrador acadêmico e compilador de documentos unificado. Historicamente, ele mantinha a pilha completa do compilador Typst (`typst-as-lib`, `typst-pdf`, `typst-layout`, `typst`) diretamente em seu `Cargo.toml`. Isso arrastava mais de 360 dependências transitivas e exigia de 15 a 20 minutos de compilação em alvos estáticos limpos.

Para viabilizar compilações rápidas e modulares — permitindo alterar filtros, tabelas, CSV ou pré-processadores sem recompilar o motor Typst:
1. O compilador Typst foi extraído para a biblioteca dedicada `joaju-pdf`, que gera e disponibiliza a biblioteca estática musl pré-compilada `libjoaju_pdf.a` (`joaju-pdf-static/0.1.0`).
2. O `joaju` passa a consumir o `libjoaju_pdf.a` via `build.rs`, invocando a FFI C-ABI para geração de PDFs.
3. As dependências pesadas do Typst são completamente expurgadas de `joaju/Cargo.toml`.
4. O componente `joaju-table` permanece rigorosamente desacoplado e autônomo, sendo reexportado como cidadão de primeira classe em `joaju::table`.

## 2. Requisitos Técnicos
1. **Expurgo de Dependências Pesadas em `Cargo.toml`:**
   - Remoção de `typst`, `typst-as-lib`, `typst-pdf` e `typst-layout`.
   - Remoção de dependências fantasmas não utilizadas diretamente no core.
   - Preservação da dependência autônoma `joaju-table = { version = "0.1.0", registry = "gambi" }`.

2. **Linking Estático Direto via `build.rs`:**
   - Criar `build.rs` em `joaju` que busca `libjoaju_pdf.a` localmente em `../joaju-pdf/target/` ou faz download de `https://gambi.2lp.in/api/packages/lumen/generic/joaju-pdf-static/0.1.0/libjoaju_pdf-x86_64-linux.a`.
   - Declarar diretivas `cargo:rustc-link-search` e `cargo:rustc-link-lib=static=joaju_pdf`.
   - Adicionar `cargo:rustc-link-arg=-Wl,--allow-multiple-definition` para tolerar duplicidades de runtime.

3. **Interface de Compilação em `src/compiler/mod.rs`:**
   - Implementar chamada FFI para `joaju_pdf_compile_typst_to_pdf`.
   - Liberar buffers de PDF via `joaju_pdf_free_bytes` e erros via `joaju_pdf_free_string`.

4. **Reexportação Autônoma de Tabelas em `src/lib.rs`:**
   - Adicionar `pub use joaju_table as table;`.

## 3. Critérios de Aceitação
1. `Cargo.toml` não contém `typst-as-lib`, `typst-pdf`, `typst-layout` nem `typst`.
2. Testes automatizados `test_no_direct_typst_dependency.sh` e `test_table_reexport.rs` validam as garantias arquiteturais.
3. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` executam com zero advertências.
4. `cccp check --strict` atinge 100% de conformidade GOST.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #70
- [x] Fase 2 (Branch): `feat/70-desacoplar-motor-typst-para-joaju-pdf-e-consumir-via-link-estatico`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [ ] Fase 4 (Red Test): Testes de ausência de typst e reexportação de tabelas
- [ ] Fase 5 (Green Implement): Implementação de build.rs, FFI e remoção de deps
- [ ] Fase 6 (Refactor): Limpeza de dead code e higienização terminológica
- [ ] Fase 7 (Mutation Test): Testes de mutação e resiliência de FFI
- [ ] Fase 8 (Quality Gate): fmt, clippy sem avisos e testes aprovados
- [ ] Fase 9 (Finish & Close): Conclusão com autorização explícita
