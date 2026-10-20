use joaju::engine::slides::{compile_slides_to_pdf, render_slides_markup, PresentationMode};

const SAMPLE_SLIDES: &str = r#"---
title: Fisiologia Cardiovascular - Joaju Integration
format: "16:9"
---

# Ciclo Cardíaco
Fechamento Mitral e Fase Isovolumétrica

???
### Notas do Orador
O fechamento da **valva mitral** marca o início da sístole ventricular.

---

# B3 (Terceira Bulha)
Sobrecarga de volume ventricular
"#;

#[test]
fn test_render_slides_markup_all_modes() {
    let slides_typst = render_slides_markup(SAMPLE_SLIDES, PresentationMode::Slides)
        .expect("Slides markup rendering failed");
    assert!(slides_typst.contains("paper: \"presentation-16-9\""));
    assert!(slides_typst.contains("Ciclo Cardíaco"));
    assert!(!slides_typst.contains("Notas do Orador"));

    let handout_typst = render_slides_markup(SAMPLE_SLIDES, PresentationMode::Handout)
        .expect("Handout markup rendering failed");
    assert!(handout_typst.contains("paper: \"a4\""));
    assert!(handout_typst.contains("stroke: 0.5pt + luma(180)"));
    assert!(handout_typst.contains("Notas do Orador"));

    let notes_typst = render_slides_markup(SAMPLE_SLIDES, PresentationMode::Notes)
        .expect("Notes markup rendering failed");
    assert!(notes_typst.contains("paper: \"a4\""));
    assert!(notes_typst.contains("= Ciclo Cardíaco"));
    assert!(notes_typst.contains("Notas do Orador"));
}

#[test]
fn test_compile_slides_to_pdf() {
    let pdf_bytes = compile_slides_to_pdf(SAMPLE_SLIDES, PresentationMode::Slides)
        .expect("PDF compilation of slides failed");
    assert!(pdf_bytes.starts_with(b"%PDF-"));
    assert!(pdf_bytes.len() > 500);

    let handout_bytes = compile_slides_to_pdf(SAMPLE_SLIDES, PresentationMode::Handout)
        .expect("PDF compilation of handout failed");
    assert!(handout_bytes.starts_with(b"%PDF-"));
    assert!(handout_bytes.len() > 500);

    let notes_bytes = compile_slides_to_pdf(SAMPLE_SLIDES, PresentationMode::Notes)
        .expect("PDF compilation of notes failed");
    assert!(notes_bytes.starts_with(b"%PDF-"));
    assert!(notes_bytes.len() > 500);
}

#[test]
fn test_aspect_ratio_4_3_slides() {
    let md = "---\ntitle: Standard Deck\nformat: \"4:3\"\n---\n# Slide 1\nConteudo";
    let typst = render_slides_markup(md, PresentationMode::Slides).unwrap();
    assert!(typst.contains("presentation-4-3"));
}

#[test]
fn test_invalid_notes_delimiter_returns_error() {
    let broken_md = "???\nOrphan notes";
    let res = render_slides_markup(broken_md, PresentationMode::Slides);
    assert!(res.is_err());
}
