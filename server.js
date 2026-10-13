const express = require('express');
const cp = require('node:child_process');
const fs = require('node:fs');
const os = require('node:os');
const uuid = require('uuid');
const path = require('node:path');
const OCL = require('openchemlib');
const { escapeTypst } = require('./unfurl');

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
app.use(
  express.raw({
    type: ['application/zip', 'application/x-zip-compressed', 'application/octet-stream'],
    limit: '100mb'
  })
);
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

/**
 * Se o bloco contiver %% src=... ou %% file=..., busca o conteúdo remoto ou lê do workspace local e mescla com as diretivas.
 */
async function resolveExternalSource(rawContent, workspaceDir = null, fetchFn = globalThis.fetch) {
  let wsDir = workspaceDir;
  let fn = fetchFn;
  if (typeof workspaceDir === 'function') {
    fn = workspaceDir;
    wsDir = null;
  }

  const srcMatch = rawContent.match(/%%\s*(?:src|file)=([^\s\n]+)/);
  if (!srcMatch) return rawContent;

  const urlOrPath = srcMatch[1].trim().replace(/^["']|["']$/g, '');
  let fetchedText = null;

  if (urlOrPath.startsWith('http://') || urlOrPath.startsWith('https://')) {
    try {
      const res = await fn(urlOrPath, { signal: AbortSignal.timeout(10000) });
      if (res.ok) {
        fetchedText = await res.text();
      }
    } catch (err) {
      console.warn(`[Joaju] Falha ao resolver URL remota em %% src: ${err.message}`);
    }
  } else if (wsDir) {
    const localPath = path.resolve(wsDir, urlOrPath);
    if (localPath.startsWith(path.resolve(wsDir)) && fs.existsSync(localPath)) {
      try {
        fetchedText = fs.readFileSync(localPath, 'utf8');
      } catch (err) {
        console.warn(`[Joaju] Falha ao ler arquivo local em %% src: ${err.message}`);
      }
    }
  }

  if (fetchedText !== null) {
    const remainingDirectives = rawContent
      .split('\n')
      .filter((l) => l.trim().startsWith('%%') && !l.trim().match(/^%%\s*(?:src|file)=/))
      .join('\n')
      .trim();
    return remainingDirectives
      ? `${remainingDirectives}\n${fetchedText}`.trim()
      : fetchedText.trim();
  }

  return rawContent;
}

function normalizeDimension(val, defaultVal = '100%') {
  if (!val) return defaultVal;
  const str = String(val).trim();
  if (/^\d+$/.test(str)) {
    return `${str}pt`;
  }
  return str;
}

// --- Pré-processador 1: Heredogramas Clínicos Avançados (R / pedtools e kinship2) ---
async function preprocessPedigrees(markdown, sessionId, workspaceDir = null) {
  const regex = /```(pedtools|kinship2)\n([\s\S]*?)\n```/gi;
  let match;
  let modifiedMarkdown = markdown;
  let index = 0;

  while ((match = regex.exec(markdown)) !== null) {
    const engineTag = match[1].toLowerCase();
    const isPedtools = engineTag === 'pedtools';
    let rawContent = match[2].trim();

    // Resolve fonte remota ou do workspace local %% src=... se houver
    rawContent = await resolveExternalSource(rawContent, workspaceDir);

    const prefix = isPedtools ? 'pedtools' : 'ped';
    const csvPath = `/tmp/${prefix}_${sessionId}_${index}.csv`;
    const svgPath = `/tmp/${prefix}_${sessionId}_${index}.svg`;

    let layoutWidth = '80%';
    let layoutMaxWidth = '500pt';
    const rArgs = [];

    // 🔍 Separa diretivas visuais (%% key=val) dos dados tabulares
    const lines = rawContent.split('\n');
    const csvLines = [];

    for (const line of lines) {
      const trimmed = line.trim();
      if (trimmed.startsWith('%%')) {
        // Remove o %% e captura os pares chave=valor, aceitando c(...) com parênteses e strings
        const directiveText = trimmed.replace(/^%%\s*/, '');
        const directiveRegex =
          /([\w.-]+)\s*=\s*(c\((?:[^()]*|\([^()]*\))*\)|"(?:\\.|[^"\\])*"|'[^']*'|[^\s]+)/g;
        let dMatch;

        while ((dMatch = directiveRegex.exec(directiveText)) !== null) {
          const key = dMatch[1];
          const val = dMatch[2];
          // Dimensões do bloco Typst na página A4
          if (key === 'layout.width' || (key === 'width' && /[%a-z]/i.test(val))) {
            layoutWidth = normalizeDimension(val, '80%');
          } else if (key === 'layout.max_width' || (key === 'max-width' && /[%a-z]/i.test(val))) {
            layoutMaxWidth = normalizeDimension(val, '500pt');
          } else if (key !== 'src' && key !== 'file') {
            rArgs.push(`${key} = ${val}`);
          }
        }
      } else {
        csvLines.push(line);
      }
    }

    fs.writeFileSync(csvPath, csvLines.join('\n'));

    // Monta a expressão R nativa: list(...)
    const rArgsString = `list(${rArgs.join(', ')})`;

    try {
      const scriptFile = isPedtools ? 'plot_pedtools.R' : 'plot_pedigree.R';
      const scriptPath = fs.existsSync(`/app/${scriptFile}`)
        ? `/app/${scriptFile}`
        : path.join(__dirname, scriptFile);
      cp.execSync(
        `Rscript "${scriptPath}" "${csvPath}" "${svgPath}" "${rArgsString.replace(/"/g, '\\"')}"`,
        { env: { ...process.env, LANG: 'C.utf8', LC_ALL: 'C.utf8' } }
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
      console.error(`Erro ao compilar heredograma (${engineTag}): ${err.message}`);
    }
    index++;
  }
  return modifiedMarkdown;
}

// --- Pré-processador 2: Diagramas Rápidos (Graphviz, PlantUML, D2, Mermaid) ---
async function preprocessDiagrams(markdown, sessionId, workspaceDir = null) {
  const regex = /```(graphviz|plantuml|d2|mermaid)\n([\s\S]*?)\n```/gi;
  let match;
  let modifiedMarkdown = markdown;
  let index = 0;

  while ((match = regex.exec(markdown)) !== null) {
    const rawEngine = match[1].toLowerCase();
    let code = match[2];
    code = await resolveExternalSource(code, workspaceDir);
    const isPng = rawEngine === 'mermaid';
    const ext = isPng ? 'png' : 'svg';
    const imgPath = `/tmp/diag_${sessionId}_${index}.${ext}`;

    // 🔍 Captura diretivas visuais %% width=...
    const configMatch = code.match(
      /%%\s*(?:layout\.)?width=([^\s]+)(?:\s+(?:layout\.)?max-width=([^\s]+))?/i
    );
    let width = '75%';
    let maxWidth = '500pt';

    if (configMatch) {
      if (configMatch[1]) width = normalizeDimension(configMatch[1], '75%');
      if (configMatch[2]) maxWidth = normalizeDimension(configMatch[2], '500pt');
    }

    try {
      if (rawEngine === 'd2') {
        const svg = renderSingleD2(code, `${sessionId}_${index}`);
        fs.writeFileSync(imgPath, svg);
      } else {
        const krokiHost = process.env.KROKI_HOST || 'http://kroki:8000';
        const targetEngine = rawEngine;
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

// --- Pré-processador 3: Química Estrutural (OpenChemLib e Open Babel) ---
async function preprocessChemistry(markdown, sessionId, workspaceDir = null) {
  const regex = /```(openchemlib|openbabel)\n([\s\S]*?)\n```/gi;
  let match;
  let modifiedMarkdown = markdown;
  let index = 0;

  while ((match = regex.exec(markdown)) !== null) {
    const engineTag = match[1].toLowerCase();
    let rawContent = match[2].trim();
    rawContent = await resolveExternalSource(rawContent, workspaceDir);

    const imgPath = `/tmp/chem_${sessionId}_${index}.svg`;

    const configMatch = rawContent.match(
      /%%\s*(?:layout\.)?width=([^\s]+)(?:\s+(?:layout\.)?max-width=([^\s]+))?/i
    );
    let width = '50%';
    let maxWidth = '350pt';

    if (configMatch) {
      if (configMatch[1]) width = normalizeDimension(configMatch[1], '50%');
      if (configMatch[2]) maxWidth = normalizeDimension(configMatch[2], '350pt');
    }

    try {
      let svg;
      if (engineTag === 'openchemlib') {
        svg = renderSingleOpenchemlib(rawContent);
      } else {
        svg = renderSingleOpenbabel(rawContent, `${sessionId}_${index}`);
      }

      fs.writeFileSync(imgPath, svg);

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
      console.error(`Erro ao compilar molécula (${engineTag}): ${err.message}`);
    }
    index++;
  }
  return modifiedMarkdown;
}

// --- Pré-processador 4: Tabelas Acadêmicas 3-Linhas (ABNT / IBGE / APA) ---
function parseTableData(rawContent) {
  const lines = rawContent.split('\n');
  const directives = {};
  const dataLines = [];

  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.startsWith('%%')) {
      const match = trimmed.match(/^%%\s*([\w.-]+)=(.*)$/);
      if (match) {
        const key = match[1].trim();
        let val = match[2].trim();
        val = val.replace(/^["']|["']$/g, '');
        directives[key] = val;
      }
    } else if (trimmed) {
      dataLines.push(trimmed);
    }
  }

  if (dataLines.length === 0) {
    const emptyErr = new Error('Tabela não contém dados.');
    emptyErr.status = 400;
    throw emptyErr;
  }

  const firstLine = dataLines[0];
  let isMarkdownPipe = false;
  let delimiter = ',';

  if (firstLine.includes('|')) {
    isMarkdownPipe = true;
  } else if (firstLine.includes('\t')) {
    delimiter = '\t';
  }

  const rows = [];

  if (isMarkdownPipe) {
    for (const line of dataLines) {
      if (/^\s*\|?\s*:?-+:?\s*(\|?\s*:?-+:?\s*)+\|?\s*$/.test(line)) {
        continue;
      }
      const rawCols = line.split('|');
      if (line.trim().startsWith('|')) rawCols.shift();
      if (line.trim().endsWith('|')) rawCols.pop();
      const cells = rawCols.map((c) => c.trim());
      if (cells.length > 0) rows.push(cells);
    }
  } else {
    for (const line of dataLines) {
      const cells = [];
      let current = '';
      let inQuotes = false;
      for (let i = 0; i < line.length; i++) {
        const char = line[i];
        if (char === '"' || char === "'") {
          inQuotes = !inQuotes;
        } else if (char === delimiter && !inQuotes) {
          cells.push(current.trim().replace(/^["']|["']$/g, ''));
          current = '';
        } else {
          current += char;
        }
      }
      cells.push(current.trim().replace(/^["']|["']$/g, ''));
      if (cells.length > 0) rows.push(cells);
    }
  }

  if (rows.length === 0) {
    const emptyErr = new Error('Nenhuma linha válida encontrada na tabela.');
    emptyErr.status = 400;
    throw emptyErr;
  }

  return { directives, rows };
}

function renderSingleTable(rawContent) {
  const { directives, rows } = parseTableData(rawContent);
  const title = directives.title || directives.titulo || directives.caption || '';
  const note = directives.note || directives.fonte || directives.source || '';
  const alignSpec = directives.align || '';
  const width = directives['layout.width'] || directives.width || 'auto';

  const alignments = alignSpec
    ? alignSpec.split(/[,|\s]+/).map((a) => {
        const code = a.trim().toLowerCase()[0];
        return code === 'c' ? 'center' : code === 'r' ? 'right' : 'left';
      })
    : [];

  const headers = rows[0];
  const dataRows = rows.slice(1);

  const escapeHtml = (str) =>
    String(str)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;');

  let html = `<div class="joaju-academic-table-wrapper" style="width: ${width};">\n`;
  if (title) {
    html += `  <div class="joaju-table-title">${escapeHtml(title)}</div>\n`;
  }
  html += `  <table class="joaju-academic-table">\n`;
  html += '    <thead>\n      <tr>\n';
  for (let i = 0; i < headers.length; i++) {
    const align = alignments[i] || 'left';
    html += `        <th style="text-align: ${align};">${escapeHtml(headers[i])}</th>\n`;
  }
  html += '      </tr>\n    </thead>\n    <tbody>\n';
  for (const row of dataRows) {
    html += '      <tr>\n';
    for (let i = 0; i < headers.length; i++) {
      const cell = row[i] !== undefined ? row[i] : '';
      const align = alignments[i] || 'left';
      html += `        <td style="text-align: ${align};">${escapeHtml(cell)}</td>\n`;
    }
    html += '      </tr>\n';
  }
  html += '    </tbody>\n  </table>\n';
  if (note) {
    html += `  <div class="joaju-table-note">${escapeHtml(note)}</div>\n`;
  }
  html += '</div>';

  return html;
}

// --- Pré-processador 3.5: Inclusão de Gists do Fuxico ---

/**
 * Cache em memória de conteúdo de gists para evitar fetches redundantes por sessão.
 * Chave: `${url}|${file}` — Valor: string com o conteúdo ou null (falha).
 */
const _gistCache = new Map();

/**
 * Resolve a URL raw de um gist do Fuxico.
 * Se `file` for especificado, usa `{url}/raw/{file}`; caso contrário `{url}/raw`.
 *
 * @param {string} baseUrl  - URL base do gist, ex.: https://2lp.in/abc123
 * @param {string} file     - Nome do arquivo específico (opcional)
 * @returns {string}
 */
function buildGistRawUrl(baseUrl, file) {
  const base = baseUrl.replace(/\/+$/, '');
  return file ? `${base}/raw/${encodeURIComponent(file)}` : `${base}/raw`;
}

/**
 * Infere a linguagem de realce de sintaxe a partir da extensão do arquivo.
 *
 * @param {string} file  - Nome do arquivo ou URL, pode ser vazio
 * @returns {string}     - Linguagem para Pandoc/Typst (ex.: 'r', 'python', '')
 */
function inferGistLang(file) {
  if (!file) return '';
  const ext = file.split('.').pop().toLowerCase();
  const map = {
    r: 'r',
    rmd: 'r',
    qmd: 'r',
    py: 'python',
    ipynb: 'python',
    jl: 'julia',
    sql: 'sql',
    sh: 'bash',
    bash: 'bash',
    zsh: 'bash',
    js: 'javascript',
    ts: 'typescript',
    mjs: 'javascript',
    json: 'json',
    yaml: 'yaml',
    yml: 'yaml',
    toml: 'toml',
    md: 'markdown',
    markdown: 'markdown',
    tex: 'latex',
    rs: 'rust',
    go: 'go',
    php: 'php',
    c: 'c',
    cpp: 'cpp',
    h: 'c',
    java: 'java',
    kt: 'kotlin',
    lua: 'lua',
    typ: 'typst'
  };
  return map[ext] || '';
}

/**
 * Pré-processador de blocos ```gist``` — busca o código-fonte do Fuxico e
 * o transforma em bloco Typst (#raw) para PDF ou bloco de código Markdown
 * anotado para HTML.
 *
 * Sintaxe no Markdown:
 * ```gist
 * %% url=https://2lp.in/{slug}
 * %% file=script.R          (opcional — omitir para exibir primeiro arquivo)
 * %% title=Legenda do bloco (opcional)
 * ```
 *
 * Resiliência: se o fetch falhar, emite aviso no log e renderiza card de fallback
 * sem abortar a compilação.
 */
async function preprocessGists(
  markdown,
  sessionId,
  workspaceDir = null,
  fetchFn = globalThis.fetch
) {
  const regex = /```gist\n([\s\S]*?)\n```/gi;
  let match;
  let modifiedMarkdown = markdown;

  while ((match = regex.exec(markdown)) !== null) {
    const rawBlock = match[1];

    // --- Leitura de diretivas %%
    const urlMatch = rawBlock.match(/%%\s*url=([^\s\n]+)/);
    const fileMatch = rawBlock.match(/%%\s*file=([^\s\n]+)/);
    const titleMatch = rawBlock.match(/%%\s*(?:title|titulo|caption)=(.+)/);

    if (!urlMatch) {
      console.warn('[Joaju/gist] Bloco gist sem diretiva %% url= — ignorado.');
      continue;
    }

    const gistUrl = urlMatch[1].trim().replace(/^["']|["']$/g, '');
    const gistFile = fileMatch ? fileMatch[1].trim().replace(/^["']|["']$/g, '') : '';
    const title = titleMatch ? titleMatch[1].trim() : '';
    const lang = inferGistLang(gistFile);
    const cacheKey = `${gistUrl}|${gistFile}`;

    // --- Cache
    let content;
    if (_gistCache.has(cacheKey)) {
      content = _gistCache.get(cacheKey);
    } else {
      const rawUrl = buildGistRawUrl(gistUrl, gistFile);
      try {
        const res = await fetchFn(rawUrl, { signal: AbortSignal.timeout(10000) });
        content = res.ok ? await res.text() : null;
        if (!res.ok) {
          console.warn(`[Joaju/gist] Fetch falhou (${res.status}) para ${rawUrl}`);
        }
      } catch (err) {
        console.warn(`[Joaju/gist] Erro de rede ao buscar gist: ${err.message}`);
        content = null;
      }
      _gistCache.set(cacheKey, content);
    }

    let replacement;

    if (content === null) {
      // --- Fallback: card de aviso sem abortar
      const displayUrl = gistFile ? `${gistUrl} (${gistFile})` : gistUrl;
      replacement = `\`\`\`{=typst}
#block(
  fill: rgb("#fff3cd"),
  stroke: 1pt + rgb("#856404"),
  radius: 4pt,
  inset: 10pt,
  width: 100%
)[
  #text(weight: "bold")[⚠ Gist indisponível] \\
  #link("${gistUrl}")[${escapeTypst(displayUrl)}]
]
\`\`\``;
    } else {
      // --- Renderização bem-sucedida
      const label = gistFile || gistUrl.split('/').pop();
      const langAttr = lang ? `, lang: "${lang}"` : '';
      const captionBlock = title
        ? `caption: [${escapeTypst(title)}],\n      supplement: [Código],`
        : '';
      const noteBlock = `#v(-4pt)\n  #align(left)[#text(size: 8pt)[_Fonte: #link("${gistUrl}")[${escapeTypst(label)}]_]]`;

      replacement = `\`\`\`{=typst}
#align(center)[
  #block(width: 100%)[
    #figure(
      ${captionBlock}
      raw(read("${label}") , block: true${langAttr})
    )
    ${noteBlock}
  ]
]
\`\`\``;

      // Para Typst, injeta o conteúdo via arquivo temporário na sessão
      // (Pandoc/Typst não aceita raw() inline com strings longas seguramente)
      // → embutimos como raw literal escapado
      const escaped = content.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/\r?\n/g, '\\n');

      replacement = `\`\`\`{=typst}
#align(center)[
  #block(width: 100%)[
    #figure(
      ${captionBlock}
      raw("${escaped}", block: true${langAttr})
    )
    ${noteBlock}
  ]
]
\`\`\``;
    }

    modifiedMarkdown = modifiedMarkdown.replace(match[0], replacement);
  }

  return modifiedMarkdown;
}

async function preprocessTables(markdown, sessionId, workspaceDir = null) {
  const regex = /```table\n([\s\S]*?)\n```/gi;
  let match;
  let modifiedMarkdown = markdown;

  while ((match = regex.exec(markdown)) !== null) {
    let rawContent = match[1].trim();
    rawContent = await resolveExternalSource(rawContent, workspaceDir);

    try {
      const { directives, rows } = parseTableData(rawContent);
      const title = directives.title || directives.titulo || directives.caption || '';
      const note = directives.note || directives.fonte || directives.source || '';
      const alignSpec = directives.align || '';
      const rawWidth = directives['layout.width'] || directives.width || '100%';
      const width = normalizeDimension(rawWidth, '100%');

      const headers = rows[0];
      const dataRows = rows.slice(1);
      const numCols = headers.length;

      const alignmentsTypst = [];
      const alignCodes = alignSpec ? alignSpec.split(/[,|\s]+/) : [];
      for (let i = 0; i < numCols; i++) {
        const code = (alignCodes[i] || 'l').trim().toLowerCase()[0];
        alignmentsTypst.push(code === 'c' ? 'center' : code === 'r' ? 'right' : 'left');
      }

      const totalRows = dataRows.length + 1;
      const colDef =
        directives.columns || directives.colunas || Array(numCols).fill('1fr').join(', ');

      const formatCell = (val) => {
        if (!val) return '';
        let s = String(val).trim();
        // Converte expressões matemáticas simples $...$ tratando identificadores multiletras como strings no Typst
        s = s.replace(/\$([^$]+)\$/g, (m, math) => {
          const tm = math.trim();
          // Trata casos específicos do Typst como ii -> "ii", B_ -> B\_
          if (tm === 'ii') return '$"ii"$';
          if (tm === 'bb') return '$"bb"$';
          if (tm === 'Bb') return '$"Bb"$';
          if (tm === 'BB') return '$"BB"$';
          if (tm === 'B_') return '$B\\_$';
          return `$${tm}$`;
        });
        s = s.replace(/\*\*([^*]+)\*\*/g, '*$1*');
        s = s.replace(/(?<!\\)\[/g, '\\[').replace(/(?<!\\)\]/g, '\\]');
        return s;
      };

      const headerCells = headers.map((h) => `[*${formatCell(h)}*]`).join(', ');

      const bodyCells = dataRows
        .map((row) => {
          const cells = [];
          for (let i = 0; i < numCols; i++) {
            const val = row[i] !== undefined ? row[i] : '';
            cells.push(`[${formatCell(val)}]`);
          }
          return cells.join(', ');
        })
        .join(',\n        ');

      const captionBlock = title ? `caption: [${escapeTypst(title)}],` : '';

      const typstTable = `
\`\`\`{=typst}
#align(center)[
  #block(width: ${width})[
    #figure(
      ${captionBlock}
      table(
        columns: (${colDef}),
        align: (${alignmentsTypst.join(', ')}),
        stroke: (x, y) => if y == 0 { (top: 1.5pt + black, bottom: 0.8pt + black) } else if y == ${totalRows} - 1 { (bottom: 1.5pt + black) } else { none },
        table.header(${headerCells}),
        ${bodyCells}
      )
    )
    ${note ? `#v(-3pt)\n    #align(left)[#text(size: 8pt)[_${escapeTypst(note)}_]]` : ''}
  ]
]
\`\`\`
`;
      modifiedMarkdown = modifiedMarkdown.replace(match[0], typstTable);
    } catch (err) {
      console.error(`Erro ao compilar tabela acadêmica: ${err.message}`);
    }
  }
  return modifiedMarkdown;
}

// --- Pré-processador 5: Imagens e Wikilinks do Obsidian ---
function findImageInWorkspace(ref, workspaceDir) {
  if (!workspaceDir || !ref) return null;
  const cleanRef = ref.trim().replace(/^["']|["']$/g, '');
  if (
    cleanRef.startsWith('http://') ||
    cleanRef.startsWith('https://') ||
    cleanRef.startsWith('data:')
  ) {
    return null;
  }

  const resolvedWs = path.resolve(workspaceDir);

  // 1. Caminho direto relativo ou absoluto
  const directPath = path.resolve(workspaceDir, cleanRef);
  if (directPath.startsWith(resolvedWs) && fs.existsSync(directPath)) {
    return path.relative(resolvedWs, directPath);
  }

  // 2. Procura nas subpastas canônicas de anexos
  const baseName = path.basename(cleanRef);
  const commonFolders = ['attachments', 'anexos', '99_anexo', 'assets', 'images', 'img'];
  for (const folder of commonFolders) {
    const candidate = path.resolve(workspaceDir, folder, baseName);
    if (candidate.startsWith(resolvedWs) && fs.existsSync(candidate)) {
      return path.relative(resolvedWs, candidate);
    }
  }

  // 3. Busca recursiva segura dentro do workspaceDir
  const searchRecursive = (dir) => {
    try {
      const entries = fs.readdirSync(dir, { withFileTypes: true });
      for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);
        if (entry.isFile() && entry.name === baseName) {
          return path.relative(resolvedWs, fullPath);
        }
        if (entry.isDirectory() && !entry.name.startsWith('.')) {
          const found = searchRecursive(fullPath);
          if (found) return found;
        }
      }
    } catch (_) {}
    return null;
  };

  return searchRecursive(resolvedWs);
}

async function preprocessImages(markdown, sessionId, workspaceDir = null) {
  if (!markdown || typeof markdown !== 'string') return markdown;

  let modifiedMarkdown = markdown;
  let imgIndex = 0;

  const resolveImageSource = async (ref) => {
    const cleanRef = ref.trim().replace(/^["']|["']$/g, '');
    if (cleanRef.startsWith('http://') || cleanRef.startsWith('https://')) {
      try {
        const response = await fetch(cleanRef, { signal: AbortSignal.timeout(10000) });
        if (response.ok) {
          const buffer = Buffer.from(await response.arrayBuffer());
          const extMatch = cleanRef.match(/\.(png|jpe?g|gif|webp|svg)/i);
          const ext = extMatch ? extMatch[1].toLowerCase() : 'png';
          const localImgPath = path.join(
            workspaceDir || os.tmpdir(),
            `remote_img_${sessionId}_${imgIndex++}.${ext}`
          );
          fs.writeFileSync(localImgPath, buffer);
          return workspaceDir ? path.relative(workspaceDir, localImgPath) : localImgPath;
        }
      } catch (err) {
        console.warn(`[Joaju] Falha ao baixar imagem remota "${cleanRef}": ${err.message}`);
      }
      return cleanRef;
    }

    if (workspaceDir) {
      const found = findImageInWorkspace(cleanRef, workspaceDir);
      if (found) return found;
    }

    return cleanRef;
  };

  // 1. Normaliza Wikilinks do Obsidian: ![[imagem.png]] ou ![[imagem.png|legenda]] ou ![[imagem.png|400]]
  const wikiRegex = /!\[\[([^\]|#]+)(?:#[^\]|]*)?(?:\|([^\]]*))?\]\]/g;
  const wikiMatches = [...modifiedMarkdown.matchAll(wikiRegex)];
  for (const match of wikiMatches) {
    const fullMatch = match[0];
    const rawRef = match[1];
    const rawAlias = match[2];
    const resolvedPath = await resolveImageSource(rawRef);

    let replacement = `![](${resolvedPath})`;
    if (rawAlias?.trim()) {
      const trimmedAlias = rawAlias.trim();
      const dimMatch = trimmedAlias.match(/^(\d+)(?:x\d+)?$/);
      if (dimMatch) {
        replacement = `![](${resolvedPath}){width=${dimMatch[1]}pt}`;
      } else {
        replacement = `![${trimmedAlias}](${resolvedPath})`;
      }
    }
    modifiedMarkdown = modifiedMarkdown.replace(fullMatch, replacement);
  }

  // 2. Normaliza imagens Markdown padrão se o arquivo estiver em subpasta do workspace
  if (workspaceDir) {
    const mdImgRegex = /!\[([^\]]*)\]\(([^)]+)\)/g;
    const mdMatches = [...modifiedMarkdown.matchAll(mdImgRegex)];
    for (const match of mdMatches) {
      const fullMatch = match[0];
      const alt = match[1];
      const rawRef = match[2];
      const cleanRef = rawRef.trim().replace(/^["']|["']$/g, '');
      if (
        !cleanRef.startsWith('http://') &&
        !cleanRef.startsWith('https://') &&
        !cleanRef.startsWith('data:')
      ) {
        const found = findImageInWorkspace(cleanRef, workspaceDir);
        if (found && found !== cleanRef) {
          modifiedMarkdown = modifiedMarkdown.replace(fullMatch, `![${alt}](${found})`);
        }
      }
    }
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
      const directiveRegex =
        /([\w.-]+)\s*=\s*(c\((?:[^()]*|\([^()]*\))*\)|"(?:\\.|[^"\\])*"|'[^']*'|[^\s]+)/g;
      let dMatch;
      while ((dMatch = directiveRegex.exec(directiveText)) !== null) {
        const key = dMatch[1];
        const val = dMatch[2];
        if (
          key !== 'layout.width' &&
          key !== 'width' &&
          key !== 'layout.max_width' &&
          key !== 'max-width' &&
          key !== 'src' &&
          key !== 'file'
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

function renderSinglePedtools(rawContent, sessionId = 'preview') {
  const csvPath = `/tmp/pedtools_${sessionId}.csv`;
  const svgPath = `/tmp/pedtools_${sessionId}.svg`;

  const rArgs = [];
  const lines = rawContent.split('\n');
  const csvLines = [];

  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.startsWith('%%')) {
      const directiveText = trimmed.replace(/^%%\s*/, '');
      const directiveRegex =
        /([\w.-]+)\s*=\s*(c\((?:[^()]*|\([^()]*\))*\)|"(?:\\.|[^"\\])*"|'[^']*'|[^\s]+)/g;
      let dMatch;
      while ((dMatch = directiveRegex.exec(directiveText)) !== null) {
        const key = dMatch[1];
        const val = dMatch[2];
        if (
          key !== 'layout.width' &&
          key !== 'width' &&
          key !== 'layout.max_width' &&
          key !== 'max-width' &&
          key !== 'src' &&
          key !== 'file'
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
  const scriptPath = fs.existsSync('/app/plot_pedtools.R')
    ? '/app/plot_pedtools.R'
    : path.join(__dirname, 'plot_pedtools.R');

  try {
    cp.execSync(
      `Rscript "${scriptPath}" "${csvPath}" "${svgPath}" "${rArgsString.replace(/"/g, '\\"')}"`,
      { stdio: ['pipe', 'pipe', 'pipe'] }
    );
    if (fs.existsSync(svgPath)) {
      const svgContent = fs.readFileSync(svgPath, 'utf8');
      return svgContent;
    }
    const missingSvgErr = new Error('Arquivo SVG não gerado pelo Rscript (pedtools)');
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
      const pedError = new Error(`Erro no heredograma (pedtools): ${cleanMsg}`);
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

function renderSingleOpenchemlib(rawContent) {
  const lines = rawContent.split('\n');
  const codeLines = [];
  let width = 400;
  let height = 300;

  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.startsWith('%%')) {
      const wMatch = trimmed.match(/width=(\d+)/i);
      const hMatch = trimmed.match(/height=(\d+)/i);
      if (wMatch) width = Number.parseInt(wMatch[1], 10);
      if (hMatch) height = Number.parseInt(hMatch[1], 10);
    } else if (trimmed && !trimmed.startsWith('#')) {
      codeLines.push(trimmed);
    }
  }

  const smiles = codeLines.join(' ').trim();
  if (!smiles) {
    const emptyErr = new Error('Código SMILES está vazio.');
    emptyErr.status = 400;
    throw emptyErr;
  }

  try {
    const mol = OCL.Molecule.fromSmiles(smiles);
    const svg = mol.toSVG(width, height);
    return svg;
  } catch (err) {
    const chemErr = new Error(`Erro na molécula (openchemlib): ${err.message}`);
    chemErr.status = 422;
    throw chemErr;
  }
}

function renderSingleOpenbabel(rawContent, sessionId = 'preview') {
  const lines = rawContent.split('\n');
  const codeLines = [];
  let size = 400;

  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.startsWith('%%')) {
      const sMatch = trimmed.match(/(?:width|size)=(\d+)/i);
      if (sMatch) size = Number.parseInt(sMatch[1], 10);
    } else if (trimmed && !trimmed.startsWith('#')) {
      codeLines.push(trimmed);
    }
  }

  const molStr = codeLines.join('\n').trim();
  if (!molStr) {
    const emptyErr = new Error('Conteúdo químico está vazio.');
    emptyErr.status = 400;
    throw emptyErr;
  }

  const inputExt = molStr.startsWith('InChI=') ? 'inchi' : 'smi';
  const inPath = `/tmp/ob_${sessionId}.${inputExt}`;
  const outSvg = `/tmp/ob_${sessionId}.svg`;

  fs.writeFileSync(inPath, molStr);

  try {
    cp.execSync(`obabel "${inPath}" -O "${outSvg}" -xP ${size} -d`, {
      stdio: ['pipe', 'pipe', 'pipe']
    });
    if (fs.existsSync(outSvg)) {
      const svg = fs.readFileSync(outSvg, 'utf8');
      return svg;
    }
    const missingErr = new Error('Arquivo SVG não gerado pelo Open Babel');
    missingErr.status = 422;
    throw missingErr;
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
    const obError = new Error(`Erro no Open Babel: ${cleanMsg}`);
    obError.status = 422;
    throw obError;
  } finally {
    try {
      if (fs.existsSync(inPath)) fs.unlinkSync(inPath);
      if (fs.existsSync(outSvg)) fs.unlinkSync(outSvg);
    } catch (_) {}
  }
}

// --- Rota de Preview Rápido para a Interface Web ---
app.post('/render', async (req, res) => {
  const rawInput = (req.body.code || req.body.markdown || '').trim();
  if (!rawInput) {
    return res.status(400).json({ error: 'Nenhum código fornecido para renderização.' });
  }

  // 1. Tenta extrair bloco demarcado por ```
  const blockMatch = rawInput.match(
    /```(openchemlib|openbabel|table|pedtools|kinship2|graphviz|plantuml|d2|mermaid)\n([\s\S]*?)\n?```/i
  );
  let type = 'unknown';
  let requestedFormat = 'unknown';
  let content = rawInput;

  if (blockMatch) {
    const tag = blockMatch[1].toLowerCase();
    if (tag === 'openchemlib') {
      type = 'openchemlib';
      requestedFormat = 'openchemlib';
    } else if (tag === 'openbabel') {
      type = 'openbabel';
      requestedFormat = 'openbabel';
    } else if (tag === 'table') {
      type = 'table';
      requestedFormat = 'table';
    } else if (tag === 'pedtools') {
      type = 'pedtools';
      requestedFormat = 'pedtools';
    } else if (tag === 'kinship2') {
      type = 'kinship2';
      requestedFormat = 'kinship2';
    } else if (tag === 'graphviz') {
      type = 'graphviz';
      requestedFormat = 'graphviz';
    } else if (tag === 'plantuml') {
      type = 'plantuml';
      requestedFormat = 'plantuml';
    } else if (tag === 'd2') {
      type = 'd2';
      requestedFormat = 'd2';
    } else if (tag === 'mermaid') {
      type = 'mermaid';
      requestedFormat = 'mermaid';
    }
    content = blockMatch[2].trim();
  } else {
    const anyFenceMatch = rawInput.trim().match(/^```([a-zA-Z0-9_-]+)/);
    if (anyFenceMatch) {
      return res.status(400).json({
        error: `Motor '${anyFenceMatch[1]}' não suportado. Motores válidos: pedtools, kinship2, openchemlib, openbabel, table, d2, plantuml, graphviz, mermaid.`
      });
    }

    // 2. Inferência heurística se o usuário colar apenas o miolo
    if (/^\s*(digraph|strict digraph|graph|subgraph)\b/m.test(rawInput)) {
      type = 'graphviz';
      requestedFormat = 'graphviz';
    } else if (/^\s*(@start|class |actor |participant |autonumber)/m.test(rawInput)) {
      type = 'plantuml';
      requestedFormat = 'plantuml';
    } else if (
      /^(flowchart|sequenceDiagram|classDiagram|stateDiagram|erDiagram|gantt|pie|gitGraph|journey)/m.test(
        rawInput
      ) ||
      rawInput.includes('-->') ||
      rawInput.includes('graph TD') ||
      rawInput.includes('graph LR')
    ) {
      type = 'mermaid';
      requestedFormat = 'mermaid';
    } else if (/^\s*[\w\d_-]+\s*(?:->|<-|--|<->)\s*[\w\d_-]+/m.test(rawInput)) {
      type = 'd2';
      requestedFormat = 'd2';
    } else if (
      /^(id\s*,|pai\s*,|mae\s*,|dadid\s*,|momid\s*,)/im.test(rawInput) ||
      /^(\d+)\s*,\s*(\d+|\bNA\b)\s*,\s*(\d+|\bNA\b)\s*,/m.test(rawInput)
    ) {
      type = 'kinship2';
      requestedFormat = 'kinship2';
    } else {
      type = rawInput.includes(',') ? 'kinship2' : 'graphviz';
      requestedFormat = type;
    }
  }

  // Resolve fonte remota %% src=http... se houver
  content = await resolveExternalSource(content);

  const sessionId = uuid.v4();

  try {
    if (type === 'openchemlib') {
      const svg = renderSingleOpenchemlib(content);
      return res.json({ format: 'openchemlib', type: 'svg', svg });
    }
    if (type === 'openbabel') {
      const svg = renderSingleOpenbabel(content, sessionId);
      return res.json({ format: 'openbabel', type: 'svg', svg });
    }
    if (type === 'table') {
      const html = renderSingleTable(content);
      return res.json({ format: 'table', type: 'html', html });
    }
    if (type === 'pedtools') {
      const svg = renderSinglePedtools(content, sessionId);
      return res.json({ format: 'pedtools', type: 'svg', svg });
    }
    if (type === 'kinship2') {
      const svg = renderSingleKinship(content, sessionId);
      return res.json({ format: requestedFormat, type: 'svg', svg });
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
  const id = uuid.v4();
  const isZip = Buffer.isBuffer(req.body) && req.body.length > 0;
  let workspaceDir = null;
  let markdown = '';

  try {
    if (isZip) {
      workspaceDir = path.join(os.tmpdir(), `job_${id}`);
      fs.mkdirSync(workspaceDir, { recursive: true });
      const zipPath = path.join(workspaceDir, `bundle_${id}.zip`);
      fs.writeFileSync(zipPath, req.body);
      try {
        cp.execSync(`unzip -q -o "${zipPath}" -d "${workspaceDir}"`);
      } finally {
        if (fs.existsSync(zipPath)) {
          try {
            fs.unlinkSync(zipPath);
          } catch (_) {}
        }
      }

      const candidates = ['main.md', 'index.md', 'document.md'];
      let entryFile = null;
      for (const cand of candidates) {
        const p = path.join(workspaceDir, cand);
        if (fs.existsSync(p)) {
          entryFile = p;
          break;
        }
      }

      if (!entryFile) {
        const rootFiles = fs.readdirSync(workspaceDir);
        const mdFiles = rootFiles.filter(
          (f) => f.endsWith('.md') && fs.statSync(path.join(workspaceDir, f)).isFile()
        );
        if (mdFiles.length > 0) {
          entryFile = path.join(workspaceDir, mdFiles[0]);
        }
      }

      if (!entryFile) {
        return res
          .status(400)
          .json({ error: 'Nenhum arquivo Markdown (.md) encontrado dentro do arquivo ZIP.' });
      }

      markdown = fs.readFileSync(entryFile, 'utf8');
    } else if (typeof req.body?.markdown === 'string') {
      markdown = req.body.markdown;
    } else if (typeof req.body === 'string') {
      markdown = req.body;
    }

    if (!markdown || markdown.trim().length === 0) {
      return res
        .status(400)
        .json({ error: 'Conteúdo Markdown está vazio ou não pôde ser lido do arquivo/ZIP.' });
    }

    const baseDir = workspaceDir || os.tmpdir();
    const tempMd = path.join(baseDir, `input_${id}.md`);
    const tempTypstOutput = path.join(baseDir, `conteudo_${id}.typ`);
    const tempWrapperTypst = path.join(baseDir, `main_${id}.typ`);
    const tempPdf = path.join(baseDir, `output_${id}.pdf`);

    // 🔍 Extrai metadados do YAML para automação bibliográfica
    // Procura o perfil (abnt, vancouver, etc.) condicionalmente
    const metaMatch = markdown.match(/^---[\s\S]*?perfil:\s*([a-zA-Z0-9_-]+)[\s\S]*?---/);
    const profile = metaMatch ? metaMatch[1] : 'abnt';

    // 🔄 ORQUESTRADORES DE PRÉ-PROCESSAMENTO
    let cleanMarkdown = await preprocessPedigrees(markdown, id, workspaceDir);
    cleanMarkdown = await preprocessDiagrams(cleanMarkdown, id, workspaceDir);
    cleanMarkdown = await preprocessChemistry(cleanMarkdown, id, workspaceDir);
    cleanMarkdown = await preprocessGists(cleanMarkdown, id, workspaceDir);
    cleanMarkdown = await preprocessTables(cleanMarkdown, id, workspaceDir);
    cleanMarkdown = await preprocessImages(cleanMarkdown, id, workspaceDir);

    // Suporte genérico a blocos nativos: ```typst -> <typst>
    cleanMarkdown = cleanMarkdown.replace(/```typst\n([\s\S]*?)\n```/g, '$1');

    // Normalização de quebras de linha em listas: garante linha em branco antes de listas para o Pandoc
    cleanMarkdown = cleanMarkdown.replace(/([^\n])\n(\d+\.\s+|[-*]\s+)/g, '$1\n\n$2');

    fs.writeFileSync(tempMd, cleanMarkdown);

    // 1️⃣ ETAPA 1: Pandoc + Citeproc (Maturação Bibliográfica)
    const resourcePathArg = workspaceDir ? `--resource-path="${workspaceDir}"` : '';
    const pandocCmd = `pandoc "${tempMd}" ${resourcePathArg} --citeproc --bibliography=/data/referencias.bib --csl=/data/${profile}.csl -t typst -o "${tempTypstOutput}"`;

    cp.execSync(pandocCmd);

    // Injeta suporte a réguas horizontais do Pandoc e função min no escopo do arquivo Typst
    if (fs.existsSync(tempTypstOutput)) {
      let originalTypst = fs.readFileSync(tempTypstOutput, 'utf8');
      // Remove âncoras/labels geradas automaticamente pelo Pandoc em títulos com caracteres acentuados
      // (ex: <introdução-...> ou <análise-...>) que contêm caracteres não-ASCII e vazam como texto no Typst
      originalTypst = originalTypst.replace(/<[^\s<>]*[^\u0020-\u007E][^\s<>]*>/gu, '');
      // Escapa caracteres '<' que não compõem labels válidos do Typst para evitar 'unclosed label'
      originalTypst = originalTypst.replace(/(?<!\\)<(?![a-zA-Z0-9_\-:]+>)/g, '\\<');

      // Torna as tabelas do Pandoc proporcionais e autoajustáveis:
      originalTypst = originalTypst.replace(
        /#table\(\s*columns:\s*\([^)]+\),([^\n]*\n\s*align:\s*\([^)]+\),\s*table\.header\(\[Código\],\s*\[Filiação\],\s*\[Sexo\],\s*\[Fenótipo ABO\])/g,
        '#table(columns: (auto, auto, auto, auto, 1fr), inset: 4.5pt,$1'
      );
      originalTypst = originalTypst.replace(
        /#table\(\s*columns:\s*\([^)]+\),([^\n]*\n\s*align:\s*\([^)]+\),\s*table\.header\(\[Código\],\s*\[Filiação\],\s*\[Sexo\],\s*\[Fenótipo\s*Observado\])/g,
        '#table(columns: (auto, auto, auto, auto, auto, 1fr), inset: 4.5pt,$1'
      );
      originalTypst = originalTypst.replace(
        /#table\(\s*columns:\s*\([^)]+\),([^\n]*\n\s*align:\s*\([^)]+\),\s*table\.header\(\[Código\],\s*\[Filiação\],\s*\[Sexo\],\s*\[Condição Relatada)/g,
        '#table(columns: (auto, auto, auto, 1.4fr, 1.2fr), inset: 4.5pt,$1'
      );

      const patchedTypst = `#let horizontalrule = line(length: 100%, stroke: 0.5pt + luma(180))\n#let min(a, b) = if type(a) == ratio { a } else if type(b) == ratio { b } else if type(a) == int { calc.min(a * 1pt, b) } else if type(b) == int { calc.min(a, b * 1pt) } else { calc.min(a, b) }\n${originalTypst}`;
      fs.writeFileSync(tempTypstOutput, patchedTypst);
    }

    // 2️⃣ ETAPA 2: Typst (Diagramação Rápida)
    const tituloMatch = markdown.match(/^---[\s\S]*?titulo:\s*"?([^"\n]+?)"?\s*$/m);
    const autorMatch = markdown.match(/^---[\s\S]*?autor:\s*"?([^"\n]+?)"?\s*$/m);
    const subtituloMatch = markdown.match(/^---[\s\S]*?subtitulo:\s*"?([^"\n]+?)"?\s*$/m);
    const docTitulo = tituloMatch ? tituloMatch[1].trim() : 'Trabalho Acadêmico';
    const docAutor = autorMatch ? autorMatch[1].trim() : 'Discente';
    const docSubtitulo = subtituloMatch ? subtituloMatch[1].trim() : '';

    let templateImport = '#import "/data/template.typ": abnt\n#show: doc => abnt(doc)';
    if (profile === 'trabalho-ufsc' && fs.existsSync('/data/trabalho-ufsc.typ')) {
      const subArg = docSubtitulo ? `, subtitulo: "${docSubtitulo.replace(/"/g, '\\"')}"` : '';
      templateImport = `#import "/data/trabalho-ufsc.typ": trabalho_ufsc\n#show: doc => trabalho_ufsc(doc, titulo: "${docTitulo.replace(/"/g, '\\"')}", autor: "${docAutor.replace(/"/g, '\\"')}"${subArg})`;
    }

    const typstWrapper = `
${templateImport}
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
  } catch (err) {
    console.error(`Erro fatal no pipeline: ${err.message}`);
    res.status(500).send(`Erro na compilação do artigo: ${err.message}`);
  } finally {
    if (workspaceDir && fs.existsSync(workspaceDir)) {
      try {
        fs.rmSync(workspaceDir, { recursive: true, force: true });
      } catch (_) {}
    } else {
      const baseDir = os.tmpdir();
      for (const f of [
        path.join(baseDir, `input_${id}.md`),
        path.join(baseDir, `conteudo_${id}.typ`),
        path.join(baseDir, `main_${id}.typ`),
        path.join(baseDir, `output_${id}.pdf`)
      ]) {
        if (fs.existsSync(f)) {
          try {
            fs.unlinkSync(f);
          } catch (_) {}
        }
      }
    }
  }
});

const { unfurlUrl } = require('./unfurl');

// 🌐 Endpoint de Unfurl, Metadados e Snapshot Web
app.post('/unfurl', async (req, res) => {
  const { url, archive } = req.body || {};
  if (!url || typeof url !== 'string') {
    return res.status(400).json({ ok: false, error: 'Parâmetro "url" é obrigatório.' });
  }

  try {
    const result = await unfurlUrl(url.trim(), { archive: Boolean(archive) });
    if (!result.ok) {
      return res.status(422).json(result);
    }
    return res.json(result);
  } catch (err) {
    console.error(`[Joaju Server] Erro em /unfurl: ${err.message}`);
    return res.status(500).json({ ok: false, error: err.message });
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
  preprocessChemistry,
  preprocessGists,
  preprocessTables,
  preprocessImages,
  findImageInWorkspace,
  resolveExternalSource,
  parseTableData,
  renderSingleTable,
  renderSingleOpenchemlib,
  renderSingleOpenbabel,
  renderSinglePedigree,
  renderSingleKinship,
  renderSinglePedtools,
  renderSingleGraphviz,
  renderSinglePlantuml,
  renderSingleD2,
  renderSingleMermaid,
  buildGistRawUrl,
  inferGistLang,
  unfurlUrl
};
