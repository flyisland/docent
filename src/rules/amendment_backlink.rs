use crate::model::adr::{AdrFront, AdrStatus};
use crate::model::{ADR_DIR, Project, Violation, id_from_rel, load_docs};
use crate::rules::Rule;
use std::collections::{HashMap, HashSet};

pub struct AmendmentBacklinkRule;

impl Rule for AmendmentBacklinkRule {
    fn id(&self) -> &'static str {
        "amendment-backlink-consistency"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let docs = load_docs::<AdrFront>(&project.root, ADR_DIR, &["README.md"]);
        let by_id: HashMap<String, _> = docs
            .iter()
            .map(|document| {
                let id = document
                    .front
                    .as_ref()
                    .map(|front| front.id.clone())
                    .unwrap_or_else(|| id_from_rel(&document.rel).unwrap_or_default());
                (id, document)
            })
            .collect();
        let mut declared_backlinks = HashSet::new();
        let mut accepted_sources_by_scope: HashMap<(String, String), Vec<String>> = HashMap::new();
        let mut violations = Vec::new();

        for document in &docs {
            let Some(front) = &document.front else {
                continue;
            };
            for amendment in &front.amends {
                if front.status == AdrStatus::Accepted {
                    accepted_sources_by_scope
                        .entry((amendment.adr.clone(), amendment.decision.clone()))
                        .or_default()
                        .push(front.id.clone());
                }
                declared_backlinks.insert((amendment.adr.clone(), front.id.clone()));
                let Some(target) = by_id.get(&amendment.adr) else {
                    violations.push(Violation::error(
                        self.id(),
                        document.rel.clone(),
                        None,
                        format!(
                            "{} amends {} decision '{}', but no such ADR exists",
                            front.id, amendment.adr, amendment.decision
                        ),
                    ));
                    continue;
                };
                let Some(target_front) = &target.front else {
                    continue;
                };
                if target_front.status != AdrStatus::Accepted {
                    violations.push(Violation::error(
                        self.id(),
                        document.rel.clone(),
                        None,
                        format!(
                            "{} cannot partially amend {} because its status is {} rather than Accepted",
                            front.id,
                            amendment.adr,
                            target_front.status.display()
                        ),
                    ));
                }
                if !target_front.amended_by.contains(&front.id) {
                    violations.push(Violation::error(
                        self.id(),
                        target.rel.clone(),
                        None,
                        format!(
                            "{} is amended by {}, but amended_by lacks the backlink",
                            amendment.adr, front.id
                        ),
                    ));
                }
            }
        }

        for document in &docs {
            let Some(front) = &document.front else {
                continue;
            };
            for source in &front.amended_by {
                if !declared_backlinks.contains(&(front.id.clone(), source.clone())) {
                    violations.push(Violation::error(
                        self.id(),
                        document.rel.clone(),
                        None,
                        format!(
                            "{} lists amended_by: {}, but {} has no matching amends entry",
                            front.id, source, source
                        ),
                    ));
                }
            }
        }

        for ((target, decision), sources) in accepted_sources_by_scope {
            if sources.len() > 1 {
                violations.push(Violation::error(
                    self.id(),
                    by_id
                        .get(&target)
                        .map(|document| document.rel.clone())
                        .unwrap_or_else(|| format!("docs/adrs/{target}")),
                    None,
                    format!(
                        "{} decision '{}' has multiple Accepted amendments: {}",
                        target,
                        decision,
                        sources.join(", ")
                    ),
                ));
            }
        }

        violations
    }
}
