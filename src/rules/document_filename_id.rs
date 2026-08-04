use crate::model::{ADR_DIR, Project, RFC_DIR, Violation, id_from_rel, load_docs};
use crate::rules::Rule;

pub struct DocumentFilenameIdRule;

impl Rule for DocumentFilenameIdRule {
    fn id(&self) -> &'static str {
        "document-filename-id"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut violations = Vec::new();
        check_directory::<crate::model::rfc::RfcFront, _>(
            project,
            RFC_DIR,
            "rfc",
            |front| &front.id,
            &mut violations,
        );
        check_directory::<crate::model::adr::AdrFront, _>(
            project,
            ADR_DIR,
            "adr",
            |front| &front.id,
            &mut violations,
        );
        violations
    }
}

fn check_directory<T, F>(
    project: &Project,
    directory: &str,
    prefix: &str,
    front_id: F,
    violations: &mut Vec<Violation>,
) where
    T: serde::de::DeserializeOwned,
    F: Fn(&T) -> &str,
{
    for doc in load_docs::<T>(&project.root, directory, &["README.md"]) {
        let Some(filename_id) = id_from_rel(&doc.rel) else {
            violations.push(Violation::error(
                "document-filename-id",
                doc.rel.clone(),
                None,
                format!("filename must use the {}-NNN(-slug).md form", prefix),
            ));
            continue;
        };

        if !filename_id.starts_with(&format!("{}-", prefix)) {
            violations.push(Violation::error(
                "document-filename-id",
                doc.rel.clone(),
                None,
                format!("filename must use the {}-NNN(-slug).md form", prefix),
            ));
            continue;
        }

        if let Some(front) = &doc.front {
            let front_id = front_id(front);
            if filename_id != front_id {
                violations.push(Violation::error(
                    "document-filename-id",
                    doc.rel.clone(),
                    None,
                    format!(
                        "filename ID {} does not match front matter id {}",
                        filename_id, front_id
                    ),
                ));
            }
        }
    }
}
