use crate::model::adr::{AdrFront, AdrStatus, Implementation};
use crate::model::rfc::{RfcFront, RfcStatus};
use crate::model::{ADR_DIR, Project, RFC_DIR, load_docs};
use crate::rules;

fn line(label: &str, value: &str) -> String {
    format!("{:<24}{}", label, value)
}

fn not_found(path: &std::path::Path) -> bool {
    !path.exists()
}

fn count_idea_entries(path: &std::path::Path) -> usize {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return 0,
    };
    content
        .lines()
        .filter(|l| {
            let t = l.trim();
            t.starts_with("- ") || t.starts_with("* ")
        })
        .count()
}

pub fn run() -> i32 {
    let cwd = std::env::current_dir().unwrap_or_default();
    let project = Project::new(cwd);

    let ideas_missing = not_found(&project.ideas());
    let ideas = count_idea_entries(&project.ideas());

    let rfc_dir = project.path(RFC_DIR);
    let rfc_missing = not_found(&rfc_dir);
    let rfc_docs = load_docs::<RfcFront>(&project.root, RFC_DIR, &["README.md"]);
    let mut rfc_draft = 0usize;
    let mut rfc_accepted = 0usize;
    let mut rfc_rejected = 0usize;
    for d in &rfc_docs {
        if let Some(f) = &d.front {
            match f.status {
                RfcStatus::Draft => rfc_draft += 1,
                RfcStatus::Accepted => rfc_accepted += 1,
                RfcStatus::Rejected => rfc_rejected += 1,
            }
        }
    }

    let adr_dir = project.path(ADR_DIR);
    let adr_missing = not_found(&adr_dir);
    let adr_docs = load_docs::<AdrFront>(&project.root, ADR_DIR, &["README.md"]);
    let mut adr_accepted = 0usize;
    let mut adr_superseded = 0usize;
    let mut adr_deprecated = 0usize;
    let mut adr_pending = 0usize;
    for d in &adr_docs {
        if let Some(f) = &d.front {
            match f.status {
                AdrStatus::Accepted => adr_accepted += 1,
                AdrStatus::Superseded => adr_superseded += 1,
                AdrStatus::Deprecated => adr_deprecated += 1,
            }
            if f.status == AdrStatus::Accepted && f.implementation == Implementation::Pending {
                adr_pending += 1;
            }
        }
    }

    let mut adr_breakdown = Vec::new();
    if adr_accepted > 0 {
        adr_breakdown.push(format!("{} Accepted", adr_accepted));
    }
    if adr_superseded > 0 {
        adr_breakdown.push(format!("{} Superseded", adr_superseded));
    }
    if adr_deprecated > 0 {
        adr_breakdown.push(format!("{} Deprecated", adr_deprecated));
    }

    let arch_missing = not_found(&project.architecture());
    let context_missing = not_found(&project.context());

    let violations = rules::run_all(&project);
    let arch_count = violations
        .iter()
        .filter(|v| v.rule == "architecture-module-sync")
        .count();
    let term_count = violations
        .iter()
        .filter(|v| v.rule == "context-avoid-term-violation")
        .count();

    let ideas_value = if ideas_missing {
        "not found (IDEAS.md)".to_string()
    } else {
        format!("{} unclaimed entries", ideas)
    };
    let rfc_value = if rfc_missing {
        format!("not found ({})", RFC_DIR)
    } else {
        format!(
            "{} Draft · {} Accepted · {} Rejected",
            rfc_draft, rfc_accepted, rfc_rejected
        )
    };
    let adr_line = if adr_missing {
        format!("not found ({})", ADR_DIR)
    } else if adr_breakdown.is_empty() {
        "0 ADRs".to_string()
    } else {
        adr_breakdown.join(" · ")
    };
    let arch_value = if arch_missing {
        "not found (docs/architecture.md)".to_string()
    } else {
        format!(
            "{} module-table {} against the code directory",
            arch_count,
            if arch_count == 1 {
                "mismatch"
            } else {
                "mismatches"
            }
        )
    };
    let term_value = if context_missing {
        "not found (CONTEXT.md)".to_string()
    } else {
        format!(
            "{} Avoid-term {}",
            term_count,
            if term_count == 1 {
                "violation"
            } else {
                "violations"
            }
        )
    };

    println!("{}", line("IDEAS.md", &ideas_value));
    println!();
    println!("{}", line("RFC", &rfc_value));
    println!();
    println!("{}", line("ADR", &adr_line));
    if adr_pending > 0 {
        let verb = if adr_pending == 1 { "is" } else { "are" };
        println!(
            "{}",
            line(
                "",
                &format!(
                    "{} ADR{} {} Accepted but implementation: pending",
                    adr_pending,
                    if adr_pending == 1 { "" } else { "s" },
                    verb
                )
            )
        );
    }
    println!();
    println!("{}", line("Architecture overview", &arch_value));
    println!();
    println!("{}", line("Terminology (CONTEXT)", &term_value));

    let mut rule_counts: std::collections::BTreeMap<&str, usize> =
        std::collections::BTreeMap::new();
    for v in &violations {
        *rule_counts.entry(v.rule).or_default() += 1;
    }
    let mut rows: Vec<(&str, usize)> = rule_counts.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    let total: usize = rows.iter().map(|(_, c)| c).sum();

    if total > 0 {
        println!();
        println!(
            "{}",
            line("Violations by rule", &format!("{} total", total))
        );
        for (rule, count) in rows {
            println!("  {:<40}{}", rule, count);
        }
    }

    0
}
