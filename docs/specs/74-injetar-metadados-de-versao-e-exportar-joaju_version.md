# Spec 74: Injetar Metadados de Versão e Exportar joaju_version

## 1. Contexto e Motivação
Para que os consumidores de C-ABI (como o CLI `mandu`, `hendu` ou interfaces em C/Python/Node.js) e outros módulos possam auditar a versão em tempo de execução sem parsing de arquivos no disco, a biblioteca canônica `joaju` deve expor seu SemVer oficial via FFI C-ABI estático com zero alocação de memória e zero overhead de sincronização de runtime.

Adicionalmente, deve fornecer funções de agregação C-ABI que permitam inspecionar os símbolos C-ABI vinculados dos subcomponentes:
- `joaju_submodule_pdf_version()` -> consulta o símbolo vinculado `joaju_pdf_version()`.
- `joaju_submodule_pedigree_version()` -> consulta `joaju_pedigree::joaju_pedigree_version()`.
- `joaju_submodule_chem_version()` -> consulta `joaju_chem::joaju_chem_version()`.

No lado Rust, deve expor a constante `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` em `src/lib.rs`.

## 2. Requisitos Técnicos
- Declarar `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` em `src/lib.rs`.
- Declarar e exportar `#[no_mangle] pub extern "C" fn joaju_version() -> *const c_char` em `src/ffi.rs`.
- Declarar e exportar as funções de agregação FFI para os módulos vinculados em `src/ffi.rs`.
- Garantir segurança contra vazamentos de memória (retornar ponteiro estático nulo-terminado).

## 3. Plano de Testes (DD-TDD)
- Fase 4: Criar `tests/abi_version_spec.rs` com testes comprovadamente falhos cobrindo `VERSION` e `joaju_version()`.
- Fase 5: Implementação funcional.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict`.

## 4. Status de Conclusão
- [x] `pub const VERSION: &str` exportado em `src/lib.rs`.
- [x] `joaju_version()` exportado como C-ABI FFI estático em `src/ffi.rs`.
- [x] Funções C-ABI FFI para inspeção de submódulos (`joaju_submodule_pdf_version()`) exportadas em `src/ffi.rs`.
- [x] Suíte de testes `tests/abi_version_spec.rs` 100% verde.
- [x] 100% de conformidade Gosplan (`cccp check --strict`).
