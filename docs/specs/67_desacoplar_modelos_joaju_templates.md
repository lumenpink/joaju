# Especificação 67: Desacoplamento de Modelos Acadêmicos e Assets para joaju-templates

- **ID do Ticket:** #67
- **Status:** Em Progresso (DD-TDD)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Motivação
O compilador acadêmico Joaju (`joaju`) incorpora modelos tipográficos em Typst (como `trabalho-ufsc.typ`, `template.typ` / ABNT), folhas de estilo de citação CSL (`trabalho-ufsc.csl`, `abnt.csl`, `vancouver.csl`) e arquivos de referência de exemplo (`referencias.bib`) diretamente no binário via macros `include_str!`.

Com a expansão de novos modelos acadêmicos e estilos de pós-graduação, manter esses assets acoplados no código do compilador incha a crate principal e impede o consumo independente de templates por outras ferramentas do ecossistema.

Esta especificação define a extração dos assets tipográficos para a crate soberana `joaju-templates`, consumida pelo Joaju via registro soberano `gambi`.

## 2. Requisitos Técnicos
1. **Crate Soberana `joaju-templates`:**
   - Publicada no registro `gambi` (`v0.1.0`).
   - Exporta:
     - `template_ufsc() -> &'static str`
     - `template_abnt() -> &'static str`
     - `referencias_bib() -> &'static str`
     - `trabalho_ufsc_csl() -> &'static str`
     - `abnt_csl() -> &'static str`
     - `vancouver_csl() -> &'static str`
     - `get_template(name: &str) -> Option<&'static str>`
     - `get_csl(name: &str) -> Option<&'static str>`
     - Constantes estáticas equivalentes.

2. **Integração no `joaju`:**
   - Adicionar `joaju-templates = { version = "0.1.0", registry = "gambi" }` em `Cargo.toml`.
   - Substituir `include_str!("../../data/...")` em [src/compiler/mod.rs](file:///home/lumen/src/joaju/src/compiler/mod.rs) por chamadas às constantes/funções de `joaju_templates`.
   - Reexportar `templates` para conveniência pública.

3. **Garantia de Não-Regressão:**
   - Teste Red para auditar presença de `joaju-templates` e validação de resolução de templates.
   - Suíte de compilação passando: `compiler_spec`, `compiler_templates_spec`, `exhaustive_spec`.

## 3. Critérios de Aceitação
1. `Cargo.toml` contém `joaju-templates`.
2. Todos os testes de compilação de documentos e templates passam 100%.
3. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem avisos.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #67
- [x] Fase 2 (Branch): `feat/67-desacoplar-modelos-joaju-templates`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [ ] Fase 4 (Red Test): Criar teste red para dependência do joaju-templates
- [ ] Fase 5 (Green Implement): Integrar joaju-templates no compilador
- [ ] Fase 6 (Refactor): Reexportar de forma idiomática
- [ ] Fase 7 (Mutation Test): Validar testes de regressão de templates
- [ ] Fase 8 (Quality Gate): fmt e clippy sem warnings, testes verdes
- [ ] Fase 9 (Finish & Close): Concluído e pronto para merge
