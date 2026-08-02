use crate::model::index::parse_index_table;
use crate::model::{Project, Violation, relative_str};
use crate::rules::Rule;

pub struct ArchitectureModuleSyncRule;

const EXCLUDED_DIRS: [&str; 6] = [".git", "target", "docs", "tests", "node_modules", "src"];

fn root_module_dirs(root: &std::path::Path) -> Vec<String> {
    let entries = match std::fs::read_dir(root) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let mut dirs = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if name.starts_with('.') || EXCLUDED_DIRS.contains(&name) {
            continue;
        }
        dirs.push(name.to_string());
    }
    dirs.sort();
    dirs
}

impl Rule for ArchitectureModuleSyncRule {
    fn id(&self) -> &'static str {
        "architecture-module-sync"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        let arch_path = project.architecture();
        let content = match std::fs::read_to_string(&arch_path) {
            Ok(c) => c,
            Err(_) => return v,
        };
        let arch_rel = relative_str(&arch_path, &project.root);
        let table = match parse_index_table(&content) {
            Some(t) => t,
            None => return v,
        };
        let Some(mod_col) = table.col("Module") else {
            return v;
        };

        let recorded: Vec<String> = table
            .rows
            .iter()
            .filter_map(|r| r.get(mod_col))
            .map(|s| s.trim().trim_end_matches('/').to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let dirs = root_module_dirs(&project.root);

        for m in &recorded {
            if !dirs.contains(m) {
                v.push(Violation::warning(
                    self.id(),
                    arch_rel.clone(),
                    None,
                    format!(
                        "Module '{}' is recorded in the architecture overview, but no directory exists at the project root",
                        m
                    ),
                ));
            }
        }
        for d in &dirs {
            if !recorded.contains(d) {
                v.push(Violation::warning(
                    self.id(),
                    arch_rel.clone(),
                    None,
                    format!(
                        "Directory '{}' exists at the project root, but is not listed in the architecture overview",
                        d
                    ),
                ));
            }
        }
        v
    }
}
