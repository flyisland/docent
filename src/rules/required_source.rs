use crate::model::{Project, Violation};
use crate::rules::Rule;

pub struct RequiredSourceMissingRule;

const REQUIRED_SOURCES: [(&str, &str); 4] = [
    ("docs/rfcs", "directory"),
    ("docs/adrs", "directory"),
    ("docs/architecture.md", "file"),
    ("AGENTS.md", "file"),
];

impl Rule for RequiredSourceMissingRule {
    fn id(&self) -> &'static str {
        "required-source-missing"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        for (rel, kind) in REQUIRED_SOURCES {
            if !project.path(rel).exists() {
                v.push(Violation::warning(
                    self.id(),
                    rel.to_string(),
                    None,
                    format!("required source {} {} does not exist", kind, rel),
                ));
            }
        }
        v
    }
}
