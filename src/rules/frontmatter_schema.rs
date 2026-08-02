use crate::model::{ADR_DIR, Project, RFC_DIR, Violation, load_docs};
use crate::rules::Rule;

pub struct FrontmatterSchemaRule;

impl Rule for FrontmatterSchemaRule {
    fn id(&self) -> &'static str {
        "frontmatter-schema-valid"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        for doc in load_docs::<crate::model::rfc::RfcFront>(&project.root, RFC_DIR, &["README.md"])
        {
            match &doc.front {
                Some(f) => {
                    for e in f.validation_errors() {
                        v.push(Violation::error(self.id(), doc.rel.clone(), None, e));
                    }
                }
                None => {
                    let why = doc
                        .parse_error
                        .clone()
                        .unwrap_or_else(|| "unable to parse YAML".to_string());
                    v.push(Violation::error(
                        self.id(),
                        doc.rel.clone(),
                        None,
                        format!("RFC front matter is invalid: {}", why),
                    ));
                }
            }
        }
        for doc in load_docs::<crate::model::adr::AdrFront>(&project.root, ADR_DIR, &["README.md"])
        {
            match &doc.front {
                Some(f) => {
                    for e in f.validation_errors() {
                        v.push(Violation::error(self.id(), doc.rel.clone(), None, e));
                    }
                }
                None => {
                    let why = doc
                        .parse_error
                        .clone()
                        .unwrap_or_else(|| "unable to parse YAML".to_string());
                    v.push(Violation::error(
                        self.id(),
                        doc.rel.clone(),
                        None,
                        format!("ADR front matter is invalid: {}", why),
                    ));
                }
            }
        }
        v
    }
}
