const cp = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
let parseHTML = null;
let Readability = null;
try {
  ({ parseHTML } = require('linkedom'));
  ({ Readability } = require('@mozilla/readability'));
} catch (e) {
  // optional dependency
}
const uuid = require('uuid');

/**
 * Escapa caracteres reservados do Typst para inserção segura em strings de markup.
 * @param {string} str
 * @returns {string}
 */
function escapeTypst(str) {
  if (!str) return '';
  return String(str).replace(/[\\#$[\]@_`*]/g, (match) => `\\${match}`);
}

/**
 * Resolve uma URL relativa para absoluta com base na URL de origem.
 * @param {string} relativeOrAbsolute
 * @param {string} baseUrl
 * @returns {string|null}
 */
function resolveUrl(relativeOrAbsolute, baseUrl) {
  if (!relativeOrAbsolute) return null;
  try {
    return new URL(relativeOrAbsolute, baseUrl).href;
  } catch {
    return null;
  }
}

/**
 * Extrai metadados completos de Open Graph, Twitter Cards e semântica HTML.
 * @param {Document} document
 * @param {string} targetUrl
 * @returns {object}
 */
function extractMetadata(document, targetUrl) {
  const getMeta = (selector, attr = 'content') => {
    const el = document.querySelector(selector);
    return el ? el.getAttribute(attr)?.trim() || null : null;
  };

  const domain = new URL(targetUrl).hostname.replace(/^www\./, '');

  const title =
    getMeta('meta[property="og:title"]') ||
    getMeta('meta[name="twitter:title"]') ||
    getMeta('meta[name="title"]') ||
    document.querySelector('title')?.textContent?.trim() ||
    document.querySelector('h1')?.textContent?.trim() ||
    domain;

  const description =
    getMeta('meta[property="og:description"]') ||
    getMeta('meta[name="twitter:description"]') ||
    getMeta('meta[name="description"]') ||
    null;

  const rawImage =
    getMeta('meta[property="og:image:secure_url"]') ||
    getMeta('meta[property="og:image"]') ||
    getMeta('meta[name="twitter:image"]') ||
    getMeta('meta[name="image"]');

  const imageUrl = resolveUrl(rawImage, targetUrl);

  const rawFavicon =
    document.querySelector('link[rel="apple-touch-icon"]')?.getAttribute('href') ||
    document.querySelector('link[rel="icon"]')?.getAttribute('href') ||
    document.querySelector('link[rel="shortcut icon"]')?.getAttribute('href') ||
    '/favicon.ico';

  const faviconUrl = resolveUrl(rawFavicon, targetUrl);

  const siteName =
    getMeta('meta[property="og:site_name"]') || getMeta('meta[name="application-name"]') || domain;

  const author =
    getMeta('meta[name="author"]') ||
    getMeta('meta[property="article:author"]') ||
    getMeta('meta[name="twitter:creator"]') ||
    null;

  const date =
    getMeta('meta[property="article:published_time"]') ||
    getMeta('meta[name="date"]') ||
    getMeta('meta[name="pubdate"]') ||
    document.querySelector('time')?.getAttribute('datetime') ||
    null;

  return {
    title,
    description,
    imageUrl,
    faviconUrl,
    domain,
    siteName,
    author,
    date
  };
}

/**
 * Compila o conteúdo HTML limpo em um documento PDF editorial via Pandoc + Typst.
 * @param {object} article
 * @param {object} meta
 * @param {string} targetUrl
 * @returns {Buffer|null}
 */
function compileSnapshotPdf(article, meta, targetUrl) {
  const sessionId = uuid.v4();
  const tempDir = os.tmpdir();
  const tempTypstWrapper = path.join(tempDir, `snap_doc_${sessionId}.typ`);
  const tempPdf = path.join(tempDir, `snap_out_${sessionId}.pdf`);

  try {
    const rawHtml = article.content || `<p>${escapeTypst(article.textContent || '')}</p>`;
    // Converte o HTML semântico limpo para código de formatação Typst
    const typstBody = cp.execSync('pandoc -f html -t typst', {
      input: rawHtml,
      encoding: 'utf8',
      maxBuffer: 20 * 1024 * 1024
    });

    const safeTitle = escapeTypst(meta.title || article.title || 'Artigo');
    const safeAuthor = escapeTypst(meta.author || article.byline || '');
    const safeDomain = escapeTypst(meta.domain || '');
    const safeDate = escapeTypst(
      meta.date ? meta.date.slice(0, 10) : new Date().toISOString().slice(0, 10)
    );
    const safeUrl = targetUrl.replace(/"/g, '\\"');

    const metaLine = [
      safeDomain ? `*Fonte:* ${safeDomain}` : null,
      safeAuthor ? `*Autor:* ${safeAuthor}` : null,
      safeDate ? `*Data:* ${safeDate}` : null
    ]
      .filter(Boolean)
      .join('  |  ');

    const typstDoc = `
#set page(
  paper: "a4",
  margin: (top: 2.5cm, bottom: 2.5cm, left: 2.5cm, right: 2.5cm),
  header: align(right)[#text(size: 8pt, fill: luma(120))[Snapshot Web | Joaju]],
  footer: [
    #set text(size: 8pt, fill: luma(120))
    #grid(
      columns: (1fr, auto),
      align(left)[#link("${safeUrl}")[#text(fill: rgb(30, 80, 160))[${safeUrl}]]],
      align(right)[#context counter(page).display("1 / 1", both: true)]
    )
  ]
)

#set text(font: ("Liberation Serif", "DejaVu Serif", "Times New Roman"), size: 11pt, lang: "pt")
#set par(justify: true, leading: 0.75em)

#v(0.5cm)
#text(size: 20pt, weight: "bold")[${safeTitle}]
#v(0.3cm)
#text(size: 9.5pt, fill: luma(90))[${metaLine}]
#v(0.5cm)
#line(length: 100%, stroke: 0.5pt + luma(180))
#v(0.6cm)

${typstBody}
`;

    fs.writeFileSync(tempTypstWrapper, typstDoc);
    cp.execSync(`typst compile "${tempTypstWrapper}" "${tempPdf}"`);

    if (fs.existsSync(tempPdf)) {
      return fs.readFileSync(tempPdf);
    }
    return null;
  } catch (err) {
    console.error(`[Joaju Unfurl] Falha ao compilar PDF de snapshot: ${err.message}`);
    return null;
  } finally {
    for (const f of [tempTypstWrapper, tempPdf]) {
      if (fs.existsSync(f)) {
        try {
          fs.unlinkSync(f);
        } catch {}
      }
    }
  }
}

/**
 * Função principal de unfurl e arquivamento de URL.
 * @param {string} targetUrl
 * @param {object} options { archive?: boolean }
 * @returns {Promise<object>}
 */
async function unfurlUrl(targetUrl, options = {}) {
  let parsedUrl;
  try {
    parsedUrl = new URL(targetUrl);
    if (!['http:', 'https:'].includes(parsedUrl.protocol)) {
      throw new Error('Protocolo inválido. Apenas http e https são permitidos.');
    }
  } catch (err) {
    return { ok: false, error: `URL inválida: ${err.message}` };
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), 12000);

  try {
    const response = await fetch(parsedUrl.href, {
      signal: controller.signal,
      headers: {
        'User-Agent':
          'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36',
        Accept: 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
        'Accept-Language': 'pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7'
      }
    });

    clearTimeout(timeout);

    if (!response.ok) {
      return {
        ok: false,
        error: `Servidor retornou HTTP ${response.status} ${response.statusText}`
      };
    }

    const contentType = response.headers.get('content-type') || '';
    if (!contentType.includes('text/html') && !contentType.includes('application/xhtml+xml')) {
      // Caso seja imagem ou PDF direto, extrai dados básicos
      const domain = parsedUrl.hostname.replace(/^www\./, '');
      return {
        ok: true,
        url: targetUrl,
        domain,
        siteName: domain,
        title: path.basename(parsedUrl.pathname) || domain,
        description: `Arquivo ${contentType}`,
        imageUrl: contentType.startsWith('image/') ? targetUrl : null,
        faviconUrl: `${parsedUrl.origin}/favicon.ico`,
        author: null,
        date: null,
        readingTimeMin: 1,
        hasArticle: false
      };
    }

    const html = await response.text();
    const { document } = parseHTML(html);

    const meta = extractMetadata(document, targetUrl);

    // Readability Engine (análise de corpo de artigo limpo)
    let article = null;
    try {
      const reader = new Readability(document);
      article = reader.parse();
    } catch (readErr) {
      console.warn(`[Joaju Unfurl] Falha no Readability: ${readErr.message}`);
    }

    // Calcula tempo estimado de leitura (base 200 palavras/min)
    let readingTimeMin = 1;
    const textContent = article?.textContent || document.body?.textContent || '';
    const wordCount = textContent.trim().split(/\s+/).filter(Boolean).length;
    if (wordCount > 0) {
      readingTimeMin = Math.max(1, Math.ceil(wordCount / 200));
    }

    let pdfBase64 = null;
    if (options.archive && article?.content) {
      const pdfBuffer = compileSnapshotPdf(article, meta, targetUrl);
      if (pdfBuffer) {
        pdfBase64 = pdfBuffer.toString('base64');
      }
    }

    return {
      ok: true,
      url: targetUrl,
      domain: meta.domain,
      siteName: meta.siteName,
      title: meta.title,
      description: meta.description,
      imageUrl: meta.imageUrl,
      faviconUrl: meta.faviconUrl,
      author: meta.author,
      date: meta.date,
      readingTimeMin,
      hasArticle: Boolean(article?.content),
      pdfBase64
    };
  } catch (err) {
    clearTimeout(timeout);
    return {
      ok: false,
      error: `Erro ao consultar URL: ${err.message}`
    };
  }
}

module.exports = {
  unfurlUrl,
  extractMetadata,
  compileSnapshotPdf,
  escapeTypst,
  resolveUrl
};
