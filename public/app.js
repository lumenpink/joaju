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

    let currentSvg = '';
    let debounceTimer = null;

    function updateCharCount() {
        const len = codeInput.value.length;
        charCount.textContent = `${len} caractere${len === 1 ? '' : 's'}`;
    }

    async function triggerRender() {
        const code = codeInput.value.trim();
        if (!code) {
            previewContainer.innerHTML = `
                <div class="empty-state">
                    <div class="empty-icon">📊</div>
                    <p>Digite ou escolha um exemplo acima para visualizar o gráfico instantaneamente.</p>
                </div>
            `;
            btnDownload.disabled = true;
            currentSvg = '';
            statusBar.textContent = 'Pronto.';
            return;
        }

        statusBar.textContent = '⏳ Renderizando...';

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
                statusBar.textContent = `✅ Gráfico renderizado com sucesso (${data.format.toUpperCase()}).`;
            } else {
                throw new Error('Servidor não retornou dados de imagem válidos.');
            }
        } catch (err) {
            currentSvg = '';
            btnDownload.disabled = true;
            previewContainer.innerHTML = `
                <div class="error-card">
                    <strong>⚠️ Falha de Renderização:</strong>\n${escapeHtml(err.message)}
                </div>
            `;
            statusBar.textContent = '❌ Erro ao renderizar.';
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

    // Carrega exemplo inicial por padrão
    codeInput.value = SAMPLE_PEDIGREE;
    updateCharCount();
    triggerRender();
});
