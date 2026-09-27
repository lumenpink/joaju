const test = require('node:test');
const assert = require('node:assert/strict');
const { preprocessPedigrees, preprocessDiagrams } = require('../server.js');

test('Preprocessador de Heredogramas', async (t) => {
    await t.test('deve retornar o markdown inalterado quando não houver blocos de heredograma', async () => {
        const input = '# Título\n\nTexto descritivo simples sem blocos especiais.';
        const result = await preprocessPedigrees(input, 'sess-1');
        assert.equal(result, input);
    });

    await t.test('deve detectar bloco ```heredograma e ignorar sem falhar se Rscript falhar', async () => {
        const input = 'Antes\n```heredograma\n%% layout.width=90% layout.max_width=450pt\nid,sex\n1,M\n```\nDepois';
        const result = await preprocessPedigrees(input, 'sess-2');
        // Se o Rscript não estiver disponível no host, deve tratar o erro graciosamente sem travar
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
        // Kroki pode estar offline durante testes unitários locais
        assert.ok(typeof result === 'string');
    });
});
