const test = require('node:test');
const assert = require('node:assert/strict');
const http = require('node:http');
const { app, preprocessPedigrees, preprocessDiagrams } = require('../server.js');

test('Preprocessador de Heredogramas', async (t) => {
    await t.test('deve retornar o markdown inalterado quando não houver blocos de heredograma', async () => {
        const input = '# Título\n\nTexto descritivo simples sem blocos especiais.';
        const result = await preprocessPedigrees(input, 'sess-1');
        assert.equal(result, input);
    });

    await t.test('deve detectar bloco ```heredograma e ignorar sem falhar se Rscript falhar', async () => {
        const input = 'Antes\n```heredograma\n%% layout.width=90% layout.max_width=450pt\nid,sex\n1,M\n```\nDepois';
        const result = await preprocessPedigrees(input, 'sess-2');
        assert.ok(typeof result === 'string');
    });

    await t.test('deve suportar alias ```pedigree', async () => {
        const input = 'Intro\n```pedigree\nid,sex\n1,M\n```\nFim';
        const result = await preprocessPedigrees(input, 'sess-3');
        assert.ok(typeof result === 'string');
    });
});

test('Preprocessador de Diagramas Mermaid', async (t) => {
    await t.test('deve retornar o markdown inalterado quando não houver blocos mermaid', async () => {
        const input = '# Artigo Científico\n\nNenhum diagrama presente.';
        const result = await preprocessDiagrams(input, 'sess-4');
        assert.equal(result, input);
    });

    await t.test('deve tratar conexão com Kroki graciosamente quando offline', async () => {
        const input = 'Diagrama:\n```mermaid\n%% width=80% max-width=600pt\ngraph TD\nA-->B\n```';
        const result = await preprocessDiagrams(input, 'sess-5');
        assert.ok(typeof result === 'string');
    });
});

test('Endpoint POST /render e Static Playground', async (t) => {
    let server;
    let baseUrl;

    await new Promise((resolve) => {
        server = http.createServer(app);
        server.listen(0, '127.0.0.1', () => {
            const port = server.address().port;
            baseUrl = `http://127.0.0.1:${port}`;
            resolve();
        });
    });

    try {
        await t.test('deve rejeitar requisição sem código com status 400', async () => {
            const res = await fetch(`${baseUrl}/render`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ code: '' })
            });
            assert.equal(res.status, 400);
            const data = await res.json();
            assert.ok(data.error);
        });

        await t.test('deve servir arquivos estáticos em GET /', async () => {
            const res = await fetch(`${baseUrl}/`);
            assert.equal(res.status, 200);
            const html = await res.text();
            assert.ok(html.includes('Joaju'));
            assert.ok(html.includes('code-input'));
        });
    } finally {
        await new Promise((resolve) => server.close(resolve));
    }
});
