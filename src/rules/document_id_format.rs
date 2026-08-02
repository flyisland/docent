use crate::model::{ADR_DIR, Project, RFC_DIR, Violation, id_from_rel, id_width, load_docs};
use crate::rules::Rule;
use std::collections::{HashMap, HashSet};

pub struct DocumentIdFormatRule;

impl Rule for DocumentIdFormatRule {
    fn id(&self) -> &'static str {
        "document-id-format"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut widths = HashSet::new();
        let mut document_widths: HashMap<String, HashSet<usize>> = HashMap::new();
        collect_widths::<crate::model::rfc::RfcFront>(
            project,
            RFC_DIR,
            &mut widths,
            &mut document_widths,
        );
        collect_widths::<crate::model::adr::AdrFront>(
            project,
            ADR_DIR,
            &mut widths,
            &mut document_widths,
        );

        // An empty project defaults to the three-digit convention. Once a
        // project has documents, its existing convention determines whether
        // three or four digits are in use; mixing them is always an error.
        if widths.len() <= 1 {
            return Vec::new();
        }

        let expected = if widths.contains(&3) { 3 } else { 4 };
        let mut violations = Vec::new();
        for (file, file_widths) in document_widths {
            if file_widths.contains(&other_width(expected)) {
                violations.push(Violation::error(
                    self.id(),
                    file,
                    None,
                    format!(
                        "document ID formats are mixed; this project uses both three-digit and four-digit numbers (expected {} digits)",
                        expected
                    ),
                ));
            }
        }
        violations
    }
}

fn other_width(width: usize) -> usize {
    if width == 3 { 4 } else { 3 }
}

fn collect_widths<T>(
    project: &Project,
    directory: &str,
    widths: &mut HashSet<usize>,
    document_widths: &mut HashMap<String, HashSet<usize>>,
) where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    for doc in load_docs::<T>(&project.root, directory, &["README.md"]) {
        let entry = document_widths.entry(doc.rel.clone()).or_default();
        if let Some(id) = id_from_rel(&doc.rel).and_then(|id| id_width(&id)) {
            widths.insert(id);
            entry.insert(id);
        }
        if let Some(id) = front_id(&doc).and_then(|id| id_width(&id)) {
            widths.insert(id);
            entry.insert(id);
        }
    }
}

fn front_id<T>(doc: &crate::model::Loaded<T>) -> Option<String>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let front = doc.front.as_ref()?;
    serde_json::to_value(front)
        .ok()?
        .get("id")?
        .as_str()
        .map(str::to_string)
}
