# 🌐 Cards para protocolos não-HTTP (gemini://, gopher://)

**Status:** Backlog  
**Contexto:** Adicionado suporte à sintaxe `!gemini://url` e `!gopher://url` para renderizar cards estáticos (sem preview), mas o backend `/unfurl` não sabe buscar esses protocolos. Esta issue acompanha o trabalho futuro para suporte real.

---

## Problema

O backend `joaju/unfurl.js` usa `fetch()` HTTP para buscar a página e extrair metadados. Isso funciona para `http://` e `https://`, mas falha completamente para:

- `gemini://` — protocolo binário sobre TLS (RFC draft), sem suporte em `fetch()`
- `gopher://` — protocolo pré-web, sem suporte nativo em Node.js

Hoje a solução é **não chamar `/unfurl`** para esses protocolos e renderizar um card mínimo (só domínio + URL).

---

## Opções a investigar

### A. Proxy Gemini → HTTP via serviço público
Serviços como [portal.mozz.us](https://portal.mozz.us) ou [tlgs.one](https://tlgs.one) expõem cápsulas Gemini via HTTPS. Poderíamos:
1. Detectar `gemini://host/path`
2. Converter para `https://portal.mozz.us/gemini/host/path`
3. Fazer fetch HTTP normal + Readability

**Prós:** Implementação simples, sem deps extras  
**Contras:** Dependência de serviço externo; uptime não garantido

### B. Cliente Gemini nativo em Node.js
Implementar o handshake TLS manualmente (protocolo é simples: TLS + `URL\r\n` → resposta com header de status + body em text/gemini). Ou usar lib como [`gemini-fetch`](https://github.com/LGUG2Z/gemini-fetch).

**Prós:** Sem dependência externa; fetch real do conteúdo  
**Contras:** Gemini usa certs autoassinados (TOFU), complicando TLS padrão do Node

### C. Cliente Gopher em Node.js
Gopher é TCP simples (sem TLS): conecta na porta 70, manda `/caminho\r\n`, recebe resposta. Implementação própria ~50 linhas.

**Prós:** Protocolo trivial  
**Contras:** Gopher não tem metadados — card seria só domínio + seletor

### D. Manter card mínimo sem preview (estado atual)
Card com domínio, URL completa, sem título/descrição/imagem. Estado de hoje.

---

## Notas de UX

- Links `gemini://` e `gopher://` não abrem no browser. O botão ↗ do card deveria **copiar a URL para o clipboard** em vez de `window.open()`, ou tentar o scheme handler do sistema (Lagrange, Kristall, etc. podem estar registrados).
- Considerar ícone diferente: 🪐 para Gemini, 🦦 para Gopher?
- `text/gemini` (gemtext) não é entendido pelo Readability — precisaria de um parser básico de gemtext para extrair o título (primeira linha `# `).

---

## Links relevantes

- [Gemini Protocol Spec](https://gemini.circumlunar.space/docs/specification.gmi)
- [Gopher RFC 1436](https://www.rfc-editor.org/rfc/rfc1436)
- [portal.mozz.us proxy](https://portal.mozz.us)
- [Awesome Gemini](https://github.com/kr1sp1n/awesome-gemini)
