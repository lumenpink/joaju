const SAMPLE_PEDIGREE = `\`\`\`heredograma
%% layout.width=90% layout.max_width=550pt cex=0.8
id,pai,mae,sex,fenotipo,label
1,0,0,M,wt,Avô
2,0,0,F,wt,Avó
3,1,2,M,het,Pai
4,0,0,F,wt,Mãe
5,3,4,F,hom,Probanda
6,3,4,M,wt,Irmão
\`\`\``;

const SAMPLE_MERMAID = `\`\`\`mermaid
%% width=85% max-width=600pt
graph TD
    A[Amostra Clínica] --> B[Sequenciamento NGS]
    B --> C{Variante Patogênica?}
    C -->|Sim| D[Aconselhamento Genético]
    C -->|Não| E[Seguimento Ambulatorial]
    D --> F[Laudo Molecular Definitivo]
\`\`\``;

const LOCALES = {
  pt: {
    lang_code: "pt-BR",
    direction: "ltr",
    tagline: "Playground de diagramas e heredogramas clínicos",
    btn_pedigree: "Heredograma",
    btn_mermaid: "Mermaid",
    btn_render: "Renderizar",
    editor_label: "Código Fonte",
    auto_render: "Auto-render",
    btn_clear: "Limpar",
    placeholder: "Cole aqui seu bloco ```heredograma ou ```mermaid (ou apenas os dados internos)...",
    characters_singular: "caractere",
    characters_plural: "caracteres",
    shortcut_tip: "Atalho:",
    preview_label: "Resultado",
    btn_download: "Baixar SVG",
    empty_state: "Digite ou escolha um exemplo acima para visualizar o gráfico instantaneamente.",
    status_ready: "Pronto.",
    status_rendering: "Renderizando...",
    status_rendered: "Gráfico renderizado com sucesso ({format}).",
    status_cleared: "Pronto.",
    error_title: "Erro ao renderizar"
  },
  en: {
    lang_code: "en",
    direction: "ltr",
    tagline: "Playground for clinical pedigrees and diagrams",
    btn_pedigree: "Pedigree",
    btn_mermaid: "Mermaid",
    btn_render: "Render",
    editor_label: "Source Code",
    auto_render: "Auto-render",
    btn_clear: "Clear",
    placeholder: "Paste your ```heredograma or ```mermaid block here (or raw tabular data)...",
    characters_singular: "character",
    characters_plural: "characters",
    shortcut_tip: "Shortcut:",
    preview_label: "Result",
    btn_download: "Download SVG",
    empty_state: "Type or select an example above to preview the chart instantly.",
    status_ready: "Ready.",
    status_rendering: "Rendering...",
    status_rendered: "Chart successfully rendered ({format}, {type}).",
    status_cleared: "Editor cleared.",
    error_title: "Rendering Error"
  },
  es: {
    lang_code: "es",
    direction: "ltr",
    tagline: "Espacio de pruebas para diagramas y pedigríes clínicos",
    btn_pedigree: "Pedigrí",
    btn_mermaid: "Mermaid",
    btn_render: "Renderizar",
    editor_label: "Código Fuente",
    auto_render: "Auto-render",
    btn_clear: "Limpiar",
    placeholder: "Pegue aquí su bloque ```heredograma o ```mermaid (o datos sin formato)...",
    characters_singular: "carácter",
    characters_plural: "caracteres",
    shortcut_tip: "Atajo:",
    preview_label: "Resultado",
    btn_download: "Descargar SVG",
    empty_state: "Escriba o elija un ejemplo arriba para ver el gráfico instantáneamente.",
    status_ready: "Listo.",
    status_rendering: "Renderizando...",
    status_rendered: "Gráfico renderizado con éxito ({format}, {type}).",
    status_cleared: "Editor limpio.",
    error_title: "Error de Renderización"
  },
  fr: {
    lang_code: "fr",
    direction: "ltr",
    tagline: "Espace d'expérimentation pour diagrammes et arbres généalogiques cliniques",
    btn_pedigree: "Arbre généalogique",
    btn_mermaid: "Mermaid",
    btn_render: "Rendre",
    editor_label: "Code Source",
    auto_render: "Rendu auto",
    btn_clear: "Effacer",
    placeholder: "Collez ici votre bloc ```heredograma ou ```mermaid (ou données brutes)...",
    characters_singular: "caractère",
    characters_plural: "caractères",
    shortcut_tip: "Raccourci :",
    preview_label: "Résultat",
    btn_download: "Télécharger SVG",
    empty_state: "Saisissez ou choisissez un exemple ci-dessus pour prévisualiser instantanément.",
    status_ready: "Prêt.",
    status_rendering: "Rendu en cours...",
    status_rendered: "Graphique rendu avec succès ({format}, {type}).",
    status_cleared: "Éditeur effacé.",
    error_title: "Erreur de Rendu"
  },
  de: {
    lang_code: "de",
    direction: "ltr",
    tagline: "Spielwiese für klinische Stammbäume und Diagramme",
    btn_pedigree: "Stammbaum",
    btn_mermaid: "Mermaid",
    btn_render: "Rendern",
    editor_label: "Quellcode",
    auto_render: "Auto-Rendern",
    btn_clear: "Löschen",
    placeholder: "Fügen Sie hier Ihren ```heredograma- oder ```mermaid-Block ein...",
    characters_singular: "Zeichen",
    characters_plural: "Zeichen",
    shortcut_tip: "Tastenkürzel:",
    preview_label: "Ergebnis",
    btn_download: "SVG Herunterladen",
    empty_state: "Geben Sie Code ein oder wählen Sie ein Beispiel aus.",
    status_ready: "Bereit.",
    status_rendering: "Wird gerendert...",
    status_rendered: "Diagramm erfolgreich gerendert ({format}, {type}).",
    status_cleared: "Editor geleert.",
    error_title: "Rendering-Fehler"
  },
  it: {
    lang_code: "it",
    direction: "ltr",
    tagline: "Playground per diagrammi e alberi genealogici clinici",
    btn_pedigree: "Albero genealogico",
    btn_mermaid: "Mermaid",
    btn_render: "Renderizza",
    editor_label: "Codice Sorgente",
    auto_render: "Auto-render",
    btn_clear: "Cancella",
    placeholder: "Incolla qui il tuo blocco ```heredograma o ```mermaid...",
    characters_singular: "carattere",
    characters_plural: "caratteri",
    shortcut_tip: "Scorciatoia:",
    preview_label: "Risultato",
    btn_download: "Scarica SVG",
    empty_state: "Scrivi o scegli un esempio sopra per visualizzare il grafico istantaneamente.",
    status_ready: "Pronto.",
    status_rendering: "Renderizzazione in corso...",
    status_rendered: "Grafico renderizzato con successo ({format}, {type}).",
    status_cleared: "Editor pulito.",
    error_title: "Errore di Renderizzazione"
  },
  ru: {
    lang_code: "ru",
    direction: "ltr",
    tagline: "Интерактивная среда для клинических родословных и диаграмм",
    btn_pedigree: "Родословная",
    btn_mermaid: "Mermaid",
    btn_render: "Рендерить",
    editor_label: "Исходный Код",
    auto_render: "Авто-рендер",
    btn_clear: "Очистить",
    placeholder: "Вставьте сюда блок ```heredograma или ```mermaid...",
    characters_singular: "символ",
    characters_plural: "символов",
    shortcut_tip: "Горячая клавиша:",
    preview_label: "Результат",
    btn_download: "Скачать SVG",
    empty_state: "Введите код или выберите пример выше для мгновенного просмотра.",
    status_ready: "Готово.",
    status_rendering: "Рендеринг...",
    status_rendered: "График успешно сгенерирован ({format}, {type}).",
    status_cleared: "Редактор очищен.",
    error_title: "Ошибка Рендеринга"
  },
  zh: {
    lang_code: "zh",
    direction: "ltr",
    tagline: "临床系谱图与科学图表开发试验场",
    btn_pedigree: "系谱图",
    btn_mermaid: "Mermaid",
    btn_render: "渲染",
    editor_label: "源代码",
    auto_render: "自动渲染",
    btn_clear: "清空",
    placeholder: "在此处粘贴您的 ```heredograma 或 ```mermaid 代码块...",
    characters_singular: "个字符",
    characters_plural: "个字符",
    shortcut_tip: "快捷键：",
    preview_label: "渲染结果",
    btn_download: "下载 SVG",
    empty_state: "在上方输入代码或选择示例以即时预览图表。",
    status_ready: "就绪。",
    status_rendering: "正在渲染...",
    status_rendered: "图表渲染成功 ({format}, {type})。",
    status_cleared: "编辑器已清空。",
    error_title: "渲染错误"
  },
  ja: {
    lang_code: "ja",
    direction: "ltr",
    tagline: "臨床家系図と科学ダイアグラムのプレイグラウンド",
    btn_pedigree: "家系図",
    btn_mermaid: "Mermaid",
    btn_render: "描画",
    editor_label: "ソースコード",
    auto_render: "自動描画",
    btn_clear: "クリア",
    placeholder: "ここに ```heredograma または ```mermaid ブロックを貼り付けてください...",
    characters_singular: "文字",
    characters_plural: "文字",
    shortcut_tip: "ショートカット:",
    preview_label: "結果",
    btn_download: "SVGを保存",
    empty_state: "コードを入力するか上記の例を選択してプレビューします。",
    status_ready: "準備完了。",
    status_rendering: "レンダリング中...",
    status_rendered: "正常に描画されました ({format}, {type})。",
    status_cleared: "エディタをクリアしました。",
    error_title: "描画エラー"
  },
  ar: {
    lang_code: "ar",
    direction: "rtl",
    tagline: "بيئة تفاعلية للمخططات وشجرات النسب السريرية",
    btn_pedigree: "شجرة النسب",
    btn_mermaid: "Mermaid",
    btn_render: "معالجة وعرض",
    editor_label: "الرمز المصدري",
    auto_render: "عرض تلقائي",
    btn_clear: "مسح",
    placeholder: "الصق كتلة ```heredograma أو ```mermaid هنا...",
    characters_singular: "حرف",
    characters_plural: "أحرف",
    shortcut_tip: "اختصار:",
    preview_label: "النتيجة",
    btn_download: "تحميل SVG",
    empty_state: "اكتب الرمز أو اختر نموذجاً من الأعلى للمعاينة الفورية.",
    status_ready: "جاهز.",
    status_rendering: "جارٍ المعالجة والعرض...",
    status_rendered: "تمت المعالجة بنجاح ({format}, {type}).",
    status_cleared: "تم مسح المحرر.",
    error_title: "خطأ في المعالجة"
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
    const btnPedigree = document.getElementById('btn-example-pedigree');
    const btnMermaid = document.getElementById('btn-example-mermaid');
    const langSelect = document.getElementById('lang-select');

    let currentSvg = '';
    let debounceTimer = null;
    let currentLang = localStorage.getItem('joaju_lang') || 'pt';
    if (!LOCALES[currentLang]) currentLang = 'pt';

    function getT() {
        return LOCALES[currentLang] || LOCALES.pt;
    }

    function applyLocale(lang) {
        if (!LOCALES[lang]) lang = 'pt';
        currentLang = lang;
        localStorage.setItem('joaju_lang', lang);

        const t = getT();
        document.documentElement.lang = t.lang_code;
        document.documentElement.dir = t.direction;

        document.querySelectorAll('[data-i18n]').forEach(el => {
            const key = el.getAttribute('data-i18n');
            if (t[key]) el.textContent = t[key];
        });

        document.querySelectorAll('[data-i18n-placeholder]').forEach(el => {
            const key = el.getAttribute('data-i18n-placeholder');
            if (t[key]) el.placeholder = t[key];
        });

        if (langSelect) {
            langSelect.value = lang;
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

    async function triggerRender() {
        const t = getT();
        const code = codeInput.value.trim();
        if (!code) {
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

        statusBar.textContent = `⏳ ${t.status_rendering}`;

        try {
            const response = await fetch('/render', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ code })
            });

            const data = await response.json();

            if (!response.ok) {
                throw new Error(data.error || 'Erro desconhecido ao processar gráfico');
            }

            if (data.svg) {
                currentSvg = data.svg;
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
            currentSvg = '';
            btnDownload.disabled = true;
            previewContainer.innerHTML = `
                <div class="error-card">
                    <strong>⚠️ ${escapeHtml(t.error_title)}:</strong>\n${escapeHtml(err.message)}
                </div>
            `;
            statusBar.textContent = `❌ ${t.error_title}.`;
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
            debounceTimer = setTimeout(triggerRender, 400);
        }
    });

    codeInput.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
            e.preventDefault();
            triggerRender();
        }
    });

    btnRender.addEventListener('click', triggerRender);

    btnClear.addEventListener('click', () => {
        codeInput.value = '';
        updateCharCount();
        triggerRender();
    });

    btnPedigree.addEventListener('click', () => {
        codeInput.value = SAMPLE_PEDIGREE;
        updateCharCount();
        triggerRender();
    });

    btnMermaid.addEventListener('click', () => {
        codeInput.value = SAMPLE_MERMAID;
        updateCharCount();
        triggerRender();
    });

    btnDownload.addEventListener('click', downloadSvg);

    if (langSelect) {
        langSelect.addEventListener('change', (e) => {
            applyLocale(e.target.value);
        });
    }

    // Inicialização da língua e dados
    applyLocale(currentLang);
    codeInput.value = SAMPLE_PEDIGREE;
    updateCharCount();
    triggerRender();
});
