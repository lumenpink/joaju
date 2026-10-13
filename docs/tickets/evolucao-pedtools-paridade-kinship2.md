# 🧬 Evolução do Motor Pedtools para Paridade com Kinship2

**Status:** Proposto / Backlog  
**Prioridade:** Alta  
**Componente:** `joaju` (Backend R: `plot_pedtools.R`, Container Docker)  

---

## 1. Contexto e Motivação
O pacote `kinship2` do R encontra-se em processo de obsolescência (*phase-out* / manutenção congelada no CRAN), enquanto o ecossistema `pedsuite` / `pedtools` é o padrão moderno e ativamente desenvolvido para genética humana, forense e médica.

Entretanto, o script `plot_pedtools.R` atual do Joaju ainda não oferece os recursos visuais e clínicos avançados que foram implementados e refinados no `plot_pedigree.R` (baseado em `kinship2`):
1. **Legendas Clínicas Detalhadas com Formas Geométricas Reais:**
   - No `plot_pedigree.R`, calculamos legendas vetoriais em polegadas físicas com círculos e quadrados divididos ao meio (heterozigotos), preenchidos sólidos, formas tracejadas/interrogadas (suspeita clínica) e variantes por sexo.
   - O `pedtools` nativo não gera essa legenda clínica padronizada de forma integrada.
2. **Seta Indicadora de Probando (↗ P):**
   - No `kinship2`, calibramos dinamicamente o cálculo de caixas (`boxw`, `boxh`) para que a seta aponte de fora do nó sem encobrir o quadrado/círculo do indivíduo.
   - No `pedtools`, a representação de probando precisa atingir o mesmo padrão visual e posicionamento não invasivo.
3. **Representação Fina de Heterocromia / Múltiplos Afetados / Segmentos:**
   - Suporte a setores de preenchimento e hachuras parciais equivalentes.
4. **Controle Estrito de Proporções e Dimensões:**
   - Garantir parâmetros como `cex`, `symbolsize`, `margins` proporcionais e auto-dimensionamento de SVG para evitar quebra de nós em árvores com múltiplos cônjuges e ramificações complexas.

---

## 2. Requisitos Técnicos
- [ ] Estender `plot_pedtools.R` para interpretar os mesmos parâmetros declarativos aceitos pelo `plot_pedigree.R` (`legend`, `legend.type`, `symbolsize`, `cex`, `mar`, etc.).
- [ ] Implementar motor de legendas clínicas desenhadas via primitivas gráficas do R sobre o canvas do `pedtools` com as mesmas chaves (`sq_empty`, `circ_empty`, `sq_half`, `circ_half`, `sq_solid`, `circ_solid`, `circ_suspect`, `proband`).
- [ ] Calibrar posicionamento da seta de probando (`pedtools::plot(..., proband = ...)` ou via injeção geométrica `arrows()`).
- [ ] Criar bateria de testes comparativos no sandbox (`/tmp` ou `tests/`) confrontando a saída gráfica de uma mesma árvore genealógica complexa renderizada por ambos os motores.

---

## 3. Critérios de Aceite
- Uma árvore genealógica renderizada com ````pedtools``` deve ter paridade visual completa com a renderizada por ````kinship2```, permitindo a transição transparente sem perda de qualidade estética ou diagnóstica.
