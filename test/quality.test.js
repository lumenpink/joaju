const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');

test('Conformidade GOST: Qualidade Técnica, Linters, Twelve-Factor R e NO_COLOR', async (t) => {
  const rootDir = path.resolve(__dirname, '..');

  await t.test('biome.json existe e é uma configuração válida', () => {
    const biomePath = path.join(rootDir, 'biome.json');
    assert.ok(fs.existsSync(biomePath), 'biome.json deve existir');
    const content = JSON.parse(fs.readFileSync(biomePath, 'utf8'));
    assert.equal(content.formatter.enabled, true);
    assert.equal(content.linter.enabled, true);
  });

  await t.test('stryker.config.json existe para testes de mutação', () => {
    const strykerPath = path.join(rootDir, 'stryker.config.json');
    assert.ok(fs.existsSync(strykerPath), 'stryker.config.json deve existir');
    const content = JSON.parse(fs.readFileSync(strykerPath, 'utf8'));
    assert.equal(content.packageManager, 'npm');
    assert.ok(content.mutate.includes('server.js'));
  });

  await t.test('renv.lock existe para reprodutibilidade matemática em R', () => {
    const renvPath = path.join(rootDir, 'renv.lock');
    assert.ok(fs.existsSync(renvPath), 'renv.lock deve existir na raiz');
    const content = JSON.parse(fs.readFileSync(renvPath, 'utf8'));
    assert.ok(content.R?.Version, 'R.Version deve estar declarado');
    assert.ok(content.Packages?.kinship2, 'Pacote kinship2 deve estar declarado no lockfile');
  });

  await t.test('scripts respeitam o padrão NO_COLOR e não emitem códigos ANSI', () => {
    const ansiPrefix = `${String.fromCharCode(27)}[`;

    // Teste no scripts/demo.sh
    const demoOutput = cp.execSync('./scripts/demo.sh', {
      cwd: rootDir,
      env: { ...process.env, NO_COLOR: '1' },
      encoding: 'utf8'
    });
    assert.equal(
      demoOutput.includes(ansiPrefix),
      false,
      'scripts/demo.sh não deve conter códigos ANSI quando NO_COLOR=1'
    );

    // Teste no scripts/dev.sh --help
    const devHelpOutput = cp.execSync('./scripts/dev.sh --help', {
      cwd: rootDir,
      env: { ...process.env, NO_COLOR: '1' },
      encoding: 'utf8'
    });
    assert.equal(
      devHelpOutput.includes(ansiPrefix),
      false,
      'scripts/dev.sh --help não deve conter códigos ANSI quando NO_COLOR=1'
    );
  });

  await t.test('linter Biome aprova o repositório com Zero Warnings', () => {
    const res = cp.spawnSync('npm', ['run', 'lint'], {
      cwd: rootDir,
      encoding: 'utf8'
    });
    assert.equal(
      res.status,
      0,
      `npm run lint deve passar com código 0:\n${res.stdout}\n${res.stderr}`
    );
  });
});
