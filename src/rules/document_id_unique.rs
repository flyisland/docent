use crate::model::{ADR_DIR, Project, RFC_DIR, Violation, id_from_rel, load_docs};
use crate::rules::Rule;
use std::collections::HashMap;

pub struct DocumentIdUniqueRule;

impl Rule for DocumentIdUniqueRule {
    fn id(&self) -> &'static str {
        "document-id-unique"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut violations = Vec::new();
        check_directory::<crate::model::rfc::RfcFront, _>(
            project,
            RFC_DIR,
            "RFC",
            |doc| doc.front.as_ref().map(|front| front.id.clone()),
            &mut violations,
        );
        check_directory::<crate::model::adr::AdrFront, _>(
            project,
            ADR_DIR,
            "ADR",
            |doc| doc.front.as_ref().map(|front| front.id.clone()),
            &mut violations,
        );
        violations
    }
}

fn check_directory<T, F>(
    project: &Project,
    directory: &str,
    kind: &str,
    front_id: F,
    violations: &mut Vec<Violation>,
) where
    T: serde::de::DeserializeOwned,
    F: Fn(&crate::model::Loaded<T>) -> Option<String>,
{
    let mut seen: HashMap<String, String> = HashMap::new();
    for doc in load_docs::<T>(&project.root, directory, &["README.md"]) {
        let mut ids = Vec::new();
        if let Some(id) = id_from_rel(&doc.rel) {
            ids.push(id);
        }
        if let Some(id) = front_id(&doc)
            && !ids.contains(&id)
        {
            ids.push(id);
        }

        for id in &ids {
            if let Some(previous) = seen.get(id) {
                violations.push(Violation::error(
                    "document-id-unique",
                    doc.rel.clone(),
                    None,
                    format!(
                        "{} ID {} is duplicated; already used by {}",
                        kind, id, previous
                    ),
                ));
                break;
            }
        }
        for id in ids {
            seen.entry(id).or_insert_with(|| doc.rel.clone());
        }
    }
}
