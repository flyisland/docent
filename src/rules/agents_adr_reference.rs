use crate::model::adr::{AdrFront, AdrStatus};
use crate::model::{ADR_DIR, Project, Violation, load_docs, relative_str};
use crate::rules::Rule;
use regex::Regex;
use std::collections::HashMap;

pub struct AgentsAdrReferenceRule;

impl Rule for AgentsAdrReferenceRule {
    fn id(&self) -> &'static str {
        "agents-adr-reference-valid"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        let agents_path = project.agents();
        let content = match std::fs::read_to_string(&agents_path) {
            Ok(c) => c,
            Err(_) => return v,
        };
        let agents_rel = relative_str(&agents_path, &project.root);

        let mut status_by_id: HashMap<String, AdrStatus> = HashMap::new();
        for d in load_docs::<AdrFront>(&project.root, ADR_DIR, &["README.md"]) {
            if let Some(f) = &d.front {
                status_by_id.insert(f.id.clone(), f.status);
            }
        }

        let re = Regex::new(r"(?i)\[See\s+(adr)-(\d{3})\]").unwrap();
        for (idx, line) in content.lines().enumerate() {
            for caps in re.captures_iter(line) {
                let id = format!("{}-{}", caps[1].to_lowercase(), &caps[2]);
                match status_by_id.get(&id) {
                    None => v.push(Violation::error(
                        self.id(),
                        agents_rel.clone(),
                        Some(idx + 1),
                        format!("References {}, but no such ADR file exists", id),
                    )),
                    Some(status) if *status != AdrStatus::Accepted => v.push(Violation::error(
                        self.id(),
                        agents_rel.clone(),
                        Some(idx + 1),
                        format!(
                            "References {}, but that ADR's status is {}",
                            id,
                            status.display()
                        ),
                    )),
                    _ => {}
                }
            }
        }
        v
    }
}
