# GOST Language Specification: R
**Identificador da Stack:** `r`  
**Padrão Base:** GOST `1.0.0`

---

## 1. Visão Geral e Filosofia
Esta especificação define os requisitos de engenharia, reprodutibilidade matemática e padrões para desenvolvimento em **R** (versão 4.3 ou superior) no ecossistema GOST.

A premissa central em R sob o padrão GOST é a **Reprodutibilidade Absoluta**: qualquer análise, script ou pacote deve produzir exatamente os mesmos resultados em qualquer máquina, sem depender de pacotes instalados globalmente no sistema operacional hospedeiro.

---

## 2. Ferramental Mandatório de Testes & Qualidade

| Requisito GOST | Ferramenta Padrão | Comando de Validação no CI |
|---|---|---|
| **Test Runner** | **testthat** (3ª edição) | `Rscript -e 'testthat::test_dir("tests/testthat")'` |
| **Cobertura (≥ 90%)** | **covr** | `Rscript -e 'stopifnot(covr::percent_coverage(covr::package_coverage()) >= 90)'` |
| **Linter (Zero Warnings)** | **lintr** | `Rscript -e 'stopifnot(length(lintr::lint_dir()) == 0)'` |
| **Formatação Determinística** | **styler** | `Rscript -e 'stopifnot(styler::style_dir(dry="fail")$changed == FALSE)'` |
| **Documentação Técnica** | **roxygen2** | `Rscript -e 'roxygen2::roxygenise()'` |

---

## 3. Gestão de Dependências e Reprodutibilidade (Twelve-Factor R)
1. **Ambiente Isolado com `renv`:** Todo projeto GOST em R **DEVE** utilizar o **`renv`** para gerenciar dependências de forma estritamente isolada e declarativa.
2. **Lockfile Obrigatório:** O arquivo **`renv.lock`** **DEVE** estar commitado na raiz do repositório, congelando versões exatas de todos os pacotes e do próprio runtime R.
3. **Restauração Hermética em CI:**
   ```bash
   Rscript -e 'renv::restore(clean = TRUE)'
   ```
4. **Proibição de `install.packages()` Solto:** Scripts de análise ou produção **NUNCA DEVEM** chamar `install.packages()` diretamente no código. Todas as dependências devem ser restauradas via `renv::restore()`.

---

## 4. Estrutura de Diretórios Recomendada
Para pacotes e projetos de análise orientados a domínio:

```text
.
├── renv.lock                   # Lockfile imutável de pacotes R (Twelve-Factor)
├── renv/                       # Ativador do ambiente isolado (renv/activate.R)
├── DESCRIPTION                 # Metadados do pacote/projeto e dependências
├── NAMESPACE                   # Exportações e importações geradas por roxygen2
├── R/                          # Funções puras, modelos e lógica de domínio
│   ├── model.R
│   └── transformations.R
├── tests/                      # Validação automatizada
│   ├── testthat.R              # Ponto de entrada do test runner
│   └── testthat/               # Especificações (test_*.R)
├── data/ / inst/extdata/       # Fixtures e dados de validação imutáveis
└── scripts/                    # Scripts executáveis de pipeline (CLI)
```

---

## 5. Disciplina de Streams Unix e Scripts CLI em R
1. **Segregação de Saída:**
   - **`stdout`:** Dados puros de saída tabular (ex: via `cat()`, `writeLines()` ou `readr::write_csv(..., stdout())`).
   - **`stderr`:** Mensagens informativas de progresso, alertas e logs diagnósticos (usar `message()`, `warning()` ou `cat(..., file = stderr())`).
2. **Resolução de Diretórios XDG:**
   Caches de dados e resultados temporários respeitam `$XDG_*`:
   ```r
   get_cache_dir <- function(app_name = "meu-app") {
     base <- Sys.getenv("XDG_CACHE_HOME", unset = file.path(Sys.getenv("HOME"), ".cache"))
     file.path(base, app_name)
   }
   ```
3. **Tratamento de Exit Codes:**
   Em caso de falha nos scripts, finalizar com código semântico via `quit(status = 1, save = "no")`.

---

## 6. Containers OCI para R
1. **Imagem Base Imutável:** Utilização de imagens mantidas pelo projeto Rocker com tags específicas de versão:
   ```dockerfile
   FROM rocker/r-ver:4.4.1 AS builder
   WORKDIR /app
   COPY renv.lock renv/activate.R ./
   RUN Rscript -e 'install.packages("renv"); renv::restore()'

   FROM rocker/r-ver:4.4.1
   USER 1000:1000
   WORKDIR /app
   COPY --from=builder /app /app
   COPY . .
   ENTRYPOINT ["Rscript", "scripts/run.R"]
   ```
