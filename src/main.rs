use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "joaju",
    about = "Scientific academic compiler and typographical orchestrator in Rust",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile Markdown document to PDF via Typst
    Compile {
        /// Input Markdown file path (reads from stdin if omitted or '-')
        #[arg(value_name = "INPUT")]
        input: Option<PathBuf>,

        /// Output PDF file path (writes to stdout if omitted or '-')
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Typst template profile (e.g. 'abnt', 'trabalho-ufsc', 'vancouver')
        #[arg(short, long, value_name = "TEMPLATE")]
        template: Option<String>,

        /// Compile in presentation slides mode (16:9 / 4:3)
        #[arg(long, conflicts_with_all = ["handout", "notes"])]
        slides: bool,

        /// Compile in handout / apostila mode (A4 with thumbnails and notes)
        #[arg(long, conflicts_with_all = ["slides", "notes"])]
        handout: bool,

        /// Compile in continuous notes / roteiro mode
        #[arg(long, conflicts_with_all = ["slides", "handout"])]
        notes: bool,
    },

    /// Generate presentation slides, handouts, or speaker notes from Markdown
    Slides {
        /// Input Markdown file path (reads from stdin if omitted)
        #[arg(value_name = "INPUT")]
        input: Option<PathBuf>,

        /// Output file path (writes to stdout if omitted)
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Generate presentation slides (16:9 / 4:3)
        #[arg(long, conflicts_with_all = ["handout", "notes"])]
        slides: bool,

        /// Generate handout / apostila (A4 with thumbnails and notes)
        #[arg(long, conflicts_with_all = ["slides", "notes"])]
        handout: bool,

        /// Generate continuous speaker notes / roteiro
        #[arg(long, conflicts_with_all = ["slides", "handout"])]
        notes: bool,

        /// Emit raw Typst markup instead of compiling to PDF
        #[arg(long)]
        typst: bool,
    },

    /// Render isolated pedigree, chemical structure, or table block to SVG/Typst
    Render {
        /// Input file containing code block or tabular CSV (reads from stdin if omitted)
        #[arg(value_name = "INPUT")]
        input: Option<PathBuf>,

        /// Engine name ('pedtools', 'kinship2', 'table', 'openchemlib', 'openbabel')
        #[arg(short, long, value_name = "ENGINE")]
        engine: Option<String>,

        /// Output file path (writes to stdout if omitted)
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
    },

    /// Run preprocessors on Markdown (resolve wikilinks, tables, gists)
    Preprocess {
        /// Input Markdown file path (reads from stdin if omitted)
        #[arg(value_name = "INPUT")]
        input: Option<PathBuf>,

        /// Output file path (writes to stdout if omitted)
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
    },

    /// Extract OpenGraph and rich metadata from a URL with SSRF protection
    Unfurl {
        /// Target URL to unfurl
        #[arg(value_name = "URL")]
        url: String,

        /// Generate archival PDF snapshot encoded in base64
        #[arg(long)]
        archive: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            input,
            output,
            template,
            slides,
            handout,
            notes,
        } => {
            let markdown = read_input(input.as_deref())?;
            if slides || handout || notes {
                let mode = if handout {
                    joaju::engine::slides::PresentationMode::Handout
                } else if notes {
                    joaju::engine::slides::PresentationMode::Notes
                } else {
                    joaju::engine::slides::PresentationMode::Slides
                };
                let pdf_bytes = joaju::engine::slides::compile_slides_to_pdf(&markdown, mode)
                    .map_err(io::Error::other)?;
                write_output_bytes(output.as_deref(), &pdf_bytes)?;
            } else {
                let pdf_bytes =
                    joaju::compile_markdown_to_pdf_with_options(&markdown, template.as_deref())
                        .map_err(io::Error::other)?;
                write_output_bytes(output.as_deref(), &pdf_bytes)?;
            }
        }
        Commands::Slides {
            input,
            output,
            slides: _,
            handout,
            notes,
            typst,
        } => {
            let markdown = read_input(input.as_deref())?;
            let mode = if handout {
                joaju::engine::slides::PresentationMode::Handout
            } else if notes {
                joaju::engine::slides::PresentationMode::Notes
            } else {
                joaju::engine::slides::PresentationMode::Slides
            };

            if typst {
                let markup = joaju::engine::slides::render_slides_markup(&markdown, mode)
                    .map_err(io::Error::other)?;
                write_output_str(output.as_deref(), &markup)?;
            } else {
                let pdf_bytes = joaju::engine::slides::compile_slides_to_pdf(&markdown, mode)
                    .map_err(io::Error::other)?;
                write_output_bytes(output.as_deref(), &pdf_bytes)?;
            }
        }
        Commands::Render {
            input,
            engine,
            output,
        } => {
            let code = read_input(input.as_deref())?;
            let is_chem = engine
                .as_deref()
                .map(|e| matches!(e, "openchemlib" | "openbabel"))
                .unwrap_or(false)
                || code.contains("```openchemlib")
                || code.contains("```openbabel");

            let is_table = engine.as_deref().map(|e| e == "table").unwrap_or(false)
                || code.contains("```table");

            let content = if is_table {
                let clean = code.trim();
                let body = if clean.starts_with("```table") {
                    clean
                        .trim_start_matches("```table")
                        .trim_end_matches("```")
                        .trim()
                } else {
                    clean
                };
                joaju::preprocessor::render_single_table(body)
                    .map_err(|e| io::Error::other(e.to_string()))?
            } else if is_chem {
                let (_, svg) =
                    joaju::render_chem(&code, engine.as_deref()).map_err(io::Error::other)?;
                svg
            } else {
                let (_, svg) =
                    joaju::render_pedigree(&code, engine.as_deref()).map_err(io::Error::other)?;
                svg
            };

            write_output_str(output.as_deref(), &content)?;
        }
        Commands::Preprocess { input, output } => {
            let markdown = read_input(input.as_deref())?;
            let processed = joaju::preprocess_markdown(&markdown);
            write_output_str(output.as_deref(), &processed)?;
        }
        Commands::Unfurl { url, archive } => {
            let response = joaju::unfurl_url(&url, archive)
                .await
                .map_err(io::Error::other)?;
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
    }

    Ok(())
}

fn read_input(path: Option<&std::path::Path>) -> io::Result<String> {
    match path {
        Some(p) if p != std::path::Path::new("-") => fs::read_to_string(p),
        _ => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn write_output_bytes(path: Option<&std::path::Path>, data: &[u8]) -> io::Result<()> {
    match path {
        Some(p) if p != std::path::Path::new("-") => fs::write(p, data),
        _ => {
            io::stdout().write_all(data)?;
            io::stdout().flush()
        }
    }
}

fn write_output_str(path: Option<&std::path::Path>, data: &str) -> io::Result<()> {
    match path {
        Some(p) if p != std::path::Path::new("-") => fs::write(p, data),
        _ => {
            io::stdout().write_all(data.as_bytes())?;
            io::stdout().flush()
        }
    }
}
