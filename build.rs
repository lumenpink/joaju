use std::env;
use std::path::PathBuf;
use std::process::Command;

const JOAJU_PDF_REQUIRED_VERSION: &str = "0.1.0-dev.47";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap_or_else(|_| "x86_64-unknown-linux-musl".to_string());

    // 1. Verificar se existe biblioteca estática local construída no repositório joaju-pdf
    let local_paths = [
        PathBuf::from("/home/lumen/src/joaju-pdf/target/x86_64-unknown-linux-musl/release"),
        PathBuf::from(format!("/home/lumen/src/joaju-pdf/target/{target}/release")),
        PathBuf::from("/home/lumen/src/joaju-pdf/target/release"),
        PathBuf::from(format!("/home/lumen/src/joaju-pdf/target/{target}/debug")),
        PathBuf::from("/home/lumen/src/joaju-pdf/target/debug"),
    ];

    let mut found_dir = None;
    for p in &local_paths {
        if p.join("libjoaju_pdf.a").exists() || p.join("joaju_pdf.lib").exists() {
            found_dir = Some(p.clone());
            break;
        }
    }

    let link_dir = if let Some(dir) = found_dir {
        dir
    } else {
        // 2. Se não existir localmente, baixar o artefato da Forgejo Generic Packages
        let target_artifact = if target.contains("windows") {
            "libjoaju_pdf-x86_64-windows.a"
        } else if target.contains("aarch64") {
            "libjoaju_pdf-aarch64-linux.a"
        } else {
            "libjoaju_pdf-x86_64-linux.a"
        };

        let dest_lib = out_dir.join("libjoaju_pdf.a");
        if !dest_lib.exists() {
            let version = env::var("JOAJU_PDF_STATIC_VERSION")
                .unwrap_or_else(|_| JOAJU_PDF_REQUIRED_VERSION.to_string());
            let url = format!(
                "https://gambi.2lp.in/api/packages/lumen/generic/joaju-pdf-static/{version}/{target_artifact}"
            );

            let token = env::var("GAMBI_2LP_IN_TOKEN")
                .or_else(|_| env::var("GIT_2LP_TOKEN"))
                .or_else(|_| env::var("REGISTRY_TOKEN"))
                .or_else(|_| env::var("FORGEJO_TOKEN"))
                .unwrap_or_else(|_| "2c1912103419968a307522910a2ebe375fa69ecc".to_string());

            let status = Command::new("curl")
                .arg("-s")
                .arg("-f")
                .arg("-H")
                .arg(format!("Authorization: token {token}"))
                .arg("-o")
                .arg(&dest_lib)
                .arg(&url)
                .status();

            if status.is_err() || !status.unwrap().success() || !dest_lib.exists() {
                let fallbacks = ["latest-dev", "0.1.0-dev", "0.1.0"];
                for fb in fallbacks {
                    let fallback_url = format!(
                        "https://gambi.2lp.in/api/packages/lumen/generic/joaju-pdf-static/{fb}/{target_artifact}"
                    );
                    let _ = Command::new("curl")
                        .arg("-s")
                        .arg("-f")
                        .arg("-H")
                        .arg(format!("Authorization: token {token}"))
                        .arg("-o")
                        .arg(&dest_lib)
                        .arg(fallback_url)
                        .status();
                    if dest_lib.exists() {
                        break;
                    }
                }
            }
        }
        out_dir
    };

    println!("cargo:rustc-link-search=native={}", link_dir.display());
    println!("cargo:rustc-link-lib=static=joaju_pdf");
    println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    println!("cargo:rerun-if-changed=build.rs");
}
