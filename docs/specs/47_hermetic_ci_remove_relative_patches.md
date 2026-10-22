# Especificação 47: Remoção de Patches Locais Relativos para CI Hermético

## 1. Problema
O arquivo `Cargo.toml` do `joaju` continha uma seção:
```toml
[patch."sparse+https://git.2lp.in/api/packages/lumen/cargo/"]
joaju-lirica = { path = "../joaju-lirica" }
joaju-pedigree = { path = "../joaju-pedigree" }
joaju-slides = { path = "../joaju-slides" }
```
No ambiente de CI soberano (onde apenas o repositório `joaju` é clonado em `/workspace/lumen/joaju`), os diretórios irmãos (`../joaju-*`) não existem, resultando no erro fatal do Cargo:
`error: patch location /workspace/lumen/joaju-lirica does not contain packages matching joaju-lirica`.

## 2. Solução
1. Remover a seção `[patch]` de caminhos locais relativos em `Cargo.toml`.
2. As dependências já estão publicadas e declaradas via `registry = "lumen"`.
3. Adicionar teste automatizado de integridade hermética assegurando ausência de `[patch]` relativo no `Cargo.toml`.
