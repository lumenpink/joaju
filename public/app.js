const SAMPLE_KINSHIP = `\`\`\`kinship
%% layout.width=90% layout.max_width=550pt symbolsize=2.2 cex=0.85 branch=0.4
id,pai,mae,sex,fenotipo,label
1,0,0,M,wt,Avô
2,0,0,F,wt,Avó
3,1,2,M,het,Pai
4,0,0,F,wt,Mãe
5,3,4,F,hom,Probanda
6,3,4,M,wt,Irmão
\`\`\``;

const SAMPLE_GRAPHVIZ = `\`\`\`graphviz
%% width=80% max-width=550pt
digraph G {
  rankdir=LR;
  node [shape=box, style="rounded,filled", fillcolor="#1e293b", fontcolor="#f8fafc", color="#38bdf8", fontname="Helvetica"];
  edge [color="#94a3b8", fontcolor="#94a3b8", fontname="Helvetica"];

  Coleta [label="Coleta de Dados"];
  PreProc [label="Processamento & Limpeza"];
  Validacao [label="Validação", shape=diamond];
  Analise [label="Análise Estatística"];
  Relatorio [label="Relatório Final"];

  Coleta -> PreProc -> Validacao;
  Validacao -> Analise [label="Válido"];
  Validacao -> PreProc [label="Ajustar", style=dashed];
  Analise -> Relatorio;
}
\`\`\``;

const SAMPLE_PLANTUML = `\`\`\`plantuml
%% width=80% max-width=550pt
@startuml
skinparam backgroundColor transparent
skinparam defaultFontName Helvetica
skinparam sequence {
  ArrowColor #38bdf8
  LifeLineBorderColor #38bdf8
  ParticipantBorderColor #38bdf8
  ParticipantBackgroundColor #1e293b
  ParticipantFontColor #f8fafc
}

autonumber
Pesquisador -> Joaju: Envia Markdown
Joaju -> Kinship: Compila Heredograma (R)
Kinship --> Joaju: Retorna SVG Vetorial
Joaju -> Typst: Compila PDF Acadêmico
Typst --> Pesquisador: Devolve Artigo
@enduml
\`\`\``;

const SAMPLE_D2 = `\`\`\`d2
%% width=80% max-width=550pt
vars: {
  d2-config: {
    theme-id: 200
  }
}

coleta: Coleta de Dados
processamento: Processamento & Limpeza
validacao: Validação Estatística {
  shape: diamond
}
publicacao: Publicação Final {
  style.stroke: "#38bdf8"
  style.fill: "#1e293b"
}

coleta -> processamento -> validacao
validacao -> publicacao: Aprovado
validacao -> processamento: Revisar {
  style.stroke-dash: 3
}
\`\`\``;

const SAMPLE_PEDIGREE = SAMPLE_KINSHIP;
const SAMPLE_MERMAID = SAMPLE_GRAPHVIZ;

const LOCALES = {
  pt: {
    lang_code: 'pt-BR',
    direction: 'ltr',
    tagline: 'Playground de diagramas acadêmicos e gráficos científicos',
    btn_pedigree: 'Heredograma',
    btn_mermaid: 'Diagramas (Mermaid)',
    btn_render: 'Renderizar',
    editor_label: 'Código Fonte',
    auto_render: 'Auto-render',
    btn_clear: 'Limpar',
    placeholder: 'Cole aqui seu bloco ```mermaid, diagramas ou ```heredograma...',
    characters_singular: 'caractere',
    characters_plural: 'caracteres',
    shortcut_tip: 'Atalho:',
    preview_label: 'Resultado',
    btn_download: 'Baixar SVG',
    empty_state: 'Digite ou escolha um exemplo acima para visualizar o gráfico instantaneamente.',
    status_ready: 'Pronto.',
    status_rendering: 'Renderizando...',
    status_rendered: 'Renderizado com sucesso ({format}, {type}).',
    status_cleared: 'Editor limpo.',
    error_title: 'Erro na renderização'
  },
  en: {
    lang_code: 'en',
    direction: 'ltr',
    tagline: 'Playground for academic diagrams and scientific graphics',
    btn_pedigree: 'Pedigree',
    btn_mermaid: 'Diagrams (Mermaid)',
    btn_render: 'Render',
    editor_label: 'Source Code',
    auto_render: 'Auto-render',
    btn_clear: 'Clear',
    placeholder: 'Paste your ```mermaid, diagrams or ```heredograma block here...',
    characters_singular: 'character',
    characters_plural: 'characters',
    shortcut_tip: 'Shortcut:',
    preview_label: 'Result',
    btn_download: 'Download SVG',
    empty_state: 'Type or select an example above to preview the chart instantly.',
    status_ready: 'Ready.',
    status_rendering: 'Rendering...',
    status_rendered: 'Rendered successfully ({format}, {type}).',
    status_cleared: 'Editor cleared.',
    error_title: 'Rendering error'
  },
  es: {
    lang_code: 'es',
    direction: 'ltr',
    tagline: 'Espacio de pruebas para diagramas académicos y gráficos científicos',
    btn_pedigree: 'Pedigrí',
    btn_mermaid: 'Diagramas (Mermaid)',
    btn_render: 'Renderizar',
    editor_label: 'Código Fuente',
    auto_render: 'Auto-render',
    btn_clear: 'Limpiar',
    placeholder: 'Pegue aquí su bloque ```mermaid, diagramas o ```heredograma...',
    characters_singular: 'carácter',
    characters_plural: 'caracteres',
    shortcut_tip: 'Atajo:',
    preview_label: 'Resultado',
    btn_download: 'Descargar SVG',
    empty_state: 'Escriba o elija un ejemplo arriba para ver el gráfico instantáneamente.',
    status_ready: 'Listo.',
    status_rendering: 'Renderizando...',
    status_rendered: 'Renderizado con éxito ({format}, {type}).',
    status_cleared: 'Editor limpio.',
    error_title: 'Error en la renderización'
  },
  fr: {
    lang_code: 'fr',
    direction: 'ltr',
    tagline: "Espace d'expérimentation pour diagrammes académiques et graphiques scientifiques",
    btn_pedigree: 'Arbre généalogique',
    btn_mermaid: 'Diagrammes (Mermaid)',
    btn_render: 'Rendre',
    editor_label: 'Code Source',
    auto_render: 'Rendu auto',
    btn_clear: 'Effacer',
    placeholder: 'Collez ici votre bloc ```mermaid, diagrammes ou ```heredograma...',
    characters_singular: 'caractère',
    characters_plural: 'caractères',
    shortcut_tip: 'Raccourci :',
    preview_label: 'Résultat',
    btn_download: 'Télécharger SVG',
    empty_state: 'Saisissez ou choisissez un exemple ci-dessus pour prévisualiser instantanément.',
    status_ready: 'Prêt.',
    status_rendering: 'Rendu en cours...',
    status_rendered: 'Rendu effectué avec succès ({format}, {type}).',
    status_cleared: 'Éditeur effacé.',
    error_title: 'Erreur de rendu'
  },
  de: {
    lang_code: 'de',
    direction: 'ltr',
    tagline: 'Playground für akademische Diagramme und wissenschaftliche Grafiken',
    btn_pedigree: 'Stammbaum',
    btn_mermaid: 'Diagramme (Mermaid)',
    btn_render: 'Rendern',
    editor_label: 'Quellcode',
    auto_render: 'Auto-Rendern',
    btn_clear: 'Löschen',
    placeholder: 'Fügen Sie hier Ihren ```mermaid-, Diagramm- oder ```heredograma-Block ein...',
    characters_singular: 'Zeichen',
    characters_plural: 'Zeichen',
    shortcut_tip: 'Tastenkürzel:',
    preview_label: 'Ergebnis',
    btn_download: 'SVG Herunterladen',
    empty_state: 'Geben Sie Code ein oder wählen Sie ein Beispiel aus.',
    status_ready: 'Bereit.',
    status_rendering: 'Wird gerendert...',
    status_rendered: 'Erfolgreich gerendert ({format}, {type}).',
    status_cleared: 'Editor geleert.',
    error_title: 'Rendering-Fehler'
  },
  it: {
    lang_code: 'it',
    direction: 'ltr',
    tagline: 'Playground per diagrammi accademici e grafici scientifici',
    btn_pedigree: 'Albero genealogico',
    btn_mermaid: 'Diagrammi (Mermaid)',
    btn_render: 'Renderizza',
    editor_label: 'Codice Sorgente',
    auto_render: 'Auto-render',
    btn_clear: 'Cancella',
    placeholder: 'Incolla qui il tuo blocco ```mermaid, diagrammi o ```heredograma...',
    characters_singular: 'carattere',
    characters_plural: 'caratteri',
    shortcut_tip: 'Scorciatoia:',
    preview_label: 'Risultato',
    btn_download: 'Scarica SVG',
    empty_state: 'Scrivi o scegli un esempio sopra per visualizzare il grafico istantaneamente.',
    status_ready: 'Pronto.',
    status_rendering: 'Renderizzazione in corso...',
    status_rendered: 'Renderizzato con successo ({format}, {type}).',
    status_cleared: 'Editor pulito.',
    error_title: 'Errore di renderizzazione'
  },
  ru: {
    lang_code: 'ru',
    direction: 'ltr',
    tagline: 'Интерактивная среда для академических диаграмм и научных графиков',
    btn_pedigree: 'Родословная',
    btn_mermaid: 'Диаграммы (Mermaid)',
    btn_render: 'Рендерить',
    editor_label: 'Исходный Код',
    auto_render: 'Авто-рендер',
    btn_clear: 'Очистить',
    placeholder: 'Вставьте сюда блок ```mermaid, диаграммы или ```heredograma...',
    characters_singular: 'символ',
    characters_plural: 'символов',
    shortcut_tip: 'Горячая клавиша:',
    preview_label: 'Результат',
    btn_download: 'Скачать SVG',
    empty_state: 'Введите код или выберите пример выше для мгновенного просмотра.',
    status_ready: 'Готово.',
    status_rendering: 'Рендеринг...',
    status_rendered: 'Успешно сгенерировано ({format}, {type}).',
    status_cleared: 'Редактор очищен.',
    error_title: 'Ошибка рендеринга'
  },
  zh: {
    lang_code: 'zh',
    direction: 'ltr',
    tagline: '学术图表与科学图形演练场',
    btn_pedigree: '系谱图',
    btn_mermaid: '图表 (Mermaid)',
    btn_render: '渲染',
    editor_label: '源代码',
    auto_render: '自动渲染',
    btn_clear: '清空',
    placeholder: '在此处粘贴您的 ```mermaid、图表或 ```heredograma 代码块...',
    characters_singular: '个字符',
    characters_plural: '个字符',
    shortcut_tip: '快捷键：',
    preview_label: '渲染结果',
    btn_download: '下载 SVG',
    empty_state: '在上方输入代码或选择示例以即时预览图表。',
    status_ready: '就绪。',
    status_rendering: '正在渲染...',
    status_rendered: '渲染成功 ({format}, {type})。',
    status_cleared: '编辑器已清空。',
    error_title: '渲染错误'
  },
  ja: {
    lang_code: 'ja',
    direction: 'ltr',
    tagline: '学術ダイアグラムと科学グラフィックのプレイグラウンド',
    btn_pedigree: '家系図',
    btn_mermaid: 'ダイアグラム (Mermaid)',
    btn_render: '描画',
    editor_label: 'ソースコード',
    auto_render: '自動描画',
    btn_clear: 'クリア',
    placeholder:
      'ここに ```mermaid、ダイアグラム、または ```heredograma ブロックを貼り付けてください...',
    characters_singular: '文字',
    characters_plural: '文字',
    shortcut_tip: 'ショートカット:',
    preview_label: '結果',
    btn_download: 'SVGを保存',
    empty_state: 'コードを入力するか上記の例を選択してプレビューします。',
    status_ready: '準備完了。',
    status_rendering: 'レンダリング中...',
    status_rendered: '正常にレンダリングされました ({format}, {type})。',
    status_cleared: 'エディタをクリアしました。',
    error_title: 'レンダリングエラー'
  },
  ar: {
    lang_code: 'ar',
    direction: 'rtl',
    tagline: 'بيئة تفاعلية للمخططات الأكاديمية والرسومات العلمية',
    btn_pedigree: 'شجرة النسب',
    btn_mermaid: 'مخططات (Mermaid)',
    btn_render: 'معالجة وعرض',
    editor_label: 'الرمز المصدري',
    auto_render: 'عرض تلقائي',
    btn_clear: 'مسح',
    placeholder: 'الصق كتلة ```mermaid أو المخططات أو ```heredograma هنا...',
    characters_singular: 'حرف',
    characters_plural: 'أحرف',
    shortcut_tip: 'اختصار:',
    preview_label: 'النتيجة',
    btn_download: 'تحميل SVG',
    empty_state: 'اكتب الرمز أو اختر نموذجاً من الأعلى للمعاينة الفورية.',
    status_ready: 'جاهز.',
    status_rendering: 'جارٍ المعالجة والعرض...',
    status_rendered: 'تمت المعالجة بنجاح ({format}, {type}).',
    status_cleared: 'تم مسح المحرر.',
    error_title: 'خطأ في المعالجة'
  }
};

document.addEventListener('DOMContentLoaded', () => {
  const codeInput = document.getElementById('code-input');
  const previewContainer = document.getElementById('preview-container');
  const statusBar = document.getElementById('status-bar');
  const charCount = document.getElementById('char-count');
  const autoRenderCheckbox = document.getElementById('auto-render');
  const btnRender = document.getElementById('btn-render');
  const btnClear = document.getElementById('btn-clear');
  const btnDownload = document.getElementById('btn-download');
  const btnKinship = document.getElementById('btn-example-kinship');
  const btnGraphviz = document.getElementById('btn-example-graphviz');
  const btnPlantuml = document.getElementById('btn-example-plantuml');
  const btnD2 = document.getElementById('btn-example-d2');
  const btnPedigree = document.getElementById('btn-example-pedigree');
  const btnMermaid = document.getElementById('btn-example-mermaid');
  const langSelect = document.getElementById('lang-select');

  let currentSvg = '';
  let debounceTimer = null;
  let currentAbortController = null;
  let lastRenderedCode = null;
  const DEBOUNCE_MS = 800;
  let currentLang = localStorage.getItem('joaju_lang') || 'pt';
  if (!LOCALES[currentLang]) currentLang = 'pt';

  function getT() {
    return LOCALES[currentLang] || LOCALES.pt;
  }

  function applyLocale(lang) {
    const targetLang = LOCALES[lang] ? lang : 'pt';
    currentLang = targetLang;
    localStorage.setItem('joaju_lang', targetLang);

    const t = getT();
    document.documentElement.lang = t.lang_code;
    document.documentElement.dir = t.direction;

    for (const el of document.querySelectorAll('[data-i18n]')) {
      const key = el.getAttribute('data-i18n');
      if (t[key]) el.textContent = t[key];
    }

    for (const el of document.querySelectorAll('[data-i18n-placeholder]')) {
      const key = el.getAttribute('data-i18n-placeholder');
      if (t[key]) el.placeholder = t[key];
    }

    if (langSelect) {
      langSelect.value = targetLang;
    }

    updateCharCount();

    if (!codeInput.value.trim()) {
      statusBar.textContent = t.status_ready;
    }
  }

  function updateCharCount() {
    const t = getT();
    const len = codeInput.value.length;
    const unit = len === 1 ? t.characters_singular : t.characters_plural;
    charCount.textContent = `${len} ${unit}`;
  }

  async function triggerRender(force = false) {
    const t = getT();
    const code = codeInput.value.trim();

    if (!code) {
      if (currentAbortController) {
        currentAbortController.abort();
        currentAbortController = null;
      }
      lastRenderedCode = '';
      previewContainer.innerHTML = `
                <div class="empty-state">
                    <div class="empty-icon">📊</div>
                    <p>${escapeHtml(t.empty_state)}</p>
                </div>
            `;
      btnDownload.disabled = true;
      currentSvg = '';
      statusBar.textContent = t.status_ready;
      return;
    }

    if (!force && code === lastRenderedCode) {
      return;
    }

    // Trava anti-empilhamento: cancela qualquer requisição em trânsito
    if (currentAbortController) {
      currentAbortController.abort();
    }
    const abortController = new AbortController();
    currentAbortController = abortController;

    statusBar.textContent = `⏳ ${t.status_rendering}`;

    try {
      const response = await fetch('/render', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code }),
        signal: abortController.signal
      });

      let data = {};
      const rawText = await response.text();
      try {
        data = JSON.parse(rawText);
      } catch (_) {
        data = { error: rawText || `Erro HTTP ${response.status}` };
      }

      if (!response.ok) {
        throw new Error(data.error || 'Erro desconhecido ao processar gráfico');
      }

      if (data.svg) {
        currentSvg = data.svg;
        lastRenderedCode = code;
        previewContainer.innerHTML = data.svg;
        btnDownload.disabled = false;
        const msg = t.status_rendered
          .replace('{format}', data.format.toUpperCase())
          .replace('{type}', (data.type || 'svg').toUpperCase());
        statusBar.textContent = `✅ ${msg}`;
      } else {
        throw new Error('Servidor não retornou dados de imagem válidos.');
      }
    } catch (err) {
      if (err.name === 'AbortError') {
        // Ignora requisição abortada deliberadamente por nova digitação
        return;
      }
      currentSvg = '';
      btnDownload.disabled = true;
      previewContainer.innerHTML = `
                <div class="error-card">
                    <strong>⚠️ ${escapeHtml(t.error_title)}:</strong>\n${escapeHtml(err.message)}
                </div>
            `;
      statusBar.textContent = `❌ ${t.error_title}.`;
    } finally {
      if (currentAbortController === abortController) {
        currentAbortController = null;
      }
    }
  }

  function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  function downloadSvg() {
    if (!currentSvg) return;
    const blob = new Blob([currentSvg], { type: 'image/svg+xml;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `joaju-grafico-${Date.now()}.svg`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  codeInput.addEventListener('input', () => {
    updateCharCount();
    if (autoRenderCheckbox.checked) {
      clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => triggerRender(false), DEBOUNCE_MS);
    }
  });

  codeInput.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      clearTimeout(debounceTimer);
      triggerRender(true);
    }
  });

  btnRender.addEventListener('click', () => {
    clearTimeout(debounceTimer);
    triggerRender(true);
  });

  btnClear.addEventListener('click', () => {
    clearTimeout(debounceTimer);
    codeInput.value = '';
    updateCharCount();
    triggerRender(true);
  });

  if (btnKinship) {
    btnKinship.addEventListener('click', () => {
      clearTimeout(debounceTimer);
      codeInput.value = SAMPLE_KINSHIP;
      updateCharCount();
      triggerRender(true);
    });
  }

  if (btnGraphviz) {
    btnGraphviz.addEventListener('click', () => {
      clearTimeout(debounceTimer);
      codeInput.value = SAMPLE_GRAPHVIZ;
      updateCharCount();
      triggerRender(true);
    });
  }

  if (btnPlantuml) {
    btnPlantuml.addEventListener('click', () => {
      clearTimeout(debounceTimer);
      codeInput.value = SAMPLE_PLANTUML;
      updateCharCount();
      triggerRender(true);
    });
  }

  if (btnD2) {
    btnD2.addEventListener('click', () => {
      clearTimeout(debounceTimer);
      codeInput.value = SAMPLE_D2;
      updateCharCount();
      triggerRender(true);
    });
  }

  if (btnPedigree) {
    btnPedigree.addEventListener('click', () => {
      clearTimeout(debounceTimer);
      codeInput.value = SAMPLE_KINSHIP;
      updateCharCount();
      triggerRender(true);
    });
  }

  if (btnMermaid) {
    btnMermaid.addEventListener('click', () => {
      clearTimeout(debounceTimer);
      codeInput.value = SAMPLE_GRAPHVIZ;
      updateCharCount();
      triggerRender(true);
    });
  }

  btnDownload.addEventListener('click', downloadSvg);

  const btnOpenGuide = document.getElementById('btn-open-guide');
  const btnCloseGuide = document.getElementById('btn-close-guide');
  const guideModal = document.getElementById('guide-modal');

  if (btnOpenGuide && guideModal) {
    btnOpenGuide.addEventListener('click', () => {
      guideModal.classList.remove('hidden');
    });
  }

  if (btnCloseGuide && guideModal) {
    btnCloseGuide.addEventListener('click', () => {
      guideModal.classList.add('hidden');
    });
  }

  if (guideModal) {
    guideModal.addEventListener('click', (e) => {
      if (e.target === guideModal) {
        guideModal.classList.add('hidden');
      }
    });

    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && !guideModal.classList.contains('hidden')) {
        guideModal.classList.add('hidden');
      }
    });
  }

  if (langSelect) {
    langSelect.addEventListener('change', (e) => {
      applyLocale(e.target.value);
    });
  }

  // Inicialização da língua e dados
  applyLocale(currentLang);
  codeInput.value = SAMPLE_KINSHIP;
  updateCharCount();
  triggerRender();
});
