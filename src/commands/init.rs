use std::path::Path;

const IDEAS: &str = "# IDEAS\n\n_A one-way funnel: jot half-formed thoughts down here. Once an idea's destination is decided, remove it from this file. See the documentation lifecycle in docs/README.md._\n";

const RFC_INDEX: &str =
    "# Table of RFCs\n\n|ID|Title|Status|Date|Linked ADR|\n|---|---|---|---|---|\n";

const ADR_INDEX: &str =
    "# Table of ADRs\n\n|ID|Title|Status|Implementation|Last Updated|\n|---|---|---|---|---|\n";

const ARCHITECTURE: &str = "# System Architecture Overview\n\n## Module breakdown\n| Module | Path | Responsibility | Detailed design | Linked ADR |\n|---|---|---|---|---|\n\n## Inter-module dependencies\n(Who calls whom, and whether reverse dependencies are allowed)\n\n## Boundary principles\n(Which interactions must go through an interface layer; cross-module access to internal data structures is prohibited, etc.)\n";

const DOCS_GUIDE: &str = include_str!("../../docs/templates/project-docs-readme.md");

const AGENTS: &str = include_str!("../../docs/templates/project-agents.md");

const RFC_TEMPLATE: &str = "---\nid: rfc-000\ntitle: <one-line title>\nstatus: Draft\ncreated: YYYY-MM-DD\nupdated:\nrelated_adr:\n---\n\n# RFC-000: <Title>\n\n## Motivation\n\n## Options considered\n\n## Technical feasibility analysis\n\n## Decision\n\n## Outcome\n\nFor an Accepted RFC, either link the durable ADR in `related_adr`, or state `ADR not required: <short reason>`.\n";

const ADR_TEMPLATE: &str = "---\nid: adr-000\ntitle: <one-line title>\nstatus: Accepted\nimplementation: pending\ncreated: YYYY-MM-DD\nupdated:\nsupersedes: []\nsuperseded_by:\namends: []\namended_by: []\nrelated_rfc:\n---\n\n# ADR-000: <Title>\n\n## Context\n\n## Decision\n\n## Non-goals\n\n## Consequences\n";

const CONTEXT_TEMPLATE: &str = "# {Module / project name}\n\n{One or two sentences describing what this semantic boundary is and why it exists}\n\n## Language\n\n**{Term}**:\n{One or two sentence definition}\n_Avoid_: {word1}, {word2}\n";

fn files() -> Vec<(&'static str, String)> {
    vec![
        ("docs/IDEAS.md", IDEAS.to_string()),
        ("docs/rfcs/README.md", RFC_INDEX.to_string()),
        ("docs/adrs/README.md", ADR_INDEX.to_string()),
        ("docs/architecture.md", ARCHITECTURE.to_string()),
        (
            "docs/README.md",
            DOCS_GUIDE.replace("{tool_version}", env!("CARGO_PKG_VERSION")),
        ),
        ("AGENTS.md", AGENTS.to_string()),
        ("docs/.templates/rfc.md", RFC_TEMPLATE.to_string()),
        ("docs/.templates/adr.md", ADR_TEMPLATE.to_string()),
        ("docs/.templates/context.md", CONTEXT_TEMPLATE.to_string()),
    ]
}

pub fn run() -> i32 {
    for (path, content) in files() {
        if Path::new(path).exists() {
            println!("- skipped {} (already exists)", path);
            continue;
        }
        if let Some(parent) = Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match std::fs::write(path, content) {
            Ok(_) => println!("✓ created {}", path),
            Err(e) => println!("✗ failed to create {} ({})", path, e),
        }
    }
    0
}
