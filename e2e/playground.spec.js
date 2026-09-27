const { test, expect } = require('@playwright/test');

test.describe('Joaju Web Playground E2E', () => {

  test.beforeEach(async ({ page }) => {
    // Mock do endpoint /render para garantir estabilidade hermética em qualquer ambiente
    await page.route('**/render', async (route) => {
      const request = route.request();
      if (request.method() === 'POST') {
        const body = request.postDataJSON() || {};
        const code = body.code || '';

        if (!code.trim()) {
          return route.fulfill({
            status: 400,
            contentType: 'application/json',
            body: JSON.stringify({ error: 'Nenhum código fornecido para renderização.' })
          });
        }

        if (code.includes('provoca_erro')) {
          return route.fulfill({
            status: 500,
            contentType: 'application/json',
            body: JSON.stringify({ error: 'Falha simulada na compilação do heredograma.' })
          });
        }

        const isMermaid = code.includes('graph ') || code.includes('flowchart') || code.includes('mermaid');
        return route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            format: isMermaid ? 'mermaid' : 'pedigree',
            type: 'svg',
            svg: isMermaid
              ? '<svg id="e2e-mermaid" viewBox="0 0 200 100"><rect width="200" height="100" fill="#38bdf8"/></svg>'
              : '<svg id="e2e-pedigree" viewBox="0 0 200 100"><circle cx="50" cy="50" r="30" fill="#10b981"/></svg>'
          })
        });
      }
      return route.continue();
    });
  });

  test('carrega a página inicial e renderiza o exemplo padrão de heredograma', async ({ page }) => {
    await page.goto('/');

    await expect(page).toHaveTitle(/Joaju/);
    await expect(page.locator('header h1')).toHaveText('Joaju');
    await expect(page.locator('.tagline')).toBeVisible();

    const codeInput = page.locator('#code-input');
    await expect(codeInput).toBeVisible();
    await expect(codeInput).toHaveValue(/```heredograma/);

    // Aguarda o SVG inicial ser renderizado no painel lateral
    const svg = page.locator('#preview-container svg');
    await expect(svg).toBeVisible();

    const statusBar = page.locator('#status-bar');
    await expect(statusBar).toContainText('renderizado com sucesso');

    const downloadBtn = page.locator('#btn-download');
    await expect(downloadBtn).toBeEnabled();
  });

  test('alterna entre templates de exemplo (Mermaid, Heredograma) e ação de limpar', async ({ page }) => {
    await page.goto('/');

    // 1. Clicar no botão Mermaid
    const btnMermaid = page.locator('#btn-example-mermaid');
    await btnMermaid.click();

    const codeInput = page.locator('#code-input');
    await expect(codeInput).toHaveValue(/```mermaid/);
    await expect(page.locator('#preview-container svg#e2e-mermaid')).toBeVisible();

    // 2. Clicar no botão Heredograma
    const btnPedigree = page.locator('#btn-example-pedigree');
    await btnPedigree.click();
    await expect(codeInput).toHaveValue(/```heredograma/);
    await expect(page.locator('#preview-container svg#e2e-pedigree')).toBeVisible();

    // 3. Clicar em Limpar
    const btnClear = page.locator('#btn-clear');
    await btnClear.click();
    await expect(codeInput).toHaveValue('');
    await expect(page.locator('.empty-state')).toBeVisible();
    await expect(page.locator('#btn-download')).toBeDisabled();
    await expect(page.locator('#status-bar')).toHaveText('Pronto.');
  });

  test('controla auto-render e atalho de teclado Ctrl+Enter', async ({ page }) => {
    await page.goto('/');

    const autoRender = page.locator('#auto-render');
    await expect(autoRender).toBeChecked();

    // Desmarca o auto-render
    await autoRender.uncheck();
    await expect(autoRender).not.toBeChecked();

    const codeInput = page.locator('#code-input');
    await codeInput.fill('```mermaid\ngraph LR\nC-->D\n```');

    // Com auto-render desmarcado, o preview ainda não deve ter mudado para e2e-mermaid
    // (aguarda tempo superior ao debounce de 400ms para certificar)
    await page.waitForTimeout(500);

    // Dispara via atalho de teclado Ctrl+Enter
    await codeInput.press('Control+Enter');

    await expect(page.locator('#preview-container svg#e2e-mermaid')).toBeVisible();
  });

  test('exibe cartão de erro amigável ao receber falha do servidor', async ({ page }) => {
    await page.goto('/');

    const codeInput = page.locator('#code-input');
    await codeInput.fill('```heredograma\nprovoca_erro\n```');

    const btnRender = page.locator('#btn-render');
    await btnRender.click();

    const errorCard = page.locator('.error-card');
    await expect(errorCard).toBeVisible();
    await expect(errorCard).toContainText('Falha simulada na compilação do heredograma');

    await expect(page.locator('#btn-download')).toBeDisabled();
    await expect(page.locator('#status-bar')).toHaveText('❌ Erro ao renderizar.');
  });

  test('permite download do arquivo SVG renderizado', async ({ page }) => {
    await page.goto('/');

    const downloadBtn = page.locator('#btn-download');
    await expect(downloadBtn).toBeEnabled();

    const downloadPromise = page.waitForEvent('download');
    await downloadBtn.click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/^joaju-grafico-\d+\.svg$/);
  });
});
