# 📊 Fonte Única de Dados (SSOT) via CSV com Projeções e Filtros para Heredogramas e Tabelas

**Status:** Proposto / Backlog  
**Prioridade:** Alta  
**Componente:** `joaju` (Backend Node.js: `server.js`, Pré-processadores `preprocessPedigrees` e `preprocessTables`)  

---

## 1. Contexto e Motivação
Em relatórios e trabalhos de genética médica (como o estudo genealógico familiar), uma linhagem possui dezenas de atributos compartilhados: filiação (`id, pai, mae`), sexo biológico, genótipos de múltiplos sistemas (`fenotipo_abo`, `genotipo_abo`, `olhos_fenotipo`, `olhos_genotipo`), histórico clínico e psiquiátrico (`neuro_diagnostico`, `medicacoes`), etc.

Atualmente, se o usuário mantiver tabelas ou blocos ````kinship2``` separados embutidos no documento Markdown:
- Qualquer novo membro da família ou retificação de data/grau de parentesco exige editar manualmente 3 a 5 blocos diferentes no texto.
- O Joaju já possui a diretiva `%% src=caminho/arquivo.csv` ou URL externa (via `resolveExternalSource`), mas ela simplesmente importa o arquivo CSV **na íntegra** para dentro do bloco.
- Falta um mecanismo declarativo para **filtrar colunas (projeção)** e **filtrar linhas (seleção)**, de modo que um único `familia.csv` alimente:
  1. A Tabela Geral e as Tabelas Específicas do documento (` ```table `);
  2. O heredograma do Sistema ABO;
  3. O heredograma da Cor dos Olhos;
  4. O heredograma do Espectro Neurodivergente.

---

## 2. Proposta de Sintaxe e Diretivas Declarativas

No cabeçalho do bloco de código Markdown (comentários `%%`), introduzir suporte a seleção e mapeamento de campos a partir do CSV fonte:

```markdown
```kinship2
%% src=dados/familia.csv
%% select=id,pai,mae,sex,fenotipo_abo as fenotipo,proband,label_abo as label
%% where=geracao <= 3
%% layout.width=98% cex=0.88 symbolsize=2.85
%% legend=c("Tipo B ou O (M)","Tipo B ou O (F)","Tipo A (M)","Tipo A (F)","Probanda")
%% legend.type=c("sq_empty","circ_empty","sq_solid","circ_solid","proband")
```
```

E para tabelas acadêmicas (` ```table `):
```markdown
```table
%% src=dados/familia.csv
%% title=Tabela 1 – Fenótipos e Genótipos Deduzidos do Sistema ABO
%% note=Fonte: Elaborado pela autora (2026).
%% select=codigo, filiacao, sexo, fenotipo_abo, genotipo_deduzido_abo
%% columns=auto, auto, auto, auto, 1fr
%% align=c, c, c, c, l
```
```

---

## 3. Requisitos Técnicos
1. **Pipeline de Resolução de Dados no `server.js`:**
   - Antes de passar o CSV para o script R (`plot_pedigree.R` / `plot_pedtools.R`) ou para o formatador Typst/HTML (`parseTableData`), interceptar as diretivas `%% select=...` e `%% where=...`.
2. **Projeção de Colunas (`select` / aliases com `as`):**
   - Permitir selecionar apenas as colunas necessárias e renomeá-las (ex: `fenotipo_olhos as fenotipo`, `rotulo_olhos as label`).
3. **Filtro de Registros (`where` básico):**
   - Suporte a filtros simples (ex: igualdade, desigualdade, pertinência a listas ou valores numéricos).
4. **Isolamento e Segurança:**
   - Parsing puramente determinístico em memória (usando parser CSV padrão como `csv-parse` ou regex robusto já existente), sem execução de código arbitrário.

---

## 4. Critérios de Aceite
- Um único arquivo `familia.csv` no repositório/workspace pode ser referenciado por múltiplos blocos de heredogramas e tabelas.
- A adição de um membro ao CSV propaga automaticamente suas ramificações e anotações para todos os gráficos e tabelas dependentes durante a compilação do PDF.
