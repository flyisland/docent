use crate::cli::{DocsArgs, DocsCommand, ProjectPolicyCommand};
use crate::model::{FrontSplit, split_front_matter};
use serde::Deserialize;
use std::io::Write;
use std::path::{Path, PathBuf};

const CANONICAL_SPECIFICATION: &str =
    include_str!("../../docs/reference/software-project-documentation-specification.md");
pub const POLICY_VERSION: u8 = 3;
const DEFAULT_EXPORT_NAME: &str = "software-project-documentation-specification.md";
const PROJECT_POLICY_EXPORT_NAME: &str = "project-docs-readme.md";
const PROJECT_POLICY_TEMPLATE: &str = include_str!("../../docs/templates/project-docs-readme.md");

#[derive(Deserialize)]
struct ProjectPolicyMetadata {
    managed_by: Option<String>,
    policy_version: Option<u8>,
}

fn rendered_specification() -> String {
    format!(
        "<!-- Canonical source: Docent bundled specification; tool version: {}; policy version: {} -->\n\n{}",
        env!("CARGO_PKG_VERSION"),
        POLICY_VERSION,
        CANONICAL_SPECIFICATION
    )
}

pub fn rendered_project_policy() -> String {
    PROJECT_POLICY_TEMPLATE.replace("{tool_version}", env!("CARGO_PKG_VERSION"))
}

pub fn managed_project_policy_version(content: &str) -> Option<u8> {
    let FrontSplit::Ok { yaml, .. } = split_front_matter(content) else {
        return None;
    };
    let metadata = serde_yaml::from_str::<ProjectPolicyMetadata>(&yaml).ok()?;
    (metadata.managed_by.as_deref() == Some("docent"))
        .then_some(metadata.policy_version)
        .flatten()
}

fn export_path(destination: &Path, file: bool, default_name: &str) -> PathBuf {
    if !file && (destination.is_dir() || destination.extension().is_none()) {
        destination.join(default_name)
    } else {
        destination.to_path_buf()
    }
}

fn export(destination: PathBuf, file: bool, default_name: &str, content: String) -> i32 {
    let path = export_path(&destination, file, default_name);
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
        Ok(mut file) => match file.write_all(content.as_bytes()) {
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

pub fn run(args: DocsArgs) -> i32 {
    let specification = rendered_specification();
    match args.command {
        DocsCommand::Show => {
            print!("{specification}");
            0
        }
        DocsCommand::Export { destination, file } => {
            export(destination, file, DEFAULT_EXPORT_NAME, specification)
        }
        DocsCommand::ProjectPolicy(args) => match args.command {
            ProjectPolicyCommand::Show => {
                print!("{}", rendered_project_policy());
                0
            }
            ProjectPolicyCommand::Export { destination, file } => export(
                destination,
                file,
                PROJECT_POLICY_EXPORT_NAME,
                rendered_project_policy(),
            ),
        },
    }
}
