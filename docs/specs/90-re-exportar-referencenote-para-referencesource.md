# Especificação 90: Re-exportar ReferenceNote para ReferenceSource

- **ID do Ticket:** #90
- **Status:** Em Andamento (DD-TDD Fase 3)
- **Autora:** Lumen
- **Alinhamento:** GOST 3.0.0 (Rust)

## 1. Contexto & Motivação
A API pública do `joaju` expõe `ReferenceSource::Memory(&'a HashMap<String, ReferenceNote>)` e `compile_markdown_to_html_with_source`. No entanto, `ReferenceNote` não estava sendo re-exportado na raiz de `joaju` ou em `joaju::compiler::references`, forçando dependência de crate interna ou erro de símbolo privado (`E0603`).

## 2. Decisão Técnica (Para Natasha)
Re-exportar `ReferenceNote` em `joaju::compiler::references`, `joaju::compiler` e `joaju::ReferenceNote` para que os consumidores de `ReferenceSource::Memory` tenham acesso direto ao tipo público sem acoplamento a crates internas.
