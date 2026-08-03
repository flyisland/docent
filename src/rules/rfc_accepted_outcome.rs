use crate::model::rfc::RfcStatus;
use crate::model::{Project, RFC_DIR, Violation, load_docs};
use crate::rules::Rule;

pub struct RfcAcceptedOutcomeRule;

impl Rule for RfcAcceptedOutcomeRule {
    fn id(&self) -> &'static str {
        "rfc-accepted-outcome"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        load_docs::<crate::model::rfc::RfcFront>(&project.root, RFC_DIR, &["README.md"])
            .into_iter()
            .filter_map(|document| {
                let front = document.front?;
                let needs_outcome = front.status == RfcStatus::Accepted && front.related_adr.is_none();
                let has_outcome = document.body.lines().any(|line| {
                    line.trim_start()
                        .starts_with("ADR not required:")
                });
                (needs_outcome && !has_outcome).then(|| {
                    Violation::error(
                        self.id(),
                        document.rel,
                        None,
                        "Accepted RFC without related_adr must state 'ADR not required: <reason>' in its outcome",
                    )
                })
            })
            .collect()
    }
}
