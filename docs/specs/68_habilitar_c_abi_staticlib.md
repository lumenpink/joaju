# Especificação 68: Habilitar C-ABI FFI Unificada e Esteira de Pacotes Estáticos

- **ID do Ticket:** #68
- **Status:** Concluído (DD-TDD) (closes #68)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Motivação
A crate `joaju` é o compilador e orquestrador tipográfico principal. Ele agrega a pilha Typst (`typst-as-lib`, `typst-pdf`) e as extensões soberanas (`joaju-pedigree`, `joaju-chem`, `joaju-markdown`, `joaju-templates`, etc.). Devido ao peso e ao tempo de compilação da infraestrutura tipográfica (~15 minutos em compilações cruzadas), aplicações clientes como o editor `mandu` não devem recompilar o `joaju` e suas dependências a cada build de CI.

Para solucionar isso de ponta a ponta:
1. `joaju` expõe uma interface C-ABI pública estável para compilar Markdown/Typst para PDF e renderizar diagramas (pedigree, química).
2. O pipeline de CI do Forgejo compila a matriz multiarch de bibliotecas estáticas (`libjoaju-x86_64-linux.a`, `libjoaju-aarch64-linux.a`, `libjoaju-x86_64-windows.a`) e publica no Forgejo Generic Packages (`https://gambi.2lp.in/api/packages/lumen/generic/joaju-static/<version>/...`).

## 2. Requisitos Técnicos
1. **Interface C-ABI (FFI):**
   - `joaju_compile_markdown_to_pdf(const char* markdown, uint8_t** out_pdf_data, size_t* out_pdf_len, char** out_error) -> int32_t`
   - `joaju_compile_typst_to_pdf(const char* typst_code, uint8_t** out_pdf_data, size_t* out_pdf_len, char** out_error) -> int32_t`
   - `joaju_render_pedigree(const char* raw_code, const char* forced_format, char** out_format, char** out_svg, char** out_error) -> int32_t`
   - `joaju_render_chem(const char* raw_code, const char* forced_format, char** out_format, char** out_svg, char** out_error) -> int32_t`
   - `joaju_free_bytes(uint8_t* ptr, size_t len)`
   - `joaju_free_string(char* ptr)`

2. **Atualização de Dependências Satélites:**
   - Atualizar `joaju-pedigree` para `0.2.2`.
   - Atualizar `joaju-chem` para `0.1.1`.

3. **Esteira de CI Multiarch:**
   - Atualizar `.forgejo/workflows/ci.yml` para compilar e publicar os arquivos `.a` nos pacotes genéricos.

## 3. Critérios de Aceitação
1. Funções C-ABI exportadas e validadas em `tests/ffi_spec.rs`.
2. Símbolos C-ABI presentes no arquivo `.a`.
3. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem avisos.
4. `cccp check --strict` 100% conforme.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #68
- [x] Fase 2 (Branch): `feat/68-habilitar-c-abi-staticlib`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Criar teste red para símbolos C-ABI (`tests/ffi_spec.rs`)
- [x] Fase 5 (Green Implement): Implementar módulo FFI (`src/ffi.rs`) e atualizar satélites
- [x] Fase 6 (Refactor): Limpeza de memória e segurança FFI
- [x] Fase 7 (Mutation Test): Validação de mutação e resiliência em erros
- [x] Fase 8 (Quality Gate): fmt, clippy sem avisos e testes verdes (100% Gosplan)
- [x] Fase 9 (Finish & Close): Concluído (closes #68)
