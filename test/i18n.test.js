const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const { app } = require('../server.js');

const CANONICAL_LANGS = ['en', 'pt', 'es', 'fr', 'de', 'it', 'ru', 'zh', 'ja', 'ar'];

test('Conformidade GOST: Matriz Canônica das 10 Línguas Oficiais (Seção 10)', async (t) => {
  const localesDir = path.resolve(__dirname, '..', 'locales');

  await t.test('todos os 10 arquivos de idioma existem no diretório locales/', () => {
    assert.ok(fs.existsSync(localesDir), 'Diretório locales/ deve existir');
    for (const lang of CANONICAL_LANGS) {
      const filePath = path.join(localesDir, `${lang}.json`);
      assert.ok(fs.existsSync(filePath), `Arquivo locales/${lang}.json deve existir`);
    }
  });

  await t.test('100% de paridade de chaves em todos os 10 catálogos de tradução', () => {
    const ptContent = JSON.parse(fs.readFileSync(path.join(localesDir, 'pt.json'), 'utf8'));
    const baselineKeys = Object.keys(ptContent).sort();

    for (const lang of CANONICAL_LANGS) {
      const langContent = JSON.parse(
        fs.readFileSync(path.join(localesDir, `${lang}.json`), 'utf8')
      );
      const langKeys = Object.keys(langContent).sort();
      assert.deepEqual(
        langKeys,
        baselineKeys,
        `Chaves de ${lang}.json devem coincidir 100% com pt.json`
      );
      assert.equal(langContent.lang_code, lang, `lang_code de ${lang}.json deve ser ${lang}`);
    }
  });

  await t.test('árabe (ar.json) declara direction: "rtl" e demais declaram "ltr"', () => {
    for (const lang of CANONICAL_LANGS) {
      const content = JSON.parse(fs.readFileSync(path.join(localesDir, `${lang}.json`), 'utf8'));
      if (lang === 'ar') {
        assert.equal(content.direction, 'rtl', 'Árabe deve ser RTL');
      } else {
        assert.equal(content.direction, 'ltr', `${lang} deve ser LTR`);
      }
    }
  });

  await t.test('seletor de idiomas em public/index.html contém as 10 línguas oficiais', () => {
    const html = fs.readFileSync(path.resolve(__dirname, '..', 'public', 'index.html'), 'utf8');
    assert.ok(html.includes('id="lang-select"'), 'Seletor #lang-select deve existir');
    for (const lang of CANONICAL_LANGS) {
      assert.ok(html.includes(`value="${lang}"`), `Opção com value="${lang}" deve estar no HTML`);
    }
  });

  await t.test('servidor Express entrega catálogos estáticos em /locales/:lang.json', async () => {
    const server = http.createServer(app);
    await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
    const port = server.address().port;
    const baseUrl = `http://127.0.0.1:${port}`;

    try {
      for (const lang of ['pt', 'en', 'ar']) {
        const res = await fetch(`${baseUrl}/locales/${lang}.json`);
        assert.equal(res.status, 200, `/locales/${lang}.json deve responder 200`);
        assert.ok(res.headers.get('content-type').includes('application/json'));
        const data = await res.json();
        assert.equal(data.lang_code, lang);
      }
    } finally {
      await new Promise((resolve) => server.close(resolve));
    }
  });
});
