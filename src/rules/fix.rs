use crate::model::adr::{AdrFront, AdrStatus};
use crate::model::index::{render_adr_index, render_rfc_index};
use crate::model::rfc::RfcFront;
use crate::model::{ADR_DIR, Project, RFC_DIR, id_from_rel, load_docs, split_front_matter};
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

fn write_if_different(project: &Project, rel: &str, content: &str) -> Option<String> {
    let path = project.root.join(rel);
    let existing = std::fs::read_to_string(&path).ok();
    if existing.as_deref() == Some(content) {
        return None;
    }
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::write(&path, content).is_ok() {
        Some(rel.to_string())
    } else {
        None
    }
}

fn regenerate_indexes(project: &Project) -> Vec<String> {
    let mut changed = Vec::new();

    let adr_docs = load_docs::<AdrFront>(&project.root, ADR_DIR, &["README.md"]);
    let mut adr_rows: Vec<(String, String, String, String, String)> = Vec::new();
    for d in &adr_docs {
        if let Some(f) = &d.front {
            adr_rows.push((
                f.id.clone(),
                f.title.clone(),
                f.status.display().to_string(),
                f.implementation.display().to_string(),
                f.index_date(),
            ));
        }
    }
    adr_rows.sort_by(|a, b| a.0.cmp(&b.0));
    if let Some(rel) =
        write_if_different(project, "docs/adrs/README.md", &render_adr_index(&adr_rows))
    {
        changed.push(rel);
    }

    let rfc_docs = load_docs::<RfcFront>(&project.root, RFC_DIR, &["README.md"]);
    let mut rfc_rows: Vec<(String, String, String, String, String)> = Vec::new();
    for d in &rfc_docs {
        if let Some(f) = &d.front {
            let linked = f.related_adr.clone().unwrap_or_else(|| "—".to_string());
            rfc_rows.push((
                f.id.clone(),
                f.title.clone(),
                f.status.display().to_string(),
                f.index_date(),
                linked,
            ));
        }
    }
    rfc_rows.sort_by(|a, b| a.0.cmp(&b.0));
    if let Some(rel) =
        write_if_different(project, "docs/rfcs/README.md", &render_rfc_index(&rfc_rows))
    {
        changed.push(rel);
    }

    changed
}

fn set_field(yaml: &str, key: &str, value: &str) -> String {
    let re = Regex::new(&format!(r"(?m)^\s*{}\s*:.*$", regex::escape(key))).unwrap();
    if re.is_match(yaml) {
        re.replace(yaml, format!("{}: {}", key, value)).into_owned()
    } else {
        format!("{}\n{}: {}", yaml.trim_end(), key, value)
    }
}

fn add_to_supersedes(yaml: &str, item: &str) -> String {
    let key = "supersedes";
    let re = Regex::new(&format!(
        r"(?m)^\s*{}\s*:\s*\[\s*(.*?)\s*\]\s*$",
        regex::escape(key)
    ))
    .unwrap();
    if let Some(caps) = re.captures(yaml) {
        let inner = caps[1].trim();
        let new_inner = if inner.is_empty() {
            item.to_string()
        } else {
            format!("{}, {}", inner, item)
        };
        re.replace(yaml, format!("{}: [{}]", key, new_inner))
            .into_owned()
    } else {
        format!("{}\n{}: [{}]", yaml.trim_end(), key, item)
    }
}

fn replace_front_matter(content: &str, f: impl FnOnce(&str) -> String) -> Option<String> {
    let start = content.find("---")?;
    let rest = &content[start + 3..];
    let close = rest.find("\n---")?;
    let yaml_start = start + 3;
    let yaml_end = start + 3 + close;
    let yaml = &content[yaml_start..yaml_end];
    let new_yaml = f(yaml);
    if new_yaml == yaml {
        return None;
    }
    Some(format!(
        "{}{}{}",
        &content[..yaml_start],
        new_yaml,
        &content[yaml_end..]
    ))
}

fn parse_adr(content: &str) -> Option<AdrFront> {
    match split_front_matter(content) {
        crate::model::FrontSplit::Ok { yaml, .. } => serde_yaml::from_str(&yaml).ok(),
        _ => None,
    }
}

fn rewrite(path: &Path, transform: impl FnOnce(&str) -> String) -> bool {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return false,
    };
    match replace_front_matter(&content, transform) {
        Some(next) => std::fs::write(path, next).is_ok(),
        None => false,
    }
}

fn fix_backlinks(project: &Project) -> Vec<String> {
    let mut changed = Vec::new();
    let docs = load_docs::<AdrFront>(&project.root, ADR_DIR, &["README.md"]);
    let mut rel_by_id: HashMap<String, String> = HashMap::new();
    for d in &docs {
        let id = d
            .front
            .as_ref()
            .map(|f| f.id.clone())
            .unwrap_or_else(|| id_from_rel(&d.rel).unwrap_or_default());
        rel_by_id.entry(id).or_insert_with(|| d.rel.clone());
    }

    for d in &docs {
        let Some(f) = &d.front else {
            continue;
        };
        for b in &f.supersedes {
            let Some(brel) = rel_by_id.get(b) else {
                continue;
            };
            let bpath = project.root.join(brel);
            let bcontent = match std::fs::read_to_string(&bpath) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let Some(bfront) = parse_adr(&bcontent) else {
                continue;
            };
            if bfront.superseded_by.is_none() {
                if rewrite(&bpath, |y| set_field(y, "superseded_by", &f.id)) {
                    changed.push(brel.clone());
                }
                if bfront.status != AdrStatus::Superseded
                    && rewrite(&bpath, |y| set_field(y, "status", "Superseded"))
                    && !changed.contains(brel)
                {
                    changed.push(brel.clone());
                }
            }
        }
        if let Some(sb) = &f.superseded_by {
            let Some(arel) = rel_by_id.get(sb) else {
                continue;
            };
            let apath = project.root.join(arel);
            let acontent = match std::fs::read_to_string(&apath) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let Some(afront) = parse_adr(&acontent) else {
                continue;
            };
            if !afront.supersedes.contains(&f.id)
                && rewrite(&apath, |y| add_to_supersedes(y, &f.id))
            {
                changed.push(arel.clone());
            }
        }
    }
    changed
}

pub fn apply_fixes(project: &Project) -> Vec<String> {
    let mut changed = Vec::new();
    changed.extend(regenerate_indexes(project));
    changed.extend(fix_backlinks(project));
    changed
}
