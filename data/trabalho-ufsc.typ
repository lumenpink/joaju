
#let legend_symbol(shape: "square", fill_color: none, stroke_color: black, size: 10pt) = {
  box(width: size, height: size, baseline: 15%)[
    #if shape == "square" [
      #rect(width: 100%, height: 100%, fill: fill_color, stroke: 0.8pt + stroke_color)
    ] else if shape == "circle" [
      #circle(radius: size / 2, fill: fill_color, stroke: 0.8pt + stroke_color)
    ]
  ]
}

#let proband_symbol() = {
  box(baseline: 15%)[
    #text(size: 10pt, weight: "bold")[↗ P]
  ]
}

#let trabalho_ufsc(
  doc,
  instituicao: "UNIVERSIDADE FEDERAL DE SANTA CATARINA\nCENTRO DE CIÊNCIAS DA SAÚDE\nCURSO DE GRADUAÇÃO EM MEDICINA",
  autor: "Lumen Pink",
  titulo: "ELABORAÇÃO DE HEREDOGRAMA FAMILIAR",
  subtitulo: none,
  local: "FLORIANÓPOLIS",
  ano: "2026",
  fonte: "Liberation Sans",
  tamanho: 12pt,
) = {
  // 1. Margens Oficiais ABNT / UFSC (Superior/Esquerda: 3cm | Inferior/Direita: 2cm)
  set page(
    paper: "a4",
    margin: (
      top: 3cm,
      left: 3cm,
      right: 2cm,
      bottom: 2cm,
    ),
    header: context {
      // Numeração no canto superior direito a partir da página 2 (após a capa)
      let page_number = counter(page).get().first()
      if page_number > 1 [
        #set text(font: fonte, size: 10pt)
        #align(right)[#page_number]
      ]
    }
  )

  // 2. Tipografia Base e Espaçamento
  set text(
    font: fonte,
    size: tamanho,
    lang: "pt",
    region: "BR"
  )

  // Espaçamento 1.5 ABNT (no Typst isso equivale a ~0.65em de leading)
  set par(
    justify: true,
    leading: 0.65em,
    first-line-indent: (amount: 1.25cm, all: true)
  )

  // 3. Hierarquia de Títulos Automática (NBR 6024)
  set heading(numbering: "1.1 ")

  show heading.where(level: 1): it => block(width: 100%, breakable: false)[
    #set text(size: 12pt, weight: "bold")
    #v(1.5em)
    #upper(it)
    #v(1em)
  ]

  show heading.where(level: 2): it => block(width: 100%, breakable: false)[
    #set text(size: 12pt, weight: "bold")
    #v(1.2em)
    #it
    #v(0.8em)
  ]

  show heading.where(level: 3): it => block(width: 100%, breakable: false)[
    #set text(size: 12pt, weight: "regular")
    #v(1em)
    #it
    #v(0.6em)
  ]

  // 4. Figuras e Legendas
  show figure: set text(size: 10pt)
  show figure.caption: it => [
    #set text(size: 10pt)
    #it
  ]

    // 6. Tabelas Acadêmicas
  show figure.where(kind: table): set figure.caption(position: top)
  show table.cell: it => [
    #set text(size: 10pt)
    #it
  ]

  // 5. Citações Diretas Longas
  show quote.where(block: true): it => [
    #set text(size: 10pt)
    #set par(leading: 0.35em, first-line-indent: 0cm)
    #pad(left: 4cm)[#it.body]
  ]

  // ==========================================
  // CAPA INSTITUCIONAL UFSC (PÁGINA 1)
  // ==========================================
  align(center)[
    #set text(weight: "bold", size: 12pt)
    #upper(instituicao)

    #v(5cm)
    #upper(autor)

    #v(5cm)
    #upper(titulo)
    #if subtitulo != none [
      #v(0.3em)
      #text(weight: "regular")[#subtitulo]
    ]

    #v(1fr)
    #upper(local)\
    #ano
  ]

  pagebreak()

  // Renderiza o corpo do trabalho
  doc
}
