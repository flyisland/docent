use crate::model::adr::{AdrFront, AdrStatus};
use crate::model::{ADR_DIR, Project, Violation, id_from_rel, load_docs};
use crate::rules::Rule;
use std::collections::{HashMap, HashSet};

pub struct SupersededBacklinkRule;

impl Rule for SupersededBacklinkRule {
    fn id(&self) -> &'static str {
        "superseded-backlink-consistency"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        let docs = load_docs::<AdrFront>(&project.root, ADR_DIR, &["README.md"]);
        let mut by_id: HashMap<String, &crate::model::Loaded<AdrFront>> = HashMap::new();
        for d in &docs {
            let id = d
                .front
                .as_ref()
                .map(|f| f.id.clone())
                .unwrap_or_else(|| id_from_rel(&d.rel).unwrap_or_default());
            by_id.entry(id).or_insert(d);
        }

        let mut referenced: HashSet<String> = HashSet::new();
        for d in &docs {
            let Some(f) = &d.front else {
                continue;
            };
            for b in &f.supersedes {
                referenced.insert(b.clone());
                match by_id.get(b) {
                    None => v.push(Violation::error(
                        self.id(),
                        d.rel.clone(),
                        None,
                        format!(
                            "{} lists {} in supersedes, but no such ADR file exists",
                            f.id, b
                        ),
                    )),
                    Some(bdoc) => {
                        let Some(bfront) = &bdoc.front else {
                            continue;
                        };
                        match &bfront.superseded_by {
                            Some(sb) if sb != &f.id => v.push(Violation::error(
                                self.id(),
                                bdoc.rel.clone(),
                                None,
                                format!(
                                    "{} lists {} in supersedes, but {} records superseded_by: {}",
                                    f.id, b, b, sb
                                ),
                            )),
                            None => v.push(Violation::error(
                                self.id(),
                                bdoc.rel.clone(),
                                None,
                                format!(
                                    "{} lists {} in supersedes, but {} records no superseded_by backlink to {}",
                                    f.id, b, b, f.id
                                ),
                            )),
                            Some(_) => {
                                if bfront.status != AdrStatus::Superseded {
                                    v.push(Violation::error(
                                        self.id(),
                                        bdoc.rel.clone(),
                                        None,
                                        format!(
                                            "{} records superseded_by: {}, but its status is not Superseded",
                                            b, f.id
                                        ),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        for d in &docs {
            let Some(f) = &d.front else {
                continue;
            };
            if f.status == AdrStatus::Superseded
                && f.superseded_by.is_none()
                && !referenced.contains(&f.id)
            {
                v.push(Violation::error(
                    self.id(),
                    d.rel.clone(),
                    None,
                    format!(
                        "{} has status Superseded but records no superseded_by",
                        f.id
                    ),
                ));
            }
        }
        v
    }
}
