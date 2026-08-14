use crate::commands::docs::{POLICY_VERSION, managed_project_policy_version};
use crate::model::{Project, Violation};
use crate::rules::Rule;

pub struct ProjectPolicyVersionRule;

impl Rule for ProjectPolicyVersionRule {
    fn id(&self) -> &'static str {
        "project-policy-version"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let rel = "docs/README.md";
        let Ok(content) = std::fs::read_to_string(project.path(rel)) else {
            return Vec::new();
        };
        let Some(version) = managed_project_policy_version(&content) else {
            return Vec::new();
        };
        let line = content
            .lines()
            .position(|line| line.trim_start().starts_with("policy_version:"))
            .map(|index| index + 1);

        if version < POLICY_VERSION {
            vec![Violation::warning(
                self.id(),
                rel.to_string(),
                line,
                format!(
                    "managed project policy is v{}; installed Docent provides v{} (review `docent docs project-policy show`)",
                    version, POLICY_VERSION
                ),
            )]
        } else if version > POLICY_VERSION {
            vec![Violation::warning(
                self.id(),
                rel.to_string(),
                line,
                format!(
                    "managed project policy is v{}, newer than installed Docent policy v{}",
                    version, POLICY_VERSION
                ),
            )]
        } else {
            Vec::new()
        }
    }
}
