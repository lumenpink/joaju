# Spec do Ticket #71: Alinhar ao Perfil Lilya e Validar Multiarch

## 1. Contexto e Motivação
Padronização do perfil GOST Lilya (`https://lilya.2lp.in/profile.md`) e higienização terminológica no repositório canônico Joaju (`joaju`), assegurando integração hermética com `joaju-pdf` e publicação de artefatos multiplataforma.

## 2. Escopo Técnico
- Atualizar perfil GOST para Lilya e sincronizar diretivas.
- Higienizar termos imperiais remanescentes na esteira `ci.yml`.
- Atualizar teste `tests/audit_gitops_cascade.sh` para suportar `ci.yml`.
- Garantir que a compilação e publicação multiplataforma da biblioteca estática consuma os pacotes genéricos canônicos de `joaju-pdf-static`.
- Manter 100% de conformidade nos quality gates locais e no Gosplan audit (`cccp check --strict`).

## 3. Plano de Fases DD-TDD
- [x] Fase 1 (Ticket): #71 criado
- [x] Fase 2 (Branch): `chore/71-alinhar-ao-perfil-lilya-e-validar-multiarch` ativada
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Validação de conformidade
- [x] Fase 5 (Green Implement): Higienização e sincronização
- [x] Fase 6 (Refactor): Limpeza e clippy zero warnings
- [x] Fase 7 (Mutation/Adversarial): Testes adversariais e mutantes
- [x] Fase 8 (Quality Gate): Testes e clippy estritos
- [ ] Fase 9 (Finish): Encerramento com `cccp close` e integração

## 4. Conclusão
Perfil alinhado ao padrão Lilya com sucesso, terminologia higienizada e quality gates 100% aprovados.
