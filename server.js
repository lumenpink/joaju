const express = require('express');
const cp = require('node:child_process');
const fs = require('node:fs');
const uuid = require('uuid');
const path = require('node:path');

const app = express();

// 🌐 Suporte a CORS para integração com Obsidian e clientes desktop/mobile
app.use((req, res, next) => {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
  if (req.method === 'OPTIONS') {
    return res.sendStatus(204);
  }
  next();
});

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

// --- Pré-processador 1: Heredogramas Clínicos Avançados (R / Kinship) ---
async function preprocessPedigrees(markdown, sessionId) {
  const regex = /```(kinship|kinship2|heredograma|pedigree)\n([\s\S]*?)\n```/gi;
  let match;
  let modifiedMarkdown = markdown;
  let index = 0;

  while ((match = regex.exec(markdown)) !== null) {
    const rawContent = match[2].trim();
    const csvPath = `/tmp/ped_${sessionId}_${index}.csv`;
    const svgPath = `/tmp/ped_${sessionId}_${index}.svg`;

    let layoutWidth = '80%';
    let layoutMaxWidth = '500pt';
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
      const scriptPath = fs.existsSync('/app/plot_pedigree.R')
        ? '/app/plot_pedigree.R'
        : path.join(__dirname, 'plot_pedigree.R');
      cp.execSync(
        `Rscript "${scriptPath}" "${csvPath}" "${svgPath}" "${rArgsString.replace(/"/g, '\\"')}"`
      );

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

// --- Pré-processador 2: Diagramas Rápidos (Graphviz, PlantUML, D2, Mermaid) ---
async function preprocessDiagrams(markdown, sessionId) {
  const regex = /```(graphviz|dot|plantuml|puml|d2|mermaid)\n([\s\S]*?)\n```/gi;
  let match;
  let modifiedMarkdown = markdown;
  let index = 0;

  while ((match = regex.exec(markdown)) !== null) {
    const rawEngine = match[1].toLowerCase();
    const code = match[2];
    const isPng = rawEngine === 'mermaid';
    const ext = isPng ? 'png' : 'svg';
    const imgPath = `/tmp/diag_${sessionId}_${index}.${ext}`;

    // 🔍 Captura diretivas visuais %% width=...
    const configMatch = code.match(
      /%%\s*(?:layout\.)?width=([\d%]+)\s+(?:layout\.)?max-width=([\d\w]+)/
    );
    let width = '75%';
    let maxWidth = '500pt';

    if (configMatch) {
      width = configMatch[1];
      maxWidth = configMatch[2];
    }

    try {
      if (rawEngine === 'd2') {
        const svg = renderSingleD2(code, `${sessionId}_${index}`);
        fs.writeFileSync(imgPath, svg);
      } else {
        const krokiHost = process.env.KROKI_HOST || 'http://kroki:8000';
        const targetEngine =
          rawEngine === 'dot' ? 'graphviz' : rawEngine === 'puml' ? 'plantuml' : rawEngine;
        const format = isPng ? 'png' : 'svg';
        const response = await fetch(`${krokiHost}/${targetEngine}/${format}`, {
          method: 'POST',
          headers: { 'Content-Type': 'text/plain' },
          body: code,
          signal: AbortSignal.timeout(10000)
        });

        if (response.ok) {
          const buffer = Buffer.from(await response.arrayBuffer());
          fs.writeFileSync(imgPath, buffer);
        } else {
          throw new Error(`Kroki retornou ${response.status}`);
        }
      }

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
    } catch (err) {
      console.error(`Erro Kroki: ${err.message}`);
    }
    index++;
  }
  return modifiedMarkdown;
}

// --- Renderizadores Isolados para Preview Instantâneo ---
function renderSingleKinship(rawContent, sessionId = 'preview') {
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
        if (
          key !== 'layout.width' &&
          key !== 'width' &&
          key !== 'layout.max_width' &&
          key !== 'max-width'
        ) {
          rArgs.push(`${key} = ${val}`);
        }
      }
    } else {
      csvLines.push(line);
    }
  }

  fs.writeFileSync(csvPath, csvLines.join('\n'));
  const rArgsString = `list(${rArgs.join(', ')})`;
  const scriptPath = fs.existsSync('/app/plot_pedigree.R')
    ? '/app/plot_pedigree.R'
    : path.join(__dirname, 'plot_pedigree.R');

  try {
    cp.execSync(
      `Rscript "${scriptPath}" "${csvPath}" "${svgPath}" "${rArgsString.replace(/"/g, '\\"')}"`,
      { stdio: ['pipe', 'pipe', 'pipe'] }
    );
    if (fs.existsSync(svgPath)) {
      const svgContent = fs.readFileSync(svgPath, 'utf8');
      return svgContent;
    }
    const missingSvgErr = new Error('Arquivo SVG não gerado pelo Rscript');
    missingSvgErr.status = 422;
    throw missingSvgErr;
  } catch (err) {
    if (err.status && err.status >= 400 && err.status < 600) {
      throw err;
    }
    const stderr = err.stderr ? err.stderr.toString() : '';
    if (err.status !== undefined || stderr?.includes('Error')) {
      const cleanMsg =
        stderr
          .split('\n')
          .filter((l) => l.startsWith('Error in') || l.startsWith('Error:'))
          .map((l) => l.replace(/^Error.*:\s*/, ''))
          .join('; ') || err.message;
      const pedError = new Error(`Erro no heredograma (kinship): ${cleanMsg}`);
      pedError.status = 422;
      throw pedError;
    }
    err.status = 500;
    throw err;
  } finally {
    try {
      if (fs.existsSync(csvPath)) fs.unlinkSync(csvPath);
      if (fs.existsSync(svgPath)) fs.unlinkSync(svgPath);
    } catch (_) {}
  }
}

const renderSinglePedigree = renderSingleKinship;

async function renderSingleGraphviz(rawContent) {
  const cleanLines = rawContent
    .split('\n')
    .filter((line) => !line.trim().startsWith('%%'))
    .join('\n')
    .trim();

  const krokiHost = process.env.KROKI_HOST || 'http://kroki:8000';
  let response;
  try {
    response = await fetch(`${krokiHost}/graphviz/svg`, {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: cleanLines,
      signal: AbortSignal.timeout(10000)
    });
  } catch (err) {
    if (err.name === 'TimeoutError') {
      const timeoutError = new Error('Tempo limite excedido na comunicação com o motor Graphviz.');
      timeoutError.status = 504;
      throw timeoutError;
    }
    const netError = new Error(`Falha de conexão com o motor gráfico Graphviz (${err.message})`);
    netError.status = 503;
    throw netError;
  }

  if (!response.ok) {
    const errorDetail = await response.text();
    const syntaxError = new Error(
      `Erro do Graphviz (${response.status}): ${errorDetail.split('\n')[0] || response.statusText}`
    );
    syntaxError.status = response.status === 400 ? 422 : response.status;
    throw syntaxError;
  }

  return await response.text();
}

async function renderSinglePlantuml(rawContent) {
  const cleanLines = rawContent
    .split('\n')
    .filter((line) => !line.trim().startsWith('%%'))
    .join('\n')
    .trim();

  let plantCode = cleanLines;
  if (
    !plantCode.includes('@startuml') &&
    !plantCode.includes('@startmindmap') &&
    !plantCode.includes('@startwbs') &&
    !plantCode.includes('@startgantt')
  ) {
    plantCode = `@startuml\n${plantCode}\n@enduml`;
  }

  const krokiHost = process.env.KROKI_HOST || 'http://kroki:8000';
  let response;
  try {
    response = await fetch(`${krokiHost}/plantuml/svg`, {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: plantCode,
      signal: AbortSignal.timeout(15000)
    });
  } catch (err) {
    if (err.name === 'TimeoutError') {
      const timeoutError = new Error('Tempo limite excedido na comunicação com o motor PlantUML.');
      timeoutError.status = 504;
      throw timeoutError;
    }
    const netError = new Error(`Falha de conexão com o motor gráfico PlantUML (${err.message})`);
    netError.status = 503;
    throw netError;
  }

  if (!response.ok) {
    const errorDetail = await response.text();
    const syntaxError = new Error(
      `Erro do PlantUML (${response.status}): ${errorDetail.split('\n')[0] || response.statusText}`
    );
    syntaxError.status = response.status === 400 ? 422 : response.status;
    throw syntaxError;
  }

  return await response.text();
}

function renderSingleD2(rawContent, sessionId = 'preview') {
  const cleanLines = rawContent
    .split('\n')
    .filter((line) => !line.trim().startsWith('%%'))
    .join('\n')
    .trim();

  const d2Path = `/tmp/d2_${sessionId}.d2`;
  const svgPath = `/tmp/d2_${sessionId}.svg`;
  fs.writeFileSync(d2Path, cleanLines);

  try {
    cp.execSync(`d2 "${d2Path}" "${svgPath}"`, { stdio: ['pipe', 'pipe', 'pipe'] });
    if (fs.existsSync(svgPath)) {
      return fs.readFileSync(svgPath, 'utf8');
    }
    const missingSvgErr = new Error('Arquivo SVG não gerado pelo D2');
    missingSvgErr.status = 422;
    throw missingSvgErr;
  } catch (err) {
    if (err.status && err.status >= 400 && err.status < 600) {
      throw err;
    }
    const stderr = err.stderr ? err.stderr.toString() : '';
    const cleanMsg =
      stderr
        .split('\n')
        .filter((l) => l.trim().length > 0)
        .slice(0, 3)
        .join('; ') || err.message;
    const d2Error = new Error(`Erro de sintaxe no D2: ${cleanMsg}`);
    d2Error.status = 422;
    throw d2Error;
  } finally {
    try {
      if (fs.existsSync(d2Path)) fs.unlinkSync(d2Path);
      if (fs.existsSync(svgPath)) fs.unlinkSync(svgPath);
    } catch (_) {}
  }
}

async function renderSingleMermaid(rawContent) {
  const cleanLines = rawContent
    .split('\n')
    .filter((line) => !line.trim().startsWith('%%'))
    .join('\n')
    .trim();

  const krokiHost = process.env.KROKI_HOST || 'http://kroki:8000';
  let response;
  try {
    response = await fetch(`${krokiHost}/mermaid/svg`, {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: cleanLines,
      signal: AbortSignal.timeout(40000)
    });
  } catch (err) {
    if (err.name === 'TimeoutError') {
      const timeoutError = new Error('Tempo limite excedido na comunicação com o gateway Kroki.');
      timeoutError.status = 504;
      throw timeoutError;
    }
    const netError = new Error(`Falha de conexão com o motor gráfico Kroki (${err.message})`);
    netError.status = 503;
    throw netError;
  }

  if (!response.ok) {
    const errorDetail = await response.text();
    const syntaxError = new Error(
      `Erro do Kroki (${response.status}): ${errorDetail.split('\n')[0] || response.statusText}`
    );
    syntaxError.status = response.status === 400 ? 422 : response.status;
    throw syntaxError;
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
  const blockMatch = rawInput.match(
    /```(kinship|kinship2|heredograma|pedigree|graphviz|dot|plantuml|puml|d2|mermaid)\n([\s\S]*?)\n?```/i
  );
  let type = 'unknown';
  let content = rawInput;

  if (blockMatch) {
    const tag = blockMatch[1].toLowerCase();
    if (tag === 'kinship' || tag === 'kinship2' || tag === 'heredograma' || tag === 'pedigree') {
      type = 'kinship';
    } else if (tag === 'graphviz' || tag === 'dot') {
      type = 'graphviz';
    } else if (tag === 'plantuml' || tag === 'puml') {
      type = 'plantuml';
    } else if (tag === 'd2') {
      type = 'd2';
    } else if (tag === 'mermaid') {
      type = 'mermaid';
    }
    content = blockMatch[2].trim();
  } else {
    // 2. Inferência heurística se o usuário colar apenas o miolo
    if (/^\s*(digraph|strict digraph|graph|subgraph)\b/m.test(rawInput)) {
      type = 'graphviz';
    } else if (/^\s*(@start|class |actor |participant |autonumber)/m.test(rawInput)) {
      type = 'plantuml';
    } else if (
      /^(flowchart|sequenceDiagram|classDiagram|stateDiagram|erDiagram|gantt|pie|gitGraph|journey)/m.test(
        rawInput
      ) ||
      rawInput.includes('-->') ||
      rawInput.includes('graph TD') ||
      rawInput.includes('graph LR')
    ) {
      type = 'mermaid';
    } else if (/^\s*[\w\d_-]+\s*(?:->|<-|--|<->)\s*[\w\d_-]+/m.test(rawInput)) {
      type = 'd2';
    } else if (
      /^(id\s*,|pai\s*,|mae\s*,|dadid\s*,|momid\s*,)/im.test(rawInput) ||
      /^(\d+)\s*,\s*(\d+|\bNA\b)\s*,\s*(\d+|\bNA\b)\s*,/m.test(rawInput)
    ) {
      type = 'kinship';
    } else {
      type = rawInput.includes(',') ? 'kinship' : 'graphviz';
    }
  }

  const sessionId = uuid.v4();

  try {
    if (type === 'kinship') {
      const svg = renderSingleKinship(content, sessionId);
      return res.json({ format: 'kinship', type: 'svg', svg });
    }
    if (type === 'graphviz') {
      const svg = await renderSingleGraphviz(content);
      return res.json({ format: 'graphviz', type: 'svg', svg });
    }
    if (type === 'plantuml') {
      const svg = await renderSinglePlantuml(content);
      return res.json({ format: 'plantuml', type: 'svg', svg });
    }
    if (type === 'd2') {
      const svg = renderSingleD2(content, sessionId);
      return res.json({ format: 'd2', type: 'svg', svg });
    }
    if (type === 'mermaid') {
      const svg = await renderSingleMermaid(content);
      return res.json({ format: 'mermaid', type: 'svg', svg });
    }
  } catch (err) {
    const rawStatus = err.statusCode || err.status;
    const statusCode =
      Number.isInteger(rawStatus) && rawStatus >= 400 && rawStatus < 600 ? rawStatus : 422;
    return res.status(statusCode).json({ error: err.message });
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

    // Injeta suporte a réguas horizontais do Pandoc e função min no escopo do arquivo Typst
    if (fs.existsSync(tempTypstOutput)) {
      const originalTypst = fs.readFileSync(tempTypstOutput, 'utf8');
      const patchedTypst = `#let horizontalrule = line(length: 100%, stroke: 0.5pt + luma(180))\n#let min(a, b) = if type(a) == ratio { a } else if type(b) == ratio { b } else { calc.min(a, b) }\n${originalTypst}`;
      fs.writeFileSync(tempTypstOutput, patchedTypst);
    }

    // 2️⃣ ETAPA 2: Typst (Diagramação Rápida)
    // Cria um wrapper que envelopa o texto compilado no template master.
    const typstWrapper = `
#import "/data/template.typ": abnt
#show: doc => abnt(doc)
#let horizontalrule = line(length: 100%, stroke: 0.5pt + luma(180))

#include "${tempTypstOutput}"
`;
    fs.writeFileSync(tempWrapperTypst, typstWrapper);

    // Compila o Typst nativo na porta 3000
    const typstCmd = `typst compile --root / "${tempWrapperTypst}" "${tempPdf}"`;
    cp.execSync(typstCmd);

    // Lê o PDF vetorial final
    const pdfBuffer = fs.readFileSync(tempPdf);
    res.setHeader('Content-Type', 'application/pdf');
    res.setHeader('Content-Disposition', 'attachment; filename="PIT.pdf"');
    res.send(pdfBuffer);

    // 🧹 Limpeza assíncrona dos arquivos temporários de compilação
    for (const file of [tempMd, tempTypstOutput, tempWrapperTypst, tempPdf]) {
      if (fs.existsSync(file)) fs.unlinkSync(file);
    }
  } catch (err) {
    console.error(`Erro fatal no pipeline: ${err.message}`);
    res.status(500).send(`Erro na compilação do artigo: ${err.message}`);
  }
});

const PORT = process.env.PORT || 3000;
const HOST = process.env.HOST || '0.0.0.0';
if (require.main === module) {
  app.listen(PORT, HOST, () =>
    console.log(`✅ Pipeline acadêmico orquestrando em http://${HOST}:${PORT}...`)
  );
}

module.exports = {
  app,
  preprocessPedigrees,
  preprocessDiagrams,
  renderSinglePedigree,
  renderSingleKinship,
  renderSingleGraphviz,
  renderSinglePlantuml,
  renderSingleD2,
  renderSingleMermaid
};
