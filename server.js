const express = require('express');
const cp = require('child_process');
const fs = require('fs');
const uuid = require('uuid');
const path = require('path');

const app = express();
app.use(express.json({ limit: '50mb' })); // Suporte a documentos longos
app.use(express.static(path.join(__dirname, 'public')));
app.use('/locales', express.static(path.join(__dirname, 'locales')));

// 🩺 Endpoint de verificação de integridade (Health Check)
app.get('/health', (req, res) => {
    res.json({
        status: 'ok',
        name: 'joaju',
        version: process.env.npm_package_version || '0.1.0',
        uptime: process.uptime()
    });
});

// --- Pré-processador 1: Heredogramas Clínicos Avançados (R) ---
async function preprocessPedigrees(markdown, sessionId) {
    const regex = /```(heredograma|pedigree)\n([\s\S]*?)\n```/g;
    let match;
    let modifiedMarkdown = markdown;
    let index = 0;

    while ((match = regex.exec(markdown)) !== null) {
        const rawContent = match[2].trim();
        const csvPath = `/tmp/ped_${sessionId}_${index}.csv`;
        const svgPath = `/tmp/ped_${sessionId}_${index}.svg`;

        let layoutWidth = "80%";
        let layoutMaxWidth = "500pt";
        const rArgs = [];

        // 🔍 Separa diretivas visuais (%% key=val) dos dados tabulares
        const lines = rawContent.split('\n');
        const csvLines = [];

        for (const line of lines) {
            const trimmed = line.trim();
            if (trimmed.startsWith('%%')) {
                // Remove o %% e captura os pares chave=valor, aceitando c(...) do R
                const directiveText = trimmed.replace(/^%%\s*/, '');
                const pairs = directiveText.match(/[\w.-]+=(?:c\([^)]+\)|"[^"]*"|'[^']*'|[^\s]+)/g) || [];

                for (const pair of pairs) {
                    const [key, val] = pair.split(/=(.*)/s);
                    // Dimensões do bloco Typst na página A4
                    if (key === 'layout.width' || (key === 'width' && /[%a-z]/i.test(val))) {
                        layoutWidth = val;
                    } else if (key === 'layout.max_width' || (key === 'max-width' && /[%a-z]/i.test(val))) {
                        layoutMaxWidth = val;
                    } else {
                        // Qualquer spec oficial do kinship2 (cex, symbolsize, branch, packed, align, etc.)
                        rArgs.push(`${key} = ${val}`);
                    }
                }
            } else {
                csvLines.push(line);
            }
        }

        fs.writeFileSync(csvPath, csvLines.join('\n'));

        // Monta a expressão R nativa: list(cex=0.8, ...)
        const rArgsString = `list(${rArgs.join(', ')})`;

        try {
            const scriptPath = fs.existsSync('/app/plot_pedigree.R') ? '/app/plot_pedigree.R' : path.join(__dirname, 'plot_pedigree.R');
            cp.execSync(`Rscript "${scriptPath}" "${csvPath}" "${svgPath}" "${rArgsString.replace(/"/g, '\\"')}"`);

            if (fs.existsSync(svgPath)) {
                // Injeta o layout Typst perfeito com os limites dimensionais
                const typstLayout = `
\`\`\`{=typst}
#align(center)[
  #block(width: min(${layoutWidth},${layoutMaxWidth}))[
    #image("${svgPath}")
  ]
]
\`\`\`
`;
                modifiedMarkdown = modifiedMarkdown.replace(match[0], typstLayout);
            }
        } catch (err) {
            console.error(`Erro ao compilar heredograma: ${err.message}`);
        }
        index++;
    }
    return modifiedMarkdown;
}

// --- Pré-processador 2: Diagramas Genéricos Isolados (Kroki via Mermaid) ---
async function preprocessDiagrams(markdown, sessionId) {
    const regex = /```mermaid\n([\s\S]*?)\n```/g;
    let match;
    let modifiedMarkdown = markdown;
    let index = 0;

    while ((match = regex.exec(markdown)) !== null) {
        const code = match[1];
        const imgPath = `/tmp/diag_${sessionId}_${index}.png`;

        // 🔍 Captura diretivas visuais %% width=... do Mermaid
        const configMatch = code.match(/%%\s*width=([\d%]+)\s+max-width=([\d\w]+)/);
        let width = "75%";
        let maxWidth = "500pt";

        if (configMatch) {
            width = configMatch[1];
            maxWidth = configMatch[2];
        }

        try {
            // Requisição ao gateway Kroki na rede Docker interna
            const response = await fetch(`http://kroki:8000/mermaid/png`, {
                method: 'POST',
                headers: { 'Content-Type': 'text/plain' },
                body: code
            });

            if (response.ok) {
                const buffer = Buffer.from(await response.arrayBuffer());
                fs.writeFileSync(imgPath, buffer);

                const typstLayout = `
\`\`\`{=typst}
#align(center)[
  #block(width: min(${width},${maxWidth}))[
    #image("${imgPath}")
  ]
]
\`\`\`
`;
                modifiedMarkdown = modifiedMarkdown.replace(match[0], typstLayout);
            }
        } catch (err) {
            console.error(`Erro Kroki: ${err.message}`);
        }
        index++;
    }
    return modifiedMarkdown;
}

// --- Renderizadores Isolados para Preview Instantâneo ---
function renderSinglePedigree(rawContent, sessionId = 'preview') {
    const csvPath = `/tmp/ped_${sessionId}.csv`;
    const svgPath = `/tmp/ped_${sessionId}.svg`;

    const rArgs = [];
    const lines = rawContent.split('\n');
    const csvLines = [];

    for (const line of lines) {
        const trimmed = line.trim();
        if (trimmed.startsWith('%%')) {
            const directiveText = trimmed.replace(/^%%\s*/, '');
            const pairs = directiveText.match(/[\w.-]+=(?:c\([^)]+\)|"[^"]*"|'[^']*'|[^\s]+)/g) || [];
            for (const pair of pairs) {
                const [key, val] = pair.split(/=(.*)/s);
                if (key !== 'layout.width' && key !== 'width' && key !== 'layout.max_width' && key !== 'max-width') {
                    rArgs.push(`${key} = ${val}`);
                }
            }
        } else {
            csvLines.push(line);
        }
    }

    fs.writeFileSync(csvPath, csvLines.join('\n'));
    const rArgsString = `list(${rArgs.join(', ')})`;
    const scriptPath = fs.existsSync('/app/plot_pedigree.R') ? '/app/plot_pedigree.R' : path.join(__dirname, 'plot_pedigree.R');

    try {
        cp.execSync(`Rscript "${scriptPath}" "${csvPath}" "${svgPath}" "${rArgsString.replace(/"/g, '\\"')}"`);
        if (fs.existsSync(svgPath)) {
            const svgContent = fs.readFileSync(svgPath, 'utf8');
            return svgContent;
        }
        throw new Error('Arquivo SVG não gerado pelo Rscript');
    } finally {
        try {
            if (fs.existsSync(csvPath)) fs.unlinkSync(csvPath);
            if (fs.existsSync(svgPath)) fs.unlinkSync(svgPath);
        } catch (_) {}
    }
}

async function renderSingleMermaid(rawContent) {
    const cleanLines = rawContent
        .split('\n')
        .filter(line => !line.trim().startsWith('%%'))
        .join('\n')
        .trim();

    const krokiHost = process.env.KROKI_HOST || 'http://kroki:8000';
    const response = await fetch(`${krokiHost}/mermaid/svg`, {
        method: 'POST',
        headers: { 'Content-Type': 'text/plain' },
        body: cleanLines
    });

    if (!response.ok) {
        const errorDetail = await response.text();
        throw new Error(`Erro do Kroki (${response.status}): ${errorDetail || response.statusText}`);
    }

    return await response.text();
}

// --- Rota de Preview Rápido para a Interface Web ---
app.post('/render', async (req, res) => {
    const rawInput = (req.body.code || req.body.markdown || '').trim();
    if (!rawInput) {
        return res.status(400).json({ error: 'Nenhum código fornecido para renderização.' });
    }

    // 1. Tenta extrair bloco demarcado por ```
    const blockMatch = rawInput.match(/```(heredograma|pedigree|mermaid)\n([\s\S]*?)\n?```/i);
    let type = 'unknown';
    let content = rawInput;

    if (blockMatch) {
        const tag = blockMatch[1].toLowerCase();
        type = (tag === 'heredograma' || tag === 'pedigree') ? 'pedigree' : 'mermaid';
        content = blockMatch[2].trim();
    } else {
        // 2. Inferência heurística se o usuário colar apenas o miolo
        if (/^(flowchart|sequenceDiagram|classDiagram|stateDiagram|erDiagram|gantt|pie|gitGraph|journey)/m.test(rawInput) || rawInput.includes('-->') || rawInput.includes('graph ')) {
            type = 'mermaid';
        } else {
            type = 'pedigree';
        }
    }

    const sessionId = uuid.v4();

    try {
        if (type === 'pedigree') {
            const svg = renderSinglePedigree(content, sessionId);
            return res.json({ format: 'pedigree', type: 'svg', svg });
        } else if (type === 'mermaid') {
            const svg = await renderSingleMermaid(content);
            return res.json({ format: 'mermaid', type: 'svg', svg });
        }
    } catch (err) {
        return res.status(500).json({ error: err.message });
    }
});

// --- Rota Principal de Compilação ---
app.post('/compile', async (req, res) => {
    const markdown = req.body.markdown || '';
    const id = uuid.v4();
    const tempMd = `/tmp/input_${id}.md`;
    const tempTypstOutput = `/tmp/conteudo_${id}.typ`;
    const tempWrapperTypst = `/tmp/main_${id}.typ`;
    const tempPdf = `/tmp/output_${id}.pdf`;

    // 🔍 Extrai metadados do YAML para automação bibliográfica
    // Procura o perfil (abnt, vancouver, etc.) condicionalmente
    const metaMatch = markdown.match(/^---[\s\S]*?perfil:\s*(\w+)[\s\S]*?---/);
    const profile = metaMatch ? metaMatch[1] : 'abnt';

    // 🔄 ORQUESTRADORES DE PRÉ-PROCESSAMENTO
    let cleanMarkdown = await preprocessPedigrees(markdown, id);
    cleanMarkdown = await preprocessDiagrams(cleanMarkdown, id);

    // Suporte genérico a blocos nativos: ```typst -> <typst>
    cleanMarkdown = cleanMarkdown.replace(/```typst\n([\s\S]*?)\n```/g, '$1');

    fs.writeFileSync(tempMd, cleanMarkdown);

    // 1️⃣ ETAPA 1: Pandoc + Citeproc (Maturação Bibliográfica)
    // O Pandoc resolve citações ABNT/Vancouver usando o Zotero e cospe texto Typst estático.
    const pandocCmd = `pandoc "${tempMd}" --citeproc --bibliography=/data/referencias.bib --csl=/data/${profile}.csl -t typst -o "${tempTypstOutput}"`;

    try {
        cp.execSync(pandocCmd);

        // 2️⃣ ETAPA 2: Typst (Diagramação Rápida)
        // Cria um wrapper que envelopa o texto compilado no template master.
        const typstWrapper = `
#import "/data/template.typ": abnt
#show: doc => abnt(doc)

#include "${tempTypstOutput}"
`;
        fs.writeFileSync(tempWrapperTypst, typstWrapper);

        // Compila o Typst nativo na porta 3000
        const typstCmd = `typst compile --root /data "${tempWrapperTypst}" "${tempPdf}"`;
        cp.execSync(typstCmd);

        // Lê o PDF vetorial final
        const pdfBuffer = fs.readFileSync(tempPdf);
        res.setHeader('Content-Type', 'application/pdf');
        res.setHeader('Content-Disposition', 'attachment; filename="PIT.pdf"');
        res.send(pdfBuffer);

        // 🧹 Limpeza assíncrona dos arquivos temporários de compilação
        [tempMd, tempTypstOutput, tempWrapperTypst, tempPdf].forEach(file => {
            if (fs.existsSync(file)) fs.unlinkSync(file);
        });

    } catch (err) {
        console.error(`Erro fatal no pipeline: ${err.message}`);
        res.status(500).send(`Erro na compilação do artigo: ${err.message}`);
    }
});

const PORT = process.env.PORT || 3000;
const HOST = process.env.HOST || '0.0.0.0';
if (require.main === module) {
    app.listen(PORT, HOST, () => console.log(`✅ Pipeline acadêmico orquestrando em http://${HOST}:${PORT}...`));
}

module.exports = { app, preprocessPedigrees, preprocessDiagrams, renderSinglePedigree, renderSingleMermaid };