use crate::model::adr::{AdrFront, AdrStatus, Implementation};
use crate::model::{ADR_DIR, Project, Violation, load_docs};
use crate::rules::Rule;

pub struct AdrPendingRule;

impl Rule for AdrPendingRule {
    fn id(&self) -> &'static str {
        "adr-pending-implementation-report"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        for doc in load_docs::<AdrFront>(&project.root, ADR_DIR, &["README.md"]) {
            if let Some(f) = &doc.front
                && f.status == AdrStatus::Accepted
                && f.implementation == Implementation::Pending
            {
                v.push(Violation::warning(
                    self.id(),
                    doc.rel.clone(),
                    None,
                    format!("{} is Accepted but implementation: pending", f.id),
                ));
            }
        }
        v
    }
}
