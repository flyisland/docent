use crate::model::adr::AdrFront;
use crate::model::rfc::RfcFront;
use crate::model::{ADR_DIR, Project, RFC_DIR, load_docs};

type Row = (String, String, String, Option<String>, String);

fn normalize_status(doc_type: &str, status: Option<&str>) -> Result<Option<String>, String> {
    let Some(s) = status else {
        return Ok(None);
    };
    let allowed: &[&str] = match doc_type {
        "rfc" => &["Draft", "Accepted", "Rejected"],
        _ => &["Accepted", "Superseded", "Deprecated"],
    };
    if allowed.contains(&s) {
        Ok(Some(s.to_string()))
    } else {
        Err(format!(
            "invalid status '{}' for {}; expected one of: {}",
            s,
            doc_type,
            allowed.join(", ")
        ))
    }
}

fn print_rows(rows: &[Row], show_impl: bool) {
    if rows.is_empty() {
        return;
    }
    let id_w = rows.iter().map(|r| r.0.len()).max().unwrap();
    let title_w = rows.iter().map(|r| r.1.len()).max().unwrap();
    let status_w = rows.iter().map(|r| r.2.len()).max().unwrap();
    let impl_w = rows
        .iter()
        .filter_map(|r| r.3.as_ref().map(|i| i.len()))
        .max()
        .unwrap_or(0);
    let date_w = rows.iter().map(|r| r.4.len()).max().unwrap();
    for r in rows {
        print!("  {:<id_w$}  {:<title_w$}  {:<status_w$}", r.0, r.1, r.2);
        if show_impl {
            print!("  {:<impl_w$}", r.3.as_deref().unwrap_or(""));
        }
        println!("  {:<date_w$}", r.4);
    }
}

pub fn run(doc_type: String, status: Option<String>, implementation: Option<String>) -> i32 {
    let cwd = std::env::current_dir().unwrap_or_default();
    let project = Project::new(cwd);

    let status = match normalize_status(&doc_type, status.as_deref()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    if implementation.is_some() && doc_type != "adr" {
        eprintln!("--implementation is only valid when listing ADRs");
        return 1;
    }

    let mut rows: Vec<Row> = Vec::new();
    if doc_type == "rfc" {
        for d in load_docs::<RfcFront>(&project.root, RFC_DIR, &["README.md"]) {
            let Some(f) = &d.front else {
                continue;
            };
            if let Some(want) = &status
                && f.status.display() != want
            {
                continue;
            }
            rows.push((
                f.id.clone(),
                f.title.clone(),
                f.status.display().to_string(),
                None,
                f.index_date(),
            ));
        }
    } else {
        for d in load_docs::<AdrFront>(&project.root, ADR_DIR, &["README.md"]) {
            let Some(f) = &d.front else {
                continue;
            };
            if let Some(want) = &status
                && f.status.display() != want
            {
                continue;
            }
            if let Some(want) = &implementation
                && f.implementation.display().to_ascii_lowercase() != *want
            {
                continue;
            }
            rows.push((
                f.id.clone(),
                f.title.clone(),
                f.status.display().to_string(),
                Some(f.implementation.display().to_string()),
                f.index_date(),
            ));
        }
    }

    let kind_label = if doc_type == "rfc" { "RFC" } else { "ADR" };
    let mut header = kind_label.to_string();
    if let Some(s) = &status {
        header = format!("{} {}", s, header);
    }
    header = format!("{}s", header);
    if doc_type == "adr"
        && let Some(i) = &implementation
    {
        header = format!("{} with implementation {}", header, i);
    }
    header = format!("{} ({})", header, rows.len());
    println!("{}", header);
    print_rows(&rows, doc_type == "adr");

    0
}
