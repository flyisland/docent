use crate::cli::{DocsArgs, DocsCommand};
use std::io::Write;
use std::path::{Path, PathBuf};

const CANONICAL_SPECIFICATION: &str =
    include_str!("../../docs/reference/software-project-documentation-specification.md");
const POLICY_VERSION: u8 = 2;
const DEFAULT_EXPORT_NAME: &str = "software-project-documentation-specification.md";

fn rendered_specification() -> String {
    format!(
        "<!-- Canonical source: Docent bundled specification; tool version: {}; policy version: {} -->\n\n{}",
        env!("CARGO_PKG_VERSION"),
        POLICY_VERSION,
        CANONICAL_SPECIFICATION
    )
}

fn export_path(destination: &Path, file: bool) -> PathBuf {
    if !file && (destination.is_dir() || destination.extension().is_none()) {
        destination.join(DEFAULT_EXPORT_NAME)
    } else {
        destination.to_path_buf()
    }
}

pub fn run(args: DocsArgs) -> i32 {
    let specification = rendered_specification();
    match args.command {
        DocsCommand::Show => {
            print!("{specification}");
            0
        }
        DocsCommand::Export { destination, file } => {
            let path = export_path(&destination, file);
            if path.exists() {
                eprintln!("refusing to overwrite existing file: {}", path.display());
                return 1;
            }
            if let Some(parent) = path.parent()
                && let Err(error) = std::fs::create_dir_all(parent)
            {
                eprintln!("failed to create {}: {error}", parent.display());
                return 1;
            }
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(mut file) => match file.write_all(specification.as_bytes()) {
                    Ok(()) => {
                        println!("exported {}", path.display());
                        0
                    }
                    Err(error) => {
                        eprintln!("failed to write {}: {error}", path.display());
                        1
                    }
                },
                Err(error) => {
                    eprintln!("failed to export {}: {error}", path.display());
                    1
                }
            }
        }
    }
}
