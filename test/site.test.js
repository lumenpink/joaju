const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

test('Conformidade GOST: Vitrine Pública e Documentação (./site)', async (t) => {
  const rootDir = path.resolve(__dirname, '..');
  const siteDir = path.join(rootDir, 'site');

  await t.test('diretório ./site e arquivos básicos existem', () => {
    assert.ok(fs.existsSync(siteDir), 'Diretório ./site deve existir');
    assert.ok(fs.existsSync(path.join(siteDir, 'index.html')), 'site/index.html deve existir');
    assert.ok(fs.existsSync(path.join(siteDir, 'style.css')), 'site/style.css deve existir');
    assert.ok(fs.existsSync(path.join(siteDir, 'app.js')), 'site/app.js deve existir');
    assert.ok(fs.existsSync(path.join(siteDir, '_headers')), 'site/_headers deve existir');
    assert.ok(fs.existsSync(path.join(rootDir, 'wrangler.jsonc')), 'wrangler.jsonc deve existir');
  });

  await t.test('site/_headers contém os 3 cabeçalhos de segurança obrigatórios', () => {
    const headersContent = fs.readFileSync(path.join(siteDir, '_headers'), 'utf8');
    assert.ok(
      headersContent.includes('X-Content-Type-Options: nosniff'),
      'Header nosniff presente'
    );
    assert.ok(headersContent.includes('X-Frame-Options: DENY'), 'Header DENY presente');
    assert.ok(
      headersContent.includes('Referrer-Policy: strict-origin-when-cross-origin'),
      'Header Referrer-Policy presente'
    );
  });

  await t.test('wrangler.jsonc declara a rota joaju.2lp.in e assets ./site', () => {
    const wranglerContent = fs.readFileSync(path.join(rootDir, 'wrangler.jsonc'), 'utf8');
    assert.ok(wranglerContent.includes('joaju.2lp.in'), 'Rota joaju.2lp.in declarada');
    assert.ok(wranglerContent.includes('./site'), 'Diretório ./site mapeado como assets');
  });

  await t.test('banner social og_preview.png tem dimensões exatas de 1200x630 pixels', () => {
    const imgPath = path.join(siteDir, 'assets', 'og_preview.png');
    assert.ok(fs.existsSync(imgPath), 'Arquivo site/assets/og_preview.png deve existir');

    const buffer = fs.readFileSync(imgPath);
    // Validar cabeçalho PNG e dimensões no chunk IHDR
    const pngSignature = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
    assert.ok(buffer.subarray(0, 8).equals(pngSignature), 'Deve ser um arquivo PNG válido');

    const width = buffer.readUInt32BE(16);
    const height = buffer.readUInt32BE(20);
    assert.equal(width, 1200, 'Largura do banner deve ser exatamente 1200px');
    assert.equal(height, 630, 'Altura do banner deve ser exatamente 630px');
  });

  await t.test('meta tags KokoroSim em index.html respeitam limites rígidos de caracteres', () => {
    const html = fs.readFileSync(path.join(siteDir, 'index.html'), 'utf8');

    // og:title (50 a 65 caracteres, limite máximo 70)
    const titleMatch = html.match(/<meta\s+property="og:title"\s+content="([^"]+)"/);
    assert.ok(titleMatch, 'og:title deve estar presente');
    const title = titleMatch[1];
    assert.ok(
      title.length >= 50 && title.length <= 70,
      `og:title deve ter entre 50 e 70 chars (atual: ${title.length})`
    );

    // og:description (120 a 155 caracteres, limite máximo 160)
    const descMatch = html.match(/<meta\s+property="og:description"\s+content="([^"]+)"/);
    assert.ok(descMatch, 'og:description deve estar presente');
    const desc = descMatch[1];
    assert.ok(
      desc.length >= 120 && desc.length <= 160,
      `og:description deve ter entre 120 e 160 chars (atual: ${desc.length})`
    );
  });
});
