use crate::model::adr::{AdrFront, AdrStatus, Implementation};
use crate::model::rfc::{RfcFront, RfcStatus};
use crate::model::{ADR_DIR, Project, RFC_DIR, load_docs};
use crate::rules::Rule;
use crate::rules::architecture_module_sync::ArchitectureModuleSyncRule;
use crate::rules::context_avoid_term::ContextAvoidTermRule;

fn line(label: &str, value: &str) -> String {
    format!("{:<24}{}", label, value)
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

    let ideas = count_idea_entries(&project.ideas());

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
    let adr_line = if adr_breakdown.is_empty() {
        "0 ADRs".to_string()
    } else {
        adr_breakdown.join(" · ")
    };

    let arch_count = ArchitectureModuleSyncRule.run(&project).len();
    let term_count = ContextAvoidTermRule.run(&project).len();

    println!(
        "{}",
        line("IDEAS.md", &format!("{} unclaimed entries", ideas))
    );
    println!();
    println!(
        "{}",
        line(
            "RFC",
            &format!(
                "{} Draft · {} Accepted · {} Rejected",
                rfc_draft, rfc_accepted, rfc_rejected
            )
        )
    );
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
    println!(
        "{}",
        line(
            "Architecture overview",
            &format!(
                "{} module-table {} against the code directory",
                arch_count,
                if arch_count == 1 {
                    "mismatch"
                } else {
                    "mismatches"
                }
            )
        )
    );
    println!();
    println!(
        "{}",
        line(
            "Terminology (CONTEXT)",
            &format!(
                "{} Avoid-term {}",
                term_count,
                if term_count == 1 {
                    "violation"
                } else {
                    "violations"
                }
            )
        )
    );

    0
}
