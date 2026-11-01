# Spec #73 — Publicar Staticlibs sob Alias Latest e Consumir Latest no Build

## 1. Contexto e Objetivo
1. Garantir que a esteira CI do repositório `joaju` publique as bibliotecas estáticas multiarquitetura (`libjoaju-x86_64-linux.a`, `libjoaju-aarch64-linux.a`, `libjoaju-x86_64-windows.a`) não apenas na tag canônica `${VERSION}` e `${BASE_VERSION}`, mas também sob o alias `latest` em Forgejo Generic Packages (`joaju-static`).
2. Atualizar o script de build (`build.rs`) para consumir `joaju-pdf-static` priorizando por padrão o alias `latest` (ou via variável `JOAJU_PDF_STATIC_VERSION`), com fallback resiliente.

## 2. Requisitos
1. No job de compilação/publicação de staticlibs em `.forgejo/workflows/ci.yml`, para cada destino `${DEST_NAME}`, realizar o upload também para `latest` com tratamento de HTTP 201/200/409.
2. Em `build.rs`, utilizar `latest` como versão padrão para resolução de `joaju-pdf-static`.
3. Adicionar asserções em testes automatizados (`tests/test_banca_pipeline_canonical.sh` e/ou teste rust) validando essas configurações.

## 3. Plano de Ação (GOST DD-TDD)
- Fase 3: Especificação técnica (este documento).
- Fase 4: Teste vermelho validando a publicação de `latest` no `ci.yml` e o consumo no `build.rs`.
- Fase 5: Implementação verde em `.forgejo/workflows/ci.yml` e `build.rs`.
- Fase 6: Refatoração e limpeza.
- Fase 7: Teste de mutação.
- Fase 8: Quality Gate (`cargo fmt`, `cargo clippy`, `cargo test`, `cccp check --strict`).
- Fase 9: Conclusão, merge e push.
