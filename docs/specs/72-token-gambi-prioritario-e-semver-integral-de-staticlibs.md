# Spec do Ticket #72: Token Gambi Prioritário e SemVer Integral de Staticlibs

## 1. Contexto e Motivação
A esteira de publicação do `joaju` falhou no upload das bibliotecas estáticas multiarquitetura para os Generic Packages do Forgejo devido à ausência do segredo preferencial `GAMBI_2LP_IN_TOKEN` e tratamento de prefixo `Bearer`, além da necessidade de preservar o SemVer 2.0.0 integral com metadados de build (`+...`) sem qualquer truncamento.

## 2. Escopo Técnico
- Priorizar `GAMBI_2LP_IN_TOKEN` como token preferencial em `.forgejo/workflows/ci.yml` e `build.rs`.
- No passo `Publish Multiarch Staticlibs to Forgejo Generic Packages`:
  - Utilizar `RAW_TOKEN="${TOKEN#Bearer }"`.
  - Publicar a biblioteca estática com a versão integral SemVer 2.0.0 (`${VERSION}`).
  - Também registrar sob o identificador canônico de release base (`${BASE_VERSION}`, ex: `0.2.0`).
  - Utilizar `curl -s -f --http1.1 --retry 3 -X PUT`.
- No `build.rs`:
  - Priorizar `GAMBI_2LP_IN_TOKEN` na resolução de credenciais.

## 3. Plano de Fases DD-TDD
- [x] Fase 1 (Ticket): #72 criado
- [x] Fase 2 (Branch): `fix/72-token-gambi-prioritario-e-semver-integral-de-staticlibs` ativada
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Validação de teste que audite a presença de GAMBI_2LP_IN_TOKEN no workflow
- [x] Fase 5 (Green Implement): Ajustes no ci.yml e build.rs
- [x] Fase 6 (Refactor): Limpeza de código e clippy zero warnings
- [x] Fase 7 (Mutation/Adversarial): Testes mutantes
- [x] Fase 8 (Quality Gate): Testes e conformidade estrita cccp
- [x] Fase 9 (Finish): Encerramento com `cccp close` e integração

## 4. Conclusão
Configuração ajustada com sucesso: GAMBI_2LP_IN_TOKEN estabelecido como prioritário, upload de bibliotecas estáticas no Forgejo preserva o SemVer integral e publica também o alias de release base (0.2.0), com 100% de conformidade nos testes e auditorias.
