const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

test('Conformidade GOST: Arquitetura de Navegação e Usabilidade do Site', async (t) => {
  const rootDir = path.resolve(__dirname, '..');
  const siteDir = path.join(rootDir, 'site');
  const motoresDir = path.join(siteDir, 'motores');

  const rawVersion = fs.readFileSync(path.join(rootDir, 'VERSION'), 'utf8').trim();
  const shortVersion = rawVersion.includes('+') ? rawVersion.split('+')[0] : rawVersion;
  const expectedBadge = `v${shortVersion}</span>`;
  const expectedFooter = `joaju v${rawVersion}`;

  const motores = [
    'pedtools.html',
    'kinship2.html',
    'quimica.html',
    'table.html',
    'mermaid.html',
    'd2.html',
    'plantuml.html',
    'graphviz.html',
    'gist.html'
  ];

  await t.test('site/index.html possui cabeçalho canônico com link e taxonomia completa', () => {
    const html = fs.readFileSync(path.join(siteDir, 'index.html'), 'utf8');
    assert.ok(
      html.includes('<a href="index.html" class="brand">') ||
        html.includes('<a href="./" class="brand">') ||
        html.includes('<a href="#" class="brand">'),
      'Brand na home deve ser link navegável'
    );
    assert.ok(html.includes(expectedBadge), `Brand deve exibir versão ${expectedBadge}`);
    assert.ok(html.includes('href="#motores">Motores'), 'Navbar deve ter link para Motores');
    assert.ok(html.includes('href="#perfis">Perfis'), 'Navbar deve ter link para Perfis');
    assert.ok(
      html.includes('href="#quickstart">Guia Rápido'),
      'Navbar deve ter link para Guia Rápido'
    );
    assert.ok(html.includes('href="#api">API Local'), 'Navbar deve ter link para API Local');
    assert.ok(
      html.includes(expectedFooter),
      `Rodapé deve conter versão completa ${expectedFooter}`
    );
  });

  await t.test('todas as páginas de motores possuem cabeçalho uniforme sem estilos inline', () => {
    for (const file of motores) {
      const html = fs.readFileSync(path.join(motoresDir, file), 'utf8');
      assert.ok(
        html.includes('<a href="../index.html" class="brand">') && html.includes(expectedBadge),
        `${file} deve possuir brand canônica apontando para ../index.html com badge ${expectedBadge}`
      );
      assert.ok(
        !html.includes(
          'style="text-decoration: none; color: inherit; display: flex; align-items: center; gap: 0.5rem;"'
        ),
        `${file} não deve conter estilos inline na brand`
      );
      assert.ok(
        html.includes('href="../index.html#motores">Motores</a>'),
        `${file} deve ter link para Motores na navbar`
      );
      assert.ok(
        html.includes('href="../index.html#perfis">Perfis</a>'),
        `${file} deve ter link para Perfis na navbar`
      );
      assert.ok(
        html.includes(expectedFooter),
        `${file} rodapé deve conter versão completa ${expectedFooter}`
      );
    }
  });

  await t.test(
    'todas as páginas de motores possuem breadcrumb semântico com link para Motores',
    () => {
      for (const file of motores) {
        const html = fs.readFileSync(path.join(motoresDir, file), 'utf8');
        assert.ok(
          html.includes('aria-label="Navegação estrutural"'),
          `${file} deve ter aria-label no breadcrumb`
        );
        assert.ok(
          html.includes('<a href="../index.html">Início</a>'),
          `${file} deve ter link Início no breadcrumb`
        );
        assert.ok(
          html.includes('<a href="../index.html#motores">Motores</a>'),
          `${file} deve ter link Motores no breadcrumb`
        );
      }
    }
  );

  await t.test(
    'todas as páginas de motores possuem subnav rápida e paginação anterior/próximo',
    () => {
      for (const file of motores) {
        const html = fs.readFileSync(path.join(motoresDir, file), 'utf8');
        assert.ok(html.includes('class="subnav-motores"'), `${file} deve conter subnav de motores`);
        assert.ok(
          html.includes('class="motor-pill active"'),
          `${file} deve ter um motor ativo destacado na subnav`
        );
        assert.ok(
          html.includes('class="motor-pagination"'),
          `${file} deve conter paginação sequencial de motores`
        );
      }
    }
  );
});
