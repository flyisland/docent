use crate::model::index::parse_index_table;
use crate::model::{ADR_DIR, Project, Violation, id_from_rel, load_docs, relative_str};
use crate::rules::Rule;
use std::collections::{HashMap, HashSet};

pub struct AdrIndexSyncRule;

impl Rule for AdrIndexSyncRule {
    fn id(&self) -> &'static str {
        "adr-index-sync"
    }

    fn run(&self, project: &Project) -> Vec<Violation> {
        let mut v = Vec::new();
        let docs = load_docs::<crate::model::adr::AdrFront>(&project.root, ADR_DIR, &["README.md"]);
        let mut by_id: HashMap<String, &crate::model::Loaded<crate::model::adr::AdrFront>> =
            HashMap::new();
        for d in &docs {
            let id = d
                .front
                .as_ref()
                .map(|f| f.id.clone())
                .unwrap_or_else(|| id_from_rel(&d.rel).unwrap_or_default());
            by_id.entry(id).or_insert(d);
        }

        let index_path = project.adr_index();
        let content = match std::fs::read_to_string(&index_path) {
            Ok(c) => c,
            Err(_) => {
                for d in &docs {
                    if let Some(f) = &d.front {
                        v.push(Violation::error(
                            self.id(),
                            d.rel.clone(),
                            None,
                            format!("{} is missing from the index table", f.id),
                        ));
                    }
                }
                return v;
            }
        };
        let index_rel = relative_str(&index_path, &project.root);
        let table = match parse_index_table(&content) {
            Some(t) => t,
            None => return v,
        };
        let row_ids: HashSet<String> = table
            .rows
            .iter()
            .filter_map(|r| r.first())
            .cloned()
            .collect();

        for row in &table.rows {
            let Some(id) = row.first() else {
                continue;
            };
            let Some(doc) = by_id.get(id) else {
                v.push(Violation::error(
                    self.id(),
                    index_rel.clone(),
                    None,
                    format!("index lists {} but no matching ADR file exists", id),
                ));
                continue;
            };
            let Some(front) = &doc.front else {
                continue;
            };
            let title_col = table.col("Title");
            let status_col = table.col("Status");
            let last_updated_col = table.col("Last Updated");
            let mut want: Vec<(&str, Option<String>, Option<String>)> = Vec::new();
            if let Some(c) = title_col {
                want.push(("title", row.get(c).cloned(), Some(front.title.clone())));
            }
            if let Some(c) = status_col {
                want.push((
                    "status",
                    row.get(c).cloned(),
                    Some(front.status.display().to_string()),
                ));
            }
            if let Some(c) = last_updated_col {
                want.push((
                    "last updated",
                    row.get(c).cloned(),
                    Some(front.index_date()),
                ));
            }
            for (label, got, exp) in want {
                if let (Some(g), Some(e)) = (got, exp)
                    && g != e
                {
                    v.push(Violation::error(
                        self.id(),
                        index_rel.clone(),
                        None,
                        format!(
                            "index entry for {}: {} differs (index '{}', front matter '{}')",
                            id, label, g, e
                        ),
                    ));
                }
            }
        }

        for d in &docs {
            if let Some(f) = &d.front
                && !row_ids.contains(&f.id)
            {
                v.push(Violation::error(
                    self.id(),
                    d.rel.clone(),
                    None,
                    format!("{} is missing from the index table", f.id),
                ));
            }
        }
        v
    }
}
