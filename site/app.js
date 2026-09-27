function copyCode(btn) {
  const codeBlock = btn.closest('.code-block');
  if (!codeBlock) return;
  const code = codeBlock.querySelector('code');
  if (!code) return;

  navigator.clipboard.writeText(code.innerText).then(() => {
    const originalText = btn.textContent;
    btn.textContent = 'Copiado!';
    btn.style.borderColor = '#10b981';
    btn.style.color = '#34d399';
    setTimeout(() => {
      btn.textContent = originalText;
      btn.style.borderColor = '';
      btn.style.color = '';
    }, 2000);
  });
}
