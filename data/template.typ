#let abnt(
  doc,
  fonte: "Liberation Sans",
  tamanho: 12pt,
) = {
  // 1. Margens Oficiais ABNT (Superior/Esquerda: 3cm | Inferior/Direita: 2cm)
  set page(
    paper: "a4",
    margin: (
      top: 3cm,
      left: 3cm,
      right: 2cm,
      bottom: 2cm,
    ),
    header: locate(loc => {
      // Numeração no canto superior direito a partir da página 2
      let page_number = counter(page).at(loc).first()
      if page_number > 1 [
        #set text(font: fonte, size: 10pt)
        #align(right)[#page_number]
      ]
    })
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
    first-line-indent: 1.25cm
  )

  // 3. Hierarquia de Títulos (NBR 6024)
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

  // 4. Figuras e Legendas (Topo ou Base a 10pt)
  show figure: set text(size: 10pt)
  show figure.caption: it => [
    #set text(size: 10pt)
    #it
  ]

  // 5. Citações Diretas Longas (> 3 linhas: recuo de 4cm, fonte 10pt, entrelinha simples)
  show quote.where(block: true): it => [
    #set text(size: 10pt)
    #set par(leading: 0.35em, first-line-indent: 0cm)
    #pad(left: 4cm)[#it.body]
  ]

  // Renderiza o corpo do documento
  doc
}