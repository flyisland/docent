use crate::model::{ADR_DIR, Project, Violation, load_docs};
use crate::rules::Rule;

pub struct AdrMissingSectionsRule;

const REQUIRED: [(&str, &str); 4] = [
    ("context", "Context"),
    ("decision", "Decision"),
    ("non-goals", "Non-goals"),
    ("consequences", "Consequences"),
];

fn section_present(body: &str, keyword: &str) -> bool {
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(text) = trimmed.strip_prefix("## ") {
            let lower = text.trim().to_lowercase();
            match keyword {
                "context" if lower.contains("context") || lower.contains("背景") => return true,
                "decision" if lower.contains("decision") || lower.contains("决策") => {
                    return true;
                }
                "non-goals"
                    if lower.contains("non-goals")
                        || lower.contains("nongoals")
                        || lower.contains("非目标") =>
                {
                    return true;
                }
                "consequences" if lower.contains("consequences") || lower.contains("后果") => {
                    return true;
                }
                _ => {}
            }
        }
    }
    false
}

impl Rule for AdrMissingSectionsRule {
    fn id(&self) -> &'static str {
        "adr-missing-required-sections"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        for doc in load_docs::<crate::model::adr::AdrFront>(&project.root, ADR_DIR, &["README.md"])
        {
            for (keyword, label) in REQUIRED {
                if !section_present(&doc.body, keyword) {
                    v.push(Violation::error(
                        self.id(),
                        doc.rel.clone(),
                        None,
                        format!("Missing {} section", label),
                    ));
                }
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::section_present;

    #[test]
    fn detects_all_four_sections() {
        let body = "## Context\n...\n## Decision\n...\n## Non-goals\n...\n## Consequences\n...\n";
        assert!(section_present(body, "context"));
        assert!(section_present(body, "decision"));
        assert!(section_present(body, "non-goals"));
        assert!(section_present(body, "consequences"));
    }

    #[test]
    fn accepts_trailing_text_and_alternate_forms() {
        assert!(section_present("## 背景\n", "context"));
        assert!(section_present("## Non-Goals (v1)\n", "non-goals"));
        assert!(section_present(
            "## Consequences and impact\n",
            "consequences"
        ));
        assert!(!section_present("## Overview\n", "context"));
    }

    #[test]
    fn third_level_heading_does_not_count() {
        assert!(!section_present("### Context\n", "context"));
    }
}
