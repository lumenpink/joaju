# Especificação Técnica: Fix #92 - Travar Nomes Canônicos Únicos de Blocos e Corrigir HTML

## Contexto e Histórico
Na versão anterior do `joaju`, múltiplos motores de diagrama, química e tabelas suportavam aliases legados (como `pedtools`, `kinship2`, `openchemlib`, `csv`, etc.). Conforme decidido com a usuária para prevenir confusão mental e dívidas técnicas no ecossistema, todos os aliases foram extintos: cada bloco tem exclusivamente um único identificador canônico.

Além disso, para Natasha manter e evoluir o visualizador web/mobile sem surpresas:
1. Blocos de diagrama (como `pedigree` e `chem`) renderizam SVG dentro de `<div class="joaju-diagram-block joaju-<engine>-block" data-engine="<engine>">...</div>`.
2. Blocos que falharem no parse/renderização não quebram silenciosamente e nem vazam stack trace bruto: geram `<div class="joaju-compile-error" data-engine="<engine>">Erro ao compilar <engine>: ...</div>`.
3. Os identificadores canônicos travados no compilador são:
   - `table` (exclusivo para tabelas tabulares / joaju-csv)
   - `pedigree` (exclusivo para heredogramas / joaju-pedigree)
   - `chem` (exclusivo para química e SMILES / joaju-chem)
   - `lirica` (exclusivo para versos / joaju-lirica)
   - `wireframe` (exclusivo para mockups / joaju-diagram wireframe)
   - `sequence` (exclusivo para sequência / joaju-diagram sequence)
   - `graph` (exclusivo para grafos / joaju-diagram graph)
   - `typst` (exclusivo para blocos typst nativos)

## Alternativas Descartadas
- *Manter aliases legados como fallback silencioso:* Descartado porque acumula complexidade em todas as pontas (joaju, mandu, mandu-mobile, mandu-gui, web playground), além de abrir brechas para inconsistências de estilo.
- *Emitir SVG cru sem contêiner:* Descartado porque impossibilita temas escuros de estilizarem fundo contrastante adequado para SVGs com linhas pretas (#000000) sem modificar a biblioteca de desenho vetorial original.

## Validação
- Testes de compilação HTML em `tests/html_compiler_spec.rs` cobrindo blocos `table`, `pedigree` e `chem`.
