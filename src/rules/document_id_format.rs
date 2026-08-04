use crate::model::{ADR_DIR, Project, RFC_DIR, Violation, id_from_rel, load_docs};
use crate::rules::Rule;
use std::collections::HashSet;

pub struct DocumentIdFormatRule;

impl Rule for DocumentIdFormatRule {
    fn id(&self) -> &'static str {
        "document-id-format"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut ids = Vec::new();
        collect_ids::<crate::model::rfc::RfcFront>(project, RFC_DIR, &mut ids);
        collect_ids::<crate::model::adr::AdrFront>(project, ADR_DIR, &mut ids);

        let widths: HashSet<usize> = ids
            .iter()
            .flat_map(|(_, document_ids)| document_ids.iter().map(|id| id_width(id)))
            .collect();
        if !widths.contains(&3) {
            return Vec::new();
        }

        ids.into_iter()
            .filter(|(_, document_ids)| document_ids.iter().any(|id| is_pre_1000_four_digit_id(id)))
            .map(|(file, _)| {
                Violation::error(
                    self.id(),
                    file,
                    None,
                    "document ID uses four digits below 1000 while this project also uses three-digit IDs",
                )
            })
            .collect()
    }
}

fn collect_ids<T>(project: &Project, directory: &str, ids: &mut Vec<(String, Vec<String>)>)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    for doc in load_docs::<T>(&project.root, directory, &["README.md"]) {
        let mut document_ids = Vec::new();
        if let Some(id) = id_from_rel(&doc.rel) {
            document_ids.push(id);
        }
        if let Some(id) = front_id(&doc)
            && !document_ids.contains(&id)
        {
            document_ids.push(id);
        }
        if !document_ids.is_empty() {
            ids.push((doc.rel, document_ids));
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

fn id_width(id: &str) -> usize {
    id.rsplit_once('-').map_or(0, |(_, number)| number.len())
}

fn is_pre_1000_four_digit_id(id: &str) -> bool {
    let Some((_, number)) = id.split_once('-') else {
        return false;
    };
    number.len() == 4 && number.parse::<u16>().is_ok_and(|value| value < 1000)
}
