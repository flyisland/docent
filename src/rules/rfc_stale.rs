use crate::model::rfc::{RfcFront, RfcStatus};
use crate::model::{Project, RFC_DIR, Violation, date_to_days, load_docs, today_days};
use crate::rules::Rule;

const STALE_DAYS: i64 = 60;

pub struct RfcStaleDraftRule;

impl Rule for RfcStaleDraftRule {
    fn id(&self) -> &'static str {
        "rfc-stale-draft"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        let today = today_days();
        for doc in load_docs::<RfcFront>(&project.root, RFC_DIR, &["README.md"]) {
            if let Some(f) = &doc.front
                && f.status == RfcStatus::Draft
            {
                let date = f.updated.as_deref().unwrap_or(&f.created);
                if let Some(days) = date_to_days(date)
                    && today - days > STALE_DAYS
                {
                    v.push(Violation::warning(
                        self.id(),
                        doc.rel.clone(),
                        None,
                        format!(
                            "Status has been Draft for over {} days without an update",
                            STALE_DAYS
                        ),
                    ));
                }
            }
        }
        v
    }
}
