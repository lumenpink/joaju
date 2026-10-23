# Spec: Desmistificação e Apresentação dos Módulos Visuais - Joaju (#49)

## Contexto
O Joaju integra módulos especializados para composição gráfica e visual em documentos. Descrições anteriores recorriam a chavões como "motor soberano em Rust" ou jargões opacos de compilador, em vez de explicar o papel visual prático que cada utilitário desempenha para quem lê e escreve.

## Objetivo
Atualizar meta-descrições e textos estáticos dos módulos Joaju (`joaju-csv`, `joaju-pedigree`, `joaju-unfurl`), substituindo o chavão recorrente "motor soberano em Rust" por explicações claras e diretas da função de cada utilitário:
- `joaju-csv`: formatação de **tabelas** nítidas no padrão acadêmico a partir de planilhas e dados CSV;
- `joaju-pedigree`: geração de **diagramas** genealógicos e heredogramas clínicos;
- `joaju-unfurl`: composição de **cartões** visuais com prévia de links e metadados de leitura.

## Critérios de Aceite
1. Ausência do chavão "motor soberano" ou "soberano em rust" nas descrições dos módulos em `site/index.html` e `README.md`.
2. Presença de descrições claras para as utilidades visuais dos módulos: tabelas (`joaju-csv`), diagramas (`joaju-pedigree`) e cartões (`joaju-unfurl`).
3. Teste automatizado validando a ausência do jargão e a presença das explicações visuais de cada módulo.
