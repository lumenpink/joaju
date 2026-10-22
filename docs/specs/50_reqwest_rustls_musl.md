# Especificação Técnica 50: Reqwest com Rustls para Cross-Compilação Estática Musl

## Objetivo
Eliminar a dependência transitiva de `native-tls` (`openssl-sys`) no pacote `joaju`, viabilizando compilação multiarquitetura estática para Linux MUSL (`x86_64-unknown-linux-musl` e `aarch64-unknown-linux-musl`) sem necessidade de bibliotecas C dinâmicas ou cabeçalhos do OpenSSL no sysroot de compilação cruzada.

## Requisitos
1. Declarar `reqwest` em `Cargo.toml` com `default-features = false` e features `["rustls-tls", "json"]`.
2. A árvore de dependências (`cargo tree -i openssl-sys`) não deve conter a crate `openssl-sys`.
3. Todos os testes de unidade, integração e pipelines multiarch devem compilar com sucesso.
