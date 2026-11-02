# Spec 77: Integrar Notificação ao Kombinat e Expurgar Legados no ci.yml

## 1. Contexto e Motivação
Com o Kombinat ativo gerenciando a orquestração em cascata do ecossistema Joaju, as esteiras de CI/CD precisam notificar o Kombinat via HTTP (`POST http://127.0.0.1:4040/api/cascade/dispatch`) para propagar atualizações aos pacotes finais dependentes (`mandu`, `hendu`, `joaju-web`). Além disso, publicações sob tags mutáveis como `/latest/` devem ser eliminadas, garantindo que apenas versões SemVer canônicas e reprodutíveis sejam publicadas no Generic Packages.

## 2. Requisitos Técnicos
- Remover o bloco de envio de artefatos estáticos sob o alias `latest` em `.forgejo/workflows/ci.yml`.
- Adicionar o step `Notify Kombinat Cascade` após a etapa de publicação de pacotes estáticos.
- Disparar requisição `POST http://127.0.0.1:4040/api/cascade/dispatch` com payload JSON contendo o nome do pacote (`joaju`) e a versão liberada.
- Assegurar tolerância a falhas caso o daemon esteja indisponível durante testes simulados no CI.

## 3. Plano de Testes (DD-TDD)
- Fase 4: Criar `tests/ci_workflow_spec.rs` verificando ausência de `/latest/` no fluxo de pacotes e presença da chamada ao Kombinat.
- Fase 5: Ajustar `.forgejo/workflows/ci.yml` fazendo o teste passar.
- Fase 8: Formatação, clippy e auditoria estrita CCCP (100% de conformidade Gosplan).

## 4. Status de Conclusão
- [x] Step de notificação ao Kombinat adicionado ao `.forgejo/workflows/ci.yml`.
- [x] Alias mutável `latest` expurgado da publicação de binários estáticos.
- [x] Teste de esteira automatizado aprovado.
- [x] 100% de conformidade Gosplan.
