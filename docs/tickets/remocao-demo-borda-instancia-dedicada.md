# 🌐 Remoção de Referências à Demonstração na Borda e Consolidação da Instância Dedicada

**Ticket:** [#22](https://codeberg.org/lumenpink/joaju/issues/22)  
**Tipo:** `chore(site)`  
**Status:** Aberto / Backlog  
**Prioridade:** Normal  
**Componente:** `joaju` (Vitrine Estática: `site/index.html`, `site/style.css`, Documentação: `README.md`, Testes: `test/site.test.js`)  

---

## 1. Contexto e Motivação

Historicamente, o Joaju possuiu uma tentativa de instância de demonstração pública hospedada no nó de borda (`obitwo` / VPS pública via domínio `joaju.lohn.in`). Entretanto, devido à alta demanda computacional do pipeline analítico (compilação do compilador tipográfico Typst, renderização de heredogramas clínicos em R via `kinship2`/`pedsuite` e geração de SVGs moleculares), a execução na VPS de borda mostrou-se inviável, resultando em lentidão extrema, exaustão de memória e falta de paridade com o ambiente de produção.

A arquitetura de infraestrutura foi reformulada (documentada no Ticket #16 do `baquara-infra`), consolidando:
1. **Descontinuação Completa na Borda:** A stack do Joaju foi purgada do nó `obitwo` e a rota reversa `joaju.lohn.in` foi desativada.
2. **Ambiente Oficial Dedicado no `obizero`:** O serviço operacional oficial do Joaju reside exclusivamente no host dedicado `obizero` com hardware robusto, acessível pela VPN Tailscale através do DNS interno `joaju.lumen.pink` (`100.69.210.39:3000`).
3. **Ambiente de Desenvolvimento Local:** Execução rápida e determinística na máquina de trabalho via `cccp serve --dev` ou `npm run dev` escutando em `http://localhost:3000/`.
4. **Vitrine Pública Estática:** Hospedada na Cloudflare Workers/Pages em `https://joaju.2lp.in/` servindo estritamente a documentação e tutoriais estáticos de `./site`.

Com essa consolidação, todo e qualquer link, botão ou referência a ambientes de demonstração externos na borda deve ser removido do repositório, garantindo conformidade com a realidade operacional e com a norma GOST.

---

## 2. Topologia Canônica dos Ambientes

| Ambiente | Host / Rede | URL / Endpoint | Finalidade |
|:---|:---|:---|:---|
| **Vitrine Pública Estática** | Cloudflare Pages / Workers | `https://joaju.2lp.in/` | Documentação oficial, manuais de sintaxe dos motores e guia rápido. |
| **Ambiente Local** | Estação de trabalho (`localhost`) | `http://localhost:3000/` | Desenvolvimento local com hot-reload (`cccp serve --dev` / `npm run dev`). |
| **Produção Dedicada** | Host dedicado `obizero` (Tailscale) | `http://joaju.lumen.pink` (`100.69.210.39:3000`) | Instância permanente interna para compilação remota de relatórios e artigos. |
| **Borda / VPS (Legado)** | ~~`obitwo` / VPS Pública~~ | ~~`https://joaju.lohn.in`~~ | **Descontinuado / Purgado.** Não deve constar em nenhum documento ou link. |

---

## 3. Diagnóstico e Inventário de Alterações

### 3.1. `site/index.html`
- **Metadados Sociais e Canônicos:** Substituir o domínio incorreto de teste `https://joaju.joaju.local/` pelo domínio oficial canônico de publicação `https://joaju.2lp.in/` nas seguintes tags:
  - `<link rel="canonical" href="https://joaju.2lp.in/">`
  - `<meta property="og:url" content="https://joaju.2lp.in/">`
  - `<meta property="og:image" content="https://joaju.2lp.in/assets/og_preview.png">`
  - `<meta name="twitter:url" content="https://joaju.2lp.in/">`
  - `<meta name="twitter:image" content="https://joaju.2lp.in/assets/og_preview.png">`
- **Seção Quickstart:** Clarificar as instruções de inicialização local com foco no fluxo produtivo padrão (`cccp serve --dev` e testes).

### 3.2. `site/style.css`
- **Limpeza de Classes Órfãs:**
  - Remover bloco de estilo da classe descontinuada `.btn-playground` (linhas 97–107):
    ```css
    .btn-playground {
        background: var(--primary);
        color: #fff !important;
        padding: 0.4rem 0.9rem;
        border-radius: 6px;
        font-weight: 600;
    }
    .btn-playground:hover {
        background: var(--primary-hover);
    }
    ```
  - Ajustar seletor na regra de responsividade (linha 350): substituir `.nav-links a:not(.btn-playground)` por `.nav-links a` para simplificar e remover dependência da classe morta.

### 3.3. `README.md`
- Clarificar a seção de execução e arquitetura de rede, documentando a distinção entre a vitrine estática (`https://joaju.2lp.in/`), o nó dedicado de compilação interna (`http://joaju.lumen.pink`) e o servidor de desenvolvimento local (`http://localhost:3000`).
- Assegurar que nenhuma menção residual a `obitwo` ou `joaju.lohn.in` permaneça.

### 3.4. Suíte de Testes Automatizados (`test/site.test.js`)
- Adicionar asserções automáticas de conformidade:
  - Garantir que a URL canônica em `site/index.html` aponte estritamente para `https://joaju.2lp.in/`.
  - Garantir ausência de ocorrências de `joaju.local`, `lohn.in` e `obitwo` nos arquivos estáticos do site.
  - Garantir ausência da classe legada `.btn-playground` em `site/style.css`.

---

## 4. Critérios de Aceite

1. **Zero Referências Obsoletas:** Nenhuma página estática em `site/` ou seção do `README.md` referencia instâncias ou subdomínios legados de borda.
2. **Canonical Canônico e KokoroSim Válido:** A vitrine estática referencia exclusivamente `https://joaju.2lp.in/` nas metatags canônicas e sociais, preservando as restrições rígidas de caracteres (KokoroSim: título de 50–70 chars, descrição de 120–160 chars).
3. **Folha de Estilos Limpa:** Removidos seletores órfãos de botões de playground sem causar regressão na responsividade da barra de navegação móvel.
4. **Testes Automatizados Aprovados:** A suíte `npm test` valida e aprova a ausência de referências obsoletas e integridade de metadados.
5. **Quality Gate GOST:** Aprovação integral de `biome check .` e `cccp check --strict`.
