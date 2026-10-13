# 🧬 Paridade de Renderização Visual de Diagramas entre Obsidian e PDF (Joaju)

**Status:** Backlog  
**Contexto:** Identificou-se discrepância visual na renderização dos diagramas (notadamente heredogramas com `kinship2` / `pedtools`) entre a pré-visualização ao vivo (*Live Preview*) no plugin `joaju-obsidian` e o documento final compilado em PDF via Typst. O design do ecossistema pressupõe paridade estrita ("o que você vê na nota é o que você obtém no PDF").

---

## Diagnóstico e Causas Raiz

### 1. Invalidação de Cache de Diagramas
O plugin Obsidian (`joaju-obsidian`) implementa um cache em memória/disco baseado no hash do código do bloco (`createDiagramCache`). Alterações nos scripts de compilação ou rotas de renderização no backend do container Joaju não invalidavam o hash do bloco, mantendo SVGs antigos exibidos na interface do usuário enquanto a compilação do PDF consumia o renderizador atualizado.

### 2. Estilos e Responsividade do DOM vs. Typst
- **No Obsidian:** O SVG é injetado diretamente no container HTML (`.joaju-diagram-container`). O CSS global do Obsidian e temas da comunidade aplicam regras de `max-width`, display flex/block e dimensionamento de fontes que podem deformar o *aspect ratio* caso o SVG não contenha `viewBox` estrito e `preserveAspectRatio="xMidYMid meet"`.
- **No Typst (PDF):** O SVG é referenciado diretamente via `#image(...)` dentro de `#block(...)`, onde o motor Typst calcula as dimensões baseando-se estritamente nas dimensões físicas (`width` e `height` do SVG em polegadas/pontos) e escala vetorialmente sem interferência de CSS de página.

### 3. Escala Física vs. Coordenadas de Usuário
No driver Cairo SVG e no R (`kinship2`), a proporção entre eixos X e Y em unidades de usuário (`usr`) não é 1:1 (`scale_x != scale_y`). Símbolos desenhados usando coordenadas puras de usuário sofriam distorção de aspecto no navegador/DOM se o container não preservasse o aspecto físico.

---

## Plano de Ação e Soluções Propostas

### A. Versão do Motor no Hash do Cache
Incorporar a versão do servidor ou um hash dos geradores de imagem (`engine_version` / `hash(plot_pedigree.R)`) à chave do cache de diagramas do plugin Obsidian.
- **Resultado:** Qualquer atualização no pipeline do Joaju força a invalidação automática de cache no Obsidian.

### B. Normalização Estrita do SVG no Backend
Garantir que todos os SVGs gerados por scripts R (`CairoSVG` / `svg()`) passem por sanitização pós-processamento:
1. Injetar ou validar atributos `viewBox="0 0 W H"`.
2. Adicionar explicitamente `preserveAspectRatio="xMidYMid meet"`.
3. Fixar `width="100%"` e `height="auto"` quando embutidos no DOM, garantindo que o desenho se adapte sem esticar ou achatar círculos e quadrados.

### C. Isolamento CSS no Plugin Obsidian (`styles.css`)
No plugin `joaju-obsidian`, aplicar regras estritas de isolamento para `.joaju-diagram-container svg`:
```css
.joaju-diagram-container svg {
  display: block;
  margin: 0 auto;
  max-width: 100%;
  height: auto;
  object-fit: contain;
}
```

### D. Testes Automatizados de Regressão Visual
Criar testes E2E / snapshots comparativos que verifiquem se as proporções e coordenadas relativas entre os nós do heredograma no SVG e no PDF renderizado pelo Typst permanecem matematicamente idênticas.

---

## Referências
- `joaju/plot_pedigree.R` e `joaju/plot_pedtools.R`
- `joaju-obsidian/main.js` (`createDiagramCache`, `renderSvgToElement`, `compileMarkdownToPdf`)
- Bennett et al. (2008) – Standardized Human Pedigree Nomenclature
