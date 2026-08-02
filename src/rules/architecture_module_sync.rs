use crate::model::index::parse_index_table;
use crate::model::{Project, Violation, relative_str};
use crate::rules::Rule;
use std::path::{Component, Path};

pub struct ArchitectureModuleSyncRule;

fn is_relative_project_path(path: &Path) -> bool {
    !path.is_absolute()
        && path
            .components()
            .any(|component| matches!(component, Component::Normal(_)))
        && !path
            .components()
            .any(|component| component == Component::ParentDir)
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
        let Some(path_col) = table.col("Path") else {
            v.push(Violation::warning(
                self.id(),
                arch_rel,
                None,
                "The module table must include a Path column".to_string(),
            ));
            return v;
        };
        let canonical_root = project
            .root
            .canonicalize()
            .unwrap_or_else(|_| project.root.clone());

        for row in &table.rows {
            let module = row
                .get(mod_col)
                .map(|value| value.trim())
                .unwrap_or_default();
            if module.is_empty() {
                v.push(Violation::warning(
                    self.id(),
                    arch_rel.clone(),
                    None,
                    "A module table entry must declare a non-empty Module".to_string(),
                ));
                continue;
            }
            let path = row
                .get(path_col)
                .map(|value| value.trim())
                .unwrap_or_default();
            if path.is_empty() || path == "—" {
                v.push(Violation::warning(
                    self.id(),
                    arch_rel.clone(),
                    None,
                    format!("Module '{}' must declare a non-empty relative Path", module),
                ));
                continue;
            }
            let path = Path::new(path);
            if !is_relative_project_path(path) {
                v.push(Violation::warning(
                    self.id(),
                    arch_rel.clone(),
                    None,
                    format!(
                        "Module '{}' declares Path '{}', which must identify a location below the project root and must not contain '..'",
                        module,
                        path.display()
                    ),
                ));
                continue;
            }
            if !project.root.join(path).exists() {
                v.push(Violation::warning(
                    self.id(),
                    arch_rel.clone(),
                    None,
                    format!(
                        "Module '{}' declares Path '{}', but that path does not exist",
                        module,
                        path.display()
                    ),
                ));
                continue;
            }
            if let Ok(canonical_path) = project.root.join(path).canonicalize()
                && !canonical_path.starts_with(&canonical_root)
            {
                v.push(Violation::warning(
                    self.id(),
                    arch_rel.clone(),
                    None,
                    format!(
                        "Module '{}' declares Path '{}', but it resolves outside the project root",
                        module,
                        path.display()
                    ),
                ));
            }
        }
        v
    }
}
