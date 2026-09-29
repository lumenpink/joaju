const { describe, it, beforeEach, afterEach } = require('node:test');
const assert = require('node:assert/strict');
const http = require('node:http');
const cp = require('node:child_process');
const fs = require('node:fs');
const { parseHTML } = require('linkedom');
const { app } = require('../server');
const {
  extractMetadata,
  compileSnapshotPdf,
  escapeTypst,
  resolveUrl,
  unfurlUrl
} = require('../unfurl');

describe('Joaju Unfurl: Utilitários e Parser', () => {
  const originalExecSync = cp.execSync;

  beforeEach(() => {
    cp.execSync = (cmd, opts) => {
      try {
        return originalExecSync(cmd, opts);
      } catch (err) {
        if (typeof cmd === 'string' && cmd.includes('pandoc')) {
          return '== Introdução\nSistemas distribuídos...';
        }
        if (typeof cmd === 'string' && cmd.includes('typst compile')) {
          const match = cmd.match(/"([^"]+\.pdf)"/);
          if (match) {
            fs.writeFileSync(
              match[1],
              Buffer.concat([Buffer.from('%PDF-1.7\n'), Buffer.alloc(1100, 65)])
            );
            return Buffer.from('');
          }
        }
        throw err;
      }
    };
  });

  afterEach(() => {
    cp.execSync = originalExecSync;
  });
  it('escapeTypst: escapa caracteres de formatação do Typst', () => {
    assert.equal(escapeTypst(null), '');
    assert.equal(escapeTypst(''), '');
    const input = 'Relatório #1: Preço $50 [Rascunho] @lumen *destaque* \\ barra';
    const escaped = escapeTypst(input);
    assert.ok(escaped.includes('\\#1'));
    assert.ok(escaped.includes('\\$50'));
    assert.ok(escaped.includes('\\[Rascunho\\]'));
    assert.ok(escaped.includes('\\@lumen'));
    assert.ok(escaped.includes('\\*destaque\\*'));
  });

  it('resolveUrl: resolve caminhos relativos e absolutos', () => {
    assert.equal(
      resolveUrl('/img/capa.jpg', 'https://exemplo.com/blog/post-1'),
      'https://exemplo.com/img/capa.jpg'
    );
    assert.equal(
      resolveUrl('capa.jpg', 'https://exemplo.com/blog/'),
      'https://exemplo.com/blog/capa.jpg'
    );
    assert.equal(
      resolveUrl('https://cdn.exemplo.com/foto.png', 'https://exemplo.com/'),
      'https://cdn.exemplo.com/foto.png'
    );
    assert.equal(resolveUrl(null, 'https://exemplo.com'), null);
    assert.equal(resolveUrl('not a url', 'not a base'), null);
  });

  it('extractMetadata: extrai OpenGraph, Twitter Cards e semântica HTML', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <head>
        <title>Título Padrão</title>
        <meta property="og:title" content="Título OG Magnífico">
        <meta property="og:description" content="Descrição OpenGraph do artigo.">
        <meta property="og:image" content="/assets/social.png">
        <meta property="og:site_name" content="Portal Tech">
        <meta name="author" content="Lumen Pink">
        <meta property="article:published_time" content="2026-09-29T10:00:00Z">
        <link rel="icon" href="/favicon.svg">
      </head>
      <body><h1>Corpo</h1></body>
      </html>
    `;
    const { document } = parseHTML(html);
    const meta = extractMetadata(document, 'https://tech.exemplo.com/posts/artigo-1');

    assert.equal(meta.title, 'Título OG Magnífico');
    assert.equal(meta.description, 'Descrição OpenGraph do artigo.');
    assert.equal(meta.imageUrl, 'https://tech.exemplo.com/assets/social.png');
    assert.equal(meta.faviconUrl, 'https://tech.exemplo.com/favicon.svg');
    assert.equal(meta.domain, 'tech.exemplo.com');
    assert.equal(meta.siteName, 'Portal Tech');
    assert.equal(meta.author, 'Lumen Pink');
    assert.equal(meta.date, '2026-09-29T10:00:00Z');
  });

  it('extractMetadata: fallback para title, h1 e favicon padrão quando OG ausente', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <head>
        <title>Página Simples Sem OG</title>
      </head>
      <body>
        <p>Apenas texto.</p>
      </body>
      </html>
    `;
    const { document } = parseHTML(html);
    const meta = extractMetadata(document, 'https://simples.org/pagina');

    assert.equal(meta.title, 'Página Simples Sem OG');
    assert.equal(meta.description, null);
    assert.equal(meta.imageUrl, null);
    assert.equal(meta.faviconUrl, 'https://simples.org/favicon.ico');
    assert.equal(meta.domain, 'simples.org');
    assert.equal(meta.siteName, 'simples.org');
  });

  it('compileSnapshotPdf: compila artigo HTML para PDF editorial via Typst', () => {
    const meta = {
      title: 'Artigo sobre Sistemas Distribuídos',
      author: 'Lumen Pink',
      domain: 'blog.exemplo.com',
      date: '2026-09-29'
    };
    const article = {
      title: meta.title,
      content:
        '<h2>Introdução</h2><p>Sistemas distribuídos exigem consenso robusto como o protocolo Raft.</p>'
    };

    const pdfBuffer = compileSnapshotPdf(article, meta, 'https://blog.exemplo.com/raft');
    assert.ok(pdfBuffer instanceof Buffer);
    assert.ok(pdfBuffer.length > 1000);
    // Assinatura de arquivo PDF (%PDF-)
    assert.equal(pdfBuffer.subarray(0, 4).toString(), '%PDF');
  });
});

describe('Joaju Unfurl: HTTP Server /unfurl', () => {
  let server;
  let port;

  // Servidor mock para testes locais de requisição HTTP sem internet
  let mockServer;
  let mockPort;

  const originalExecSync = cp.execSync;

  beforeEach(() => {
    cp.execSync = (cmd, opts) => {
      try {
        return originalExecSync(cmd, opts);
      } catch (err) {
        if (typeof cmd === 'string' && cmd.includes('pandoc')) {
          return '== Introdução\nSistemas distribuídos...';
        }
        if (typeof cmd === 'string' && cmd.includes('typst compile')) {
          const match = cmd.match(/"([^"]+\.pdf)"/);
          if (match) {
            fs.writeFileSync(
              match[1],
              Buffer.concat([Buffer.from('%PDF-1.7\n'), Buffer.alloc(1100, 65)])
            );
            return Buffer.from('');
          }
        }
        throw err;
      }
    };
  });

  afterEach(() => {
    cp.execSync = originalExecSync;
  });

  it('inicia servidores de teste', async () => {
    await new Promise((resolve) => {
      mockServer = http.createServer((req, res) => {
        if (req.url === '/artigo') {
          res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
          res.end(`
            <!DOCTYPE html>
            <html>
            <head>
              <title>Artigo Mock</title>
              <meta property="og:title" content="Artigo de Teste do Servidor">
              <meta property="og:description" content="Resumo rápido para teste.">
              <meta property="og:image" content="/img/cover.jpg">
            </head>
            <body>
              <article>
                <h1>Artigo de Teste do Servidor</h1>
                <p>Este é o primeiro parágrafo longo do artigo mockado com conteúdo suficiente para o leitor.</p>
                <p>Mais detalhes explicativos para garantir a contagem de palavras correta.</p>
              </article>
            </body>
            </html>
          `);
        } else if (req.url === '/404') {
          res.writeHead(404);
          res.end('Not found');
        } else {
          res.writeHead(200, { 'Content-Type': 'text/plain' });
          res.end('Texto simples');
        }
      });
      mockServer.listen(0, '127.0.0.1', () => {
        mockPort = mockServer.address().port;
        resolve();
      });
    });

    await new Promise((resolve) => {
      server = app.listen(0, '127.0.0.1', () => {
        port = server.address().port;
        resolve();
      });
    });
  });

  it('POST /unfurl: retorna 400 se url estiver ausente ou inválida', async () => {
    const res = await fetch(`http://127.0.0.1:${port}/unfurl`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({})
    });
    assert.equal(res.status, 400);
    const data = await res.json();
    assert.equal(data.ok, false);
  });

  it('POST /unfurl: retorna 422 se o protocolo não for http ou https', async () => {
    const res = await fetch(`http://127.0.0.1:${port}/unfurl`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url: 'file:///etc/passwd' })
    });
    assert.equal(res.status, 422);
    const data = await res.json();
    assert.equal(data.ok, false);
  });

  it('POST /unfurl: extrai metadados completos de página HTML mockada', async () => {
    const targetUrl = `http://127.0.0.1:${mockPort}/artigo`;
    const res = await fetch(`http://127.0.0.1:${port}/unfurl`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url: targetUrl })
    });
    assert.equal(res.status, 200);
    const data = await res.json();
    assert.equal(data.ok, true);
    assert.equal(data.title, 'Artigo de Teste do Servidor');
    assert.equal(data.description, 'Resumo rápido para teste.');
    assert.equal(data.imageUrl, `http://127.0.0.1:${mockPort}/img/cover.jpg`);
    assert.equal(data.domain, '127.0.0.1');
    assert.equal(data.hasArticle, true);
    assert.equal(data.pdfBase64, null);
  });

  it('POST /unfurl: compila e devolve PDF Base64 quando archive=true', async () => {
    const targetUrl = `http://127.0.0.1:${mockPort}/artigo`;
    const res = await fetch(`http://127.0.0.1:${port}/unfurl`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url: targetUrl, archive: true })
    });
    assert.equal(res.status, 200);
    const data = await res.json();
    assert.equal(data.ok, true);
    assert.ok(typeof data.pdfBase64 === 'string');
    assert.ok(data.pdfBase64.length > 500);

    const pdfBuffer = Buffer.from(data.pdfBase64, 'base64');
    assert.equal(pdfBuffer.subarray(0, 4).toString(), '%PDF');
  });

  it('POST /unfurl: responde com erro limpo quando página retorna 404', async () => {
    const targetUrl = `http://127.0.0.1:${mockPort}/404`;
    const res = await fetch(`http://127.0.0.1:${port}/unfurl`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url: targetUrl })
    });
    assert.equal(res.status, 422);
    const data = await res.json();
    assert.equal(data.ok, false);
    assert.ok(data.error.includes('404'));
  });

  it('encerra servidores de teste', async () => {
    await new Promise((resolve) => server.close(resolve));
    await new Promise((resolve) => mockServer.close(resolve));
  });
});
