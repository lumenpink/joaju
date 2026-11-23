# Spec #94: Atualizar Perfil Lilya para Versão 2.2.0

## 1. Contexto e Motivação
Com o lançamento do GOST Lilya 2.2.0 e Kollontai 2.2.0, o compilador Rust de referência foi atualizado para rustc >= 1.100.0 e as rotas canônicas de regras foram unificadas. O repositório joaju precisa atualizar suas definições de governança (.gost/GOST.md e .gost/rules.toml) para herdar as regras e padrões da versão 2.2.0.

## 2. Escopo
- Atualizar referências de `2.1.0` para `2.2.0` em `.gost/GOST.md` e `.gost/rules.toml`.
- Atualizar a versão mínima do compilador Rust para `1.100.0`.
- Validar conformidade estrita com `cccp check --strict`.
- Validar integridade da crate com `cargo test`.

## 3. Plano de Implementação (GOST DD-TDD)
1. Fase 3: Especificação técnica (este documento).
2. Fase 4: Teste Red (N/A - governança).
3. Fase 5: Atualização de `.gost/GOST.md` e `.gost/rules.toml`.
4. Fase 6-7: Refatoração e Mutação (N/A).
5. Fase 8: Quality Gate (`cargo test` e `cccp check --strict`).
6. Fase 9: Fechamento do ticket e sincronização.

## 4. Árvore de Decisão e Guardiã da Natasha
- **Por que atualizar imediatamente?** Manter o repositório sincronizado com a Metanorma 2.2.0 previne quebras nas pipelines do CI que utilizam o container `gost-ci:1.100.0`. A Natasha terá histórico claro da evolução da toolchain.
- **Alternativa descartada:** Fazer o bump apenas no CI sem atualizar a metanorma no repositório geraria divergência em `cccp check --strict`.

## 5. Resultados e Validação
- cccp check: Pendente execução do Quality Gate.
- cargo test: Pendente execução do Quality Gate.
