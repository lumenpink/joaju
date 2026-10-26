use joaju::compiler::{compile_markdown_to_pdf, preprocess_code_blocks};
use joaju::engine::wireframe::render_wireframe;

#[test]
fn test_render_wireframe_simple_form() {
    let source = r#"
    Login:    | "usuaria@exemplo.com"
    Senha:    | "*******"
    [Entrar]  | [Cancelar]
    "#;

    let typst = render_wireframe(source).expect("Wireframe should parse and render successfully");

    assert!(typst.contains("#grid"), "Output must contain Typst grid");
    assert!(typst.contains("usuaria@exemplo.com"), "Output must contain input value");
    assert!(typst.contains("Entrar"), "Output must contain button label");
    assert!(typst.contains("Cancelar"), "Output must contain button label");
}

#[test]
fn test_render_wireframe_salt_window_with_controls() {
    let source = r#"{+
      "Janela de Configuração"
      --
      [X] Notificações ativas | [ ] Modo escuro
      (X) Servidor Primário   | ( ) Servidor Secundário
      ^Selecionar Região^     | <Tag: Produção>
      (O) Auto-backup         | [========.. 80%]
    }"#;

    let typst = render_wireframe(source).expect("Salt window syntax should parse successfully");

    assert!(typst.contains("#grid") || typst.contains("#rect"));
    assert!(typst.contains("Janela de Configuração"));
    assert!(typst.contains("Notificações ativas"));
    assert!(typst.contains("Servidor Primário"));
    assert!(typst.contains("Produção"));
}

#[test]
fn test_render_wireframe_tabs() {
    let source = r#"
    {/ Geral | Segurança | Notificações }
    Conteúdo da aba ativa
    "#;

    let typst = render_wireframe(source).expect("Tabs should render successfully");
    assert!(typst.contains("Geral"));
    assert!(typst.contains("Segurança"));
    assert!(typst.contains("Notificações"));
}

#[test]
fn test_preprocess_wireframe_and_salt_blocks() {
    let md = r#"# Documentação de Tela

```wireframe
Nome: | "Maria"
[Salvar]
```

E outra tela legada:

```salt
{+
  [X] Ativo
}
```
"#;

    let (processed, assets) = preprocess_code_blocks(md);

    // Native Typst rendering generates 0 image files
    assert!(assets.is_empty(), "Wireframe must not produce raster/SVG image assets");
    assert!(!processed.contains("```wireframe"));
    assert!(!processed.contains("```salt"));
    assert!(processed.contains("Salvar"));
    assert!(processed.contains("Ativo"));
}

#[test]
fn test_compile_markdown_with_wireframe_to_pdf() {
    let md = r#"
# Relatório de Arquitetura

Abaixo está o wireframe da nova interface:

```wireframe
{+
  "Autenticação"
  --
  Usuário: | "admin"
  Senha:   | "••••••"
  [ Entrar ]
}
```
"#;

    let pdf = compile_markdown_to_pdf(md).expect("Markdown with wireframe must compile to PDF");
    assert!(!pdf.is_empty(), "PDF bytes should not be empty");
    assert!(pdf.starts_with(b"%PDF-"), "Output must be valid PDF starting with %PDF-");
}
