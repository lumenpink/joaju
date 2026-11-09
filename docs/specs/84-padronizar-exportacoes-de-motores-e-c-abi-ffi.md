# Spec Ticket #84: Padronizar Exportações de Motores e C-ABI FFI no Joaju

## 1. Contexto & Motivação
O compilador `joaju` integra e orquestra uma rica constelação de motores tipográficos e diagramáticos (`joaju-lirica`, `joaju-wireframe`, `joaju-sequence`, `joaju-graph`, `joaju-chem`, `joaju-pedigree`, `joaju-slides`, `joaju-table`, `joaju-templates`, `joaju-markdown`).
Contudo, a exposição pública da biblioteca em `src/lib.rs` apresentava inconsistências e omissões:
- Motores como `lirica`, `wireframe`, `sequence` e `graph` eram utilizados internamente durante o pré-processamento de PDF, mas não estavam reexportados publicamente na raiz do crate.
- Na camada C-ABI (`src/ffi.rs`), apenas `pedigree` e `chem` possuíam funções FFI dedicadas (`joaju_render_pedigree`, `joaju_render_chem`), impossibilitando que clientes da biblioteca estática `libjoaju.a` (como o `mandu-core`) acionassem os demais motores diretamente.

## 2. Requisitos Técnicos

### 2.1 Reexportações Canônicas em `src/lib.rs`
1. Expor módulos dedicados de motor sob `joaju::`:
   - `pub mod chem { pub use crate::engine::chem::*; }`
   - `pub mod graph { pub use crate::engine::graph::*; }`
   - `pub mod lirica { pub use crate::engine::lirica::*; }`
   - `pub mod pedigree { pub use crate::engine::pedigree::*; }`
   - `pub mod sequence { pub use crate::engine::sequence::*; }`
   - `pub mod slides { pub use crate::engine::slides::*; }`
   - `pub mod wireframe { pub use crate::engine::wireframe::*; }`
2. Reexportar funções de topo para ergonomia direta:
   - `pub use engine::chem::render_chem;`
   - `pub use engine::graph::render_graph;`
   - `pub use engine::lirica::render_lirica;`
   - `pub use engine::pedigree::{render_pedigree, RenderRequest, RenderResponse};`
   - `pub use engine::sequence::render_sequence;`
   - `pub use engine::slides::render_slides;`
   - `pub use engine::wireframe::render_wireframe;`
3. Reexportar satélites de documentos:
   - `pub use joaju_table as table;`
   - `pub use joaju_templates as templates;`
   - `pub use joaju_markdown as markdown;`
4. Atualizar `ecosystem_inventory()` com todas as 11 crates satélites oficiais e suas versões.

### 2.2 Funções C-ABI em `src/ffi.rs`
Implementar exportações C-ABI com convenção padronizada:
- `joaju_render_lirica(code: *const c_char, out_typst: *mut *mut c_char, out_error: *mut *mut c_char) -> i32`
- `joaju_render_wireframe(code: *const c_char, out_typst: *mut *mut c_char, out_error: *mut *mut c_char) -> i32`
- `joaju_render_sequence(code: *const c_char, out_typst: *mut *mut c_char, out_error: *mut *mut c_char) -> i32`
- `joaju_render_graph(code: *const c_char, out_typst: *mut *mut c_char, out_error: *mut *mut c_char) -> i32`

Todas as funções devem seguir o contrato de segurança estrito:
- Validação de ponteiros nulos;
- Conversão segura UTF-8;
- Liberação de memória gerenciada por `joaju_free_string`.

## 3. Critérios de Aceitação
- `cargo fmt -- --check`: Formatação sem violações.
- `cargo clippy --all-targets -- -D warnings`: Zero advertências.
- `cargo test --all-targets`: Todos os testes unitários e de integração aprovados.
- Testes cobrindo as chamadas de FFI e as reexportações públicas de todos os motores.
- `cccp check --strict`: Auditoria Gosplan 100% aprovada.
