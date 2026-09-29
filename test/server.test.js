const test = require('node:test');
const assert = require('node:assert/strict');
const http = require('node:http');
const fs = require('node:fs');
const cp = require('node:child_process');

const {
  app,
  preprocessPedigrees,
  preprocessDiagrams,
  preprocessChemistry,
  preprocessTables,
  resolveExternalSource,
  parseTableData,
  renderSingleTable,
  renderSingleOpenchemlib,
  renderSingleOpenbabel,
  renderSinglePedigree,
  renderSingleKinship,
  renderSinglePedtools,
  renderSingleMermaid
} = require('../server.js');

const nativeFetch = globalThis.fetch;
function mockKroki(handler) {
  globalThis.fetch = async (url, opts) => {
    if (String(url).includes('kroki') || String(url).includes(':8000')) {
      return handler(url, opts);
    }
    return nativeFetch(url, opts);
  };
}

function restoreFetch() {
  globalThis.fetch = nativeFetch;
}

test('Preprocessador de Heredogramas Clínicos', async (t) => {
  const originalExecSync = cp.execSync;

  t.afterEach(() => {
    cp.execSync = originalExecSync;
  });

  await t.test('retorna markdown inalterado se não houver blocos de heredograma', async () => {
    const input = '# Título\n\nTexto descritivo simples.';
    const result = await preprocessPedigrees(input, 'test-session-1');
    assert.equal(result, input);
  });

  await t.test(
    'parseia diretivas %% de dimensões, parâmetros do R e injeta layout Typst com sucesso',
    async () => {
      cp.execSync = (cmd) => {
        const match = cmd.match(/"(\/tmp\/ped_[^"]+\.svg)"/);
        if (match) {
          fs.writeFileSync(
            match[1],
            '<svg viewBox="0 0 100 100"><circle cx="50" cy="50" r="40"/></svg>'
          );
        }
      };

      const input = `
# Artigo
\`\`\`heredograma
%% layout.width=85% layout.max_width=500pt cex=0.75 symbolsize=1.1 align=c(1,2)
id,pai,mae,sex,fenotipo
1,0,0,M,wt
2,0,0,F,wt
3,1,2,F,hom
\`\`\`
Texto final.`;

      const result = await preprocessPedigrees(input, 'test-session-2');
      assert.ok(result.includes('```{=typst}'));
      assert.ok(result.includes('#align(center)'));
      assert.ok(result.includes('min(85%,500pt)'));
      assert.ok(result.includes('#image('));
    }
  );

  await t.test(
    'processa diretivas alternativas (width e max-width genéricas) e múltiplos blocos',
    async () => {
      cp.execSync = (cmd) => {
        const match = cmd.match(/"(\/tmp\/ped_[^"]+\.svg)"/);
        if (match) {
          fs.writeFileSync(match[1], '<svg><rect/></svg>');
        }
      };

      const input = `
\`\`\`pedigree
%% width=70% max-width=400pt packed=TRUE
id,sex
1,M
\`\`\`
Meio
\`\`\`heredograma
%% layout.width=90%
id,sex
2,F
\`\`\``;

      const result = await preprocessPedigrees(input, 'test-multi');
      assert.ok(result.includes('min(70%,400pt)'));
      assert.ok(result.includes('min(90%,500pt)'));
    }
  );

  await t.test(
    'trata erro de execução do Rscript graciosamente sem quebrar a execução',
    async () => {
      cp.execSync = () => {
        throw new Error('Falha simulada na execução do Rscript');
      };

      const input = '```heredograma\nid,sex\n1,M\n```';
      const result = await preprocessPedigrees(input, 'test-err');
      assert.ok(typeof result === 'string');
    }
  );

  await t.test('resolve fonte externa remota %% src=http... com sucesso', async () => {
    const rawBlock = '%% layout.width=80%\n%% src="https://exemplo.com/familia.csv"';
    const mockFetch = async () => new Response('id,pai,mae,sex\n1,0,0,M\n2,0,0,F', { status: 200 });

    const resolved = await resolveExternalSource(rawBlock, mockFetch);
    assert.ok(resolved.includes('%% layout.width=80%'));
    assert.ok(resolved.includes('id,pai,mae,sex'));
    assert.ok(!resolved.includes('src='));
  });
});

test('Preprocessador de Diagramas Mermaid via Kroki', async (t) => {
  t.afterEach(() => {
    restoreFetch();
  });

  await t.test('retorna markdown inalterado quando não há diagramas mermaid', async () => {
    const input = '# Sem diagramas aqui';
    const result = await preprocessDiagrams(input, 'test-diag-1');
    assert.equal(result, input);
  });

  await t.test(
    'processa diretivas %% width/max-width e injeta bloco Typst quando Kroki responde 200 OK',
    async () => {
      mockKroki(async () => {
        return new Response(Buffer.from('fake-png-binary-content'), {
          status: 200,
          headers: { 'Content-Type': 'image/png' }
        });
      });

      const input = `
\`\`\`mermaid
%% width=80% max-width=600pt
graph TD
    A --> B
\`\`\``;

      const result = await preprocessDiagrams(input, 'test-kroki-ok');
      assert.ok(result.includes('```{=typst}'));
      assert.ok(result.includes('#align(center)'));
      assert.ok(result.includes('min(80%,600pt)'));
      assert.ok(result.includes('#image('));
    }
  );

  await t.test('trata erro HTTP ou falha de conexão com Kroki graciosamente', async () => {
    mockKroki(async () => {
      throw new Error('Kroki Gateway offline');
    });

    const input = '```mermaid\ngraph TD\nA-->B\n```';
    const result = await preprocessDiagrams(input, 'test-kroki-err');
    assert.ok(typeof result === 'string');
  });
});

test('Motores Isolados: renderSinglePedigree e renderSingleMermaid', async (t) => {
  const originalExecSync = cp.execSync;

  t.afterEach(() => {
    cp.execSync = originalExecSync;
    restoreFetch();
  });

  await t.test(
    'renderSinglePedigree: gera SVG com sucesso e executa cleanup de arquivos',
    async () => {
      let capturedCsv = '';
      cp.execSync = (cmd) => {
        const csvMatch = cmd.match(/"(\/tmp\/ped_[^"]+\.csv)"/);
        const svgMatch = cmd.match(/"(\/tmp\/ped_[^"]+\.svg)"/);
        if (csvMatch) capturedCsv = fs.readFileSync(csvMatch[1], 'utf8');
        if (svgMatch) fs.writeFileSync(svgMatch[1], '<svg class="pedigree"></svg>');
      };

      const content = '%% layout.width=90% cex=0.8\nid,pai,mae,sex\n1,0,0,1';
      const svg = renderSinglePedigree(content, 'single-test-ok');
      assert.equal(svg, '<svg class="pedigree"></svg>');
      assert.ok(capturedCsv.includes('id,pai,mae,sex'));
      assert.ok(!capturedCsv.includes('%%'));
    }
  );

  await t.test('renderSinglePedigree: lança exceção se arquivo SVG não for gerado', async () => {
    cp.execSync = () => {
      /* não cria o SVG */
    };
    assert.throws(() => {
      renderSinglePedigree('id,sex\n1,1', 'single-test-nosvg');
    }, /Arquivo SVG não gerado/);
  });

  await t.test(
    'renderSinglePedigree: limpa arquivos temporários no bloco finally mesmo com erro',
    async () => {
      cp.execSync = () => {
        throw new Error('Erro forçado no script R');
      };
      assert.throws(() => {
        renderSinglePedigree('id,sex\n1,1', 'single-cleanup-test');
      }, /Erro forçado no script R/);
      assert.equal(fs.existsSync('/tmp/ped_single-cleanup-test.csv'), false);
    }
  );

  await t.test(
    'renderSinglePedtools: gera SVG com sucesso e executa cleanup de arquivos',
    async () => {
      let capturedCsv = '';
      cp.execSync = (cmd) => {
        const csvMatch = cmd.match(/"(\/tmp\/pedtools_[^"]+\.csv)"/);
        const svgMatch = cmd.match(/"(\/tmp\/pedtools_[^"]+\.svg)"/);
        if (csvMatch) capturedCsv = fs.readFileSync(csvMatch[1], 'utf8');
        if (svgMatch) fs.writeFileSync(svgMatch[1], '<svg class="pedtools"></svg>');
      };

      const content = '%% layout.width=90% cex=0.8\nid,pai,mae,sex\n1,0,0,1';
      const svg = renderSinglePedtools(content, 'single-pedtools-ok');
      assert.equal(svg, '<svg class="pedtools"></svg>');
      assert.ok(capturedCsv.includes('id,pai,mae,sex'));
      assert.ok(!capturedCsv.includes('%%'));
      assert.equal(fs.existsSync('/tmp/pedtools_single-pedtools-ok.csv'), false);
    }
  );

  await t.test('renderSingleMermaid: remove diretivas %% e retorna SVG com sucesso', async () => {
    let sentBody = '';
    mockKroki(async (url, opts) => {
      sentBody = opts.body;
      return new Response('<svg class="mermaid"></svg>', { status: 200 });
    });

    const content = '%% width=90%\ngraph TD\nA-->B';
    const svg = await renderSingleMermaid(content);
    assert.equal(svg, '<svg class="mermaid"></svg>');
    assert.ok(!sentBody.includes('%%'));
    assert.ok(sentBody.includes('graph TD'));
  });

  await t.test(
    'renderSingleMermaid: propaga erro do Kroki quando resposta não for 200 OK',
    async () => {
      mockKroki(async () => {
        return new Response('Diagram syntax error on line 2', { status: 400 });
      });

      await assert.rejects(async () => {
        await renderSingleMermaid('invalid mermaid code');
      }, /Erro do Kroki \(400\): Diagram syntax error/);
    }
  );

  await t.test('renderSingleOpenchemlib: renderiza molécula SMILES válida em SVG', async () => {
    const smiles = 'CC(=O)Oc1ccccc1C(=O)O'; // Aspirina
    const svg = renderSingleOpenchemlib(smiles);
    assert.ok(svg.includes('<svg'));
    assert.ok(svg.includes('mol'));
  });

  await t.test('renderSingleOpenchemlib: rejeita SMILES vazio com 400', async () => {
    assert.throws(
      () => renderSingleOpenchemlib('   '),
      (err) => err.status === 400
    );
  });

  await t.test('renderSingleOpenchemlib: rejeita SMILES com erro de sintaxe com 422', async () => {
    assert.throws(
      () => renderSingleOpenchemlib('invalid_smiles_@@@'),
      (err) => err.status === 422
    );
  });

  await t.test('renderSingleOpenbabel: renderiza molécula com obabel em SVG', async () => {
    cp.execSync = (cmd) => {
      const match = cmd.match(/"(\/tmp\/(?:ob|chem)_[^"]+\.svg)"/);
      if (match) fs.writeFileSync(match[1], '<svg class="openbabel"></svg>');
    };
    const smiles = 'CC(=O)O';
    const svg = renderSingleOpenbabel(smiles, 'test-obabel');
    assert.ok(svg.includes('<svg'));
  });

  await t.test('renderSingleTable: gera tabela acadêmica HTML com 3 linhas', async () => {
    const input = `
%% title="Tabela 1 - Teste"
%% note="Fonte: Pesquisa 2026"
%% align=l,c,r
Item, Qtd, Valor
Alfa, 10, R$ 100
Beta, 20, R$ 200
`;
    const html = renderSingleTable(input);
    assert.ok(html.includes('class="joaju-academic-table-wrapper"'));
    assert.ok(html.includes('class="joaju-academic-table"'));
    assert.ok(html.includes('Tabela 1 - Teste'));
    assert.ok(html.includes('Fonte: Pesquisa 2026'));
    assert.ok(html.includes('text-align: right;'));
  });

  await t.test('preprocessChemistry: injeta bloco Typst com SVG gerado', async () => {
    const md = '```openchemlib\nCC(=O)O\n```';
    const result = await preprocessChemistry(md, 'test-chem');
    assert.ok(result.includes('```{=typst}'));
    assert.ok(result.includes('#image('));
  });

  await t.test(
    'preprocessTables: injeta tabela Typst nativa com figura e regras Booktabs',
    async () => {
      const md = `
\`\`\`table
%% title="Tabela Teste Typst"
%% note="Fonte do autor"
ColA, ColB
1, 2
\`\`\`
`;
      const result = await preprocessTables(md, 'test-tab');
      assert.ok(result.includes('```{=typst}'));
      assert.ok(result.includes('#figure('));
      assert.ok(result.includes('caption: [Tabela Teste Typst]'));
      assert.ok(result.includes('stroke: (x, y) => if y == 0'));
      assert.ok(result.includes('table.header([*ColA*], [*ColB*])'));
      assert.ok(result.includes('[1], [2]'));
      assert.ok(result.includes('Fonte do autor'));
    }
  );
});

test('Servidor HTTP: Rotas /render, /compile e Arquivos Estáticos', async (t) => {
  let server;
  let baseUrl;
  const originalExecSync = cp.execSync;

  await new Promise((resolve) => {
    server = http.createServer(app);
    server.listen(0, '127.0.0.1', () => {
      const port = server.address().port;
      baseUrl = `http://127.0.0.1:${port}`;
      resolve();
    });
  });

  t.afterEach(() => {
    cp.execSync = originalExecSync;
    restoreFetch();
  });

  t.after(async () => {
    await new Promise((resolve) => server.close(resolve));
  });

  // --- POST /render ---
  await t.test('POST /render: rejeita código vazio com 400', async () => {
    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '   ' })
    });
    assert.equal(res.status, 400);
    const body = await res.json();
    assert.ok(body.error.includes('Nenhum código fornecido'));
  });

  await t.test('POST /render: processa bloco ```kinship / ```heredograma', async () => {
    cp.execSync = (cmd) => {
      const match = cmd.match(/"(\/tmp\/ped_[^"]+\.svg)"/);
      if (match) fs.writeFileSync(match[1], '<svg class="ped"></svg>');
    };

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```kinship\nid,sex\n1,M\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'kinship');
    assert.equal(body.type, 'svg');
    assert.ok(body.svg.includes('<svg'));
  });

  await t.test('POST /render: processa bloco ```pedtools', async () => {
    cp.execSync = (cmd) => {
      const match = cmd.match(/"(\/tmp\/pedtools_[^"]+\.svg)"/);
      if (match) fs.writeFileSync(match[1], '<svg class="pedtools-res"></svg>');
    };

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```pedtools\nid,sex\n1,M\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'pedtools');
    assert.equal(body.type, 'svg');
    assert.ok(body.svg.includes('pedtools-res'));
  });

  await t.test('POST /render: processa bloco ```kinship2', async () => {
    cp.execSync = (cmd) => {
      const match = cmd.match(/"(\/tmp\/ped_[^"]+\.svg)"/);
      if (match) fs.writeFileSync(match[1], '<svg class="kinship2-res"></svg>');
    };

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```kinship2\nid,sex\n1,M\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'kinship2');
    assert.equal(body.type, 'svg');
    assert.ok(body.svg.includes('kinship2-res'));
  });

  await t.test('POST /render: processa bloco ```openchemlib', async () => {
    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```openchemlib\nCC(=O)Oc1ccccc1C(=O)O\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'openchemlib');
    assert.equal(body.type, 'svg');
    assert.ok(body.svg.includes('<svg'));
  });

  await t.test('POST /render: processa bloco ```openbabel', async () => {
    cp.execSync = (cmd) => {
      const match = cmd.match(/"(\/tmp\/(?:ob|chem)_[^"]+\.svg)"/);
      if (match) fs.writeFileSync(match[1], '<svg class="openbabel"></svg>');
    };
    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```openbabel\nCC(=O)O\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'openbabel');
    assert.equal(body.type, 'svg');
    assert.ok(body.svg.includes('<svg'));
  });

  await t.test('POST /render: processa bloco ```table', async () => {
    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```table\nA, B\n1, 2\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'table');
    assert.equal(body.type, 'html');
    assert.ok(body.html.includes('class="joaju-academic-table"'));
  });

  await t.test('POST /render: processa bloco ```graphviz', async () => {
    mockKroki(async () => new Response('<svg class="dot"></svg>', { status: 200 }));

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```graphviz\ndigraph G { A -> B }\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'graphviz');
    assert.ok(body.svg.includes('class="dot"'));
  });

  await t.test('POST /render: processa bloco ```plantuml', async () => {
    mockKroki(async () => new Response('<svg class="puml"></svg>', { status: 200 }));

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```plantuml\nAlice -> Bob\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'plantuml');
    assert.ok(body.svg.includes('class="puml"'));
  });

  await t.test('POST /render: processa bloco ```d2', async () => {
    cp.execSync = (cmd) => {
      const match = cmd.match(/"(\/tmp\/d2_[^"]+\.svg)"/);
      if (match) fs.writeFileSync(match[1], '<svg class="d2"></svg>');
    };

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```d2\nx -> y\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'd2');
    assert.ok(body.svg.includes('class="d2"'));
  });

  await t.test('POST /render: processa bloco ```mermaid', async () => {
    mockKroki(async () => new Response('<svg class="mmd"></svg>', { status: 200 }));

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```mermaid\ngraph TD\nA-->B\n```' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'mermaid');
    assert.ok(body.svg.includes('class="mmd"'));
  });

  await t.test('POST /render: infere Mermaid heuristicamente sem crases', async () => {
    mockKroki(async () => new Response('<svg class="heuristic-mmd"></svg>', { status: 200 }));

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: 'flowchart LR\nInicio --> Fim' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'mermaid');
    assert.ok(body.svg.includes('heuristic-mmd'));
  });

  await t.test('POST /render: infere Kinship heuristicamente sem crases', async () => {
    cp.execSync = (cmd) => {
      const match = cmd.match(/"(\/tmp\/ped_[^"]+\.svg)"/);
      if (match) fs.writeFileSync(match[1], '<svg class="heuristic-ped"></svg>');
    };

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: 'id,pai,mae,sex\n1,0,0,M' })
    });
    assert.equal(res.status, 200);
    const body = await res.json();
    assert.equal(body.format, 'kinship');
    assert.ok(body.svg.includes('heuristic-ped'));
  });

  await t.test('POST /render: responde 500 em falhas do renderizador', async () => {
    cp.execSync = () => {
      throw new Error('Falha crítica de execução');
    };

    const res = await fetch(`${baseUrl}/render`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code: '```heredograma\nid,sex\n1,1\n```' })
    });
    assert.equal(res.status, 500);
    const body = await res.json();
    assert.ok(body.error.includes('Falha crítica de execução'));
  });

  // --- POST /compile (Pipeline Completo) ---
  await t.test('POST /compile: compila artigo ABNT com sucesso e devolve PDF', async () => {
    let pandocExecuted = false;
    let typstExecuted = false;

    cp.execSync = (cmd) => {
      if (cmd.startsWith('pandoc')) {
        pandocExecuted = true;
        const outMatch = cmd.match(/-o "([^"]+)"/);
        if (outMatch) fs.writeFileSync(outMatch[1], '= Artigo Científico\nTexto gerado.');
      } else if (cmd.startsWith('typst compile')) {
        typstExecuted = true;
        const pdfMatch = cmd.match(/"([^"]+\.pdf)"/);
        if (pdfMatch) fs.writeFileSync(pdfMatch[1], '%PDF-1.4 Mock Binary PDF Buffer');
      }
    };

    const md = `---
titulo: Artigo Teste
---
# Introdução
Texto do artigo.

\`\`\`typst
#text(blue)[Texto em azul nativo]
\`\`\`
`;

    const res = await fetch(`${baseUrl}/compile`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ markdown: md })
    });

    assert.equal(res.status, 200);
    assert.equal(res.headers.get('content-type'), 'application/pdf');
    assert.ok(res.headers.get('content-disposition').includes('filename="PIT.pdf"'));
    const pdfBytes = Buffer.from(await res.arrayBuffer());
    assert.ok(pdfBytes.toString().includes('%PDF-1.4'));
    assert.ok(pandocExecuted);
    assert.ok(typstExecuted);
  });

  await t.test('POST /compile: extrai perfil vancouver do YAML frontmatter', async () => {
    let capturedCmd = '';
    cp.execSync = (cmd) => {
      if (cmd.startsWith('pandoc')) {
        capturedCmd = cmd;
        const outMatch = cmd.match(/-o "([^"]+)"/);
        if (outMatch) fs.writeFileSync(outMatch[1], '= Vancouver Output');
      } else if (cmd.startsWith('typst compile')) {
        const pdfMatch = cmd.match(/"([^"]+\.pdf)"/);
        if (pdfMatch) fs.writeFileSync(pdfMatch[1], '%PDF-1.4 Vancouver');
      }
    };

    const md = `---
titulo: Artigo Médico
perfil: vancouver
---
# Caso Clínico
`;

    const res = await fetch(`${baseUrl}/compile`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ markdown: md })
    });

    assert.equal(res.status, 200);
    assert.ok(capturedCmd.includes('--csl=/data/vancouver.csl'));
  });

  await t.test('POST /compile: retorna 500 quando Pandoc falha', async () => {
    cp.execSync = (cmd) => {
      if (cmd.startsWith('pandoc')) throw new Error('Pandoc filter syntax error');
    };

    const res = await fetch(`${baseUrl}/compile`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ markdown: '# Artigo que quebra' })
    });

    assert.equal(res.status, 500);
    const errText = await res.text();
    assert.ok(errText.includes('Pandoc filter syntax error'));
  });

  // --- Health Check ---
  await t.test('retorna 200 e payload json em GET /health', async () => {
    const res = await fetch(`${baseUrl}/health`);
    assert.equal(res.status, 200);
    assert.ok(res.headers.get('content-type').includes('application/json'));
    const body = await res.json();
    assert.equal(body.status, 'ok');
    assert.equal(body.name, 'joaju');
    assert.ok(typeof body.version === 'string');
    assert.ok(typeof body.uptime === 'number');
  });

  // --- Ativos Estáticos (Public Playground) ---
  await t.test('serve index.html em GET /', async () => {
    const res = await fetch(`${baseUrl}/`);
    assert.equal(res.status, 200);
    assert.ok(res.headers.get('content-type').includes('text/html'));
    const text = await res.text();
    assert.ok(text.includes('Joaju'));
    assert.ok(text.includes('btn-render'));
  });

  await t.test('serve style.css em GET /style.css', async () => {
    const res = await fetch(`${baseUrl}/style.css`);
    assert.equal(res.status, 200);
    assert.ok(res.headers.get('content-type').includes('text/css'));
    const text = await res.text();
    assert.ok(text.includes('--bg-main'));
  });

  await t.test('serve app.js em GET /app.js', async () => {
    const res = await fetch(`${baseUrl}/app.js`);
    assert.equal(res.status, 200);
    assert.ok(res.headers.get('content-type').includes('javascript'));
    const text = await res.text();
    assert.ok(text.includes('SAMPLE_PEDIGREE'));
  });

  await t.test('retorna 404 para rota ou arquivo estático inexistente', async () => {
    const res = await fetch(`${baseUrl}/nao-existe.html`);
    assert.equal(res.status, 404);
  });
});
