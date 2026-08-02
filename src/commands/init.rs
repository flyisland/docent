use std::path::Path;

const IDEAS: &str = "# IDEAS\n\n_A one-way funnel: jot half-formed thoughts down here. Once an idea's destination is decided, remove it from this file. See the documentation lifecycle in docs/README.md._\n";

const RFC_INDEX: &str =
    "# Table of RFCs\n\n|ID|Title|Status|Date|Linked ADR|\n|---|---|---|---|---|\n";

const ADR_INDEX: &str =
    "# Table of ADRs\n\n|ID|Title|Status|Implementation|Last Updated|\n|---|---|---|---|---|\n";

const ARCHITECTURE: &str = "# System Architecture Overview\n\n## Module breakdown\n| Module | Responsibility | Detailed design | Linked ADR |\n|---|---|---|---|\n\n## Inter-module dependencies\n(Who calls whom, and whether reverse dependencies are allowed)\n\n## Boundary principles\n(Which interactions must go through an interface layer; cross-module access to internal data structures is prohibited, etc.)\n";

const AGENTS: &str = "# Core Technical Constraints for This Project (System Instructions for Code Generation)\n\nBefore creating or editing any RFC, ADR, or CONTEXT.md, read the corresponding template under `docs/.templates/` first. After finishing, run `docent lint` until it reports no errors.\n\n## Architectural principles\n\n- Any change that adds/removes/merges/splits a module or adjusts responsibility boundaries between modules must first read docs/architecture.md to confirm it doesn't violate the existing module breakdown. If there's a conflict, go through the ADR process rather than editing the code directly.\n- When adding any range-of-values or state-transition constraint, prefer enforcing it in code — via the type system, enums, validation, or assertions — rather than only describing it in a comment or a document.\n- When naming anything project-specific, first read CONTEXT.md at the root or in the relevant module (if it exists). You must use the canonical term defined there — using a synonym listed under Avoid, or inventing a new synonym, is not permitted.\n\n## Documentation maintenance rules\n\n- Strictly follow the documentation lifecycle defined in docs/README.md.\n- When updating an architectural decision, only the module README may be edited — erasing the body of an ADR is prohibited.\n\n## Rule maintenance\n\n- Review and trim this file once a quarter, removing entries superseded by newer ADRs or no longer applicable.\n";

const RFC_TEMPLATE: &str = "---\nid: rfc-000\ntitle: <one-line title>\nstatus: Draft\ncreated: YYYY-MM-DD\nupdated:\nrelated_adr:\n---\n\n# RFC-000: <Title>\n\n## Motivation\n\n## Options considered\n\n## Technical feasibility analysis\n\n## Decision\n";

const ADR_TEMPLATE: &str = "---\nid: adr-000\ntitle: <one-line title>\nstatus: Accepted\nimplementation: pending\ncreated: YYYY-MM-DD\nupdated:\nsupersedes: []\nsuperseded_by:\nrelated_rfc:\n---\n\n# ADR-000: <Title>\n\n## Context\n\n## Decision\n\n## Non-goals\n\n## Consequences\n";

const CONTEXT_TEMPLATE: &str = "# {Module / project name}\n\n{One or two sentences describing what this semantic boundary is and why it exists}\n\n## Language\n\n**{Term}**:\n{One or two sentence definition}\n_Avoid_: {word1}, {word2}\n";

fn files() -> Vec<(&'static str, &'static str)> {
    vec![
        ("IDEAS.md", IDEAS),
        ("docs/rfcs/README.md", RFC_INDEX),
        ("docs/adrs/README.md", ADR_INDEX),
        ("docs/architecture.md", ARCHITECTURE),
        ("AGENTS.md", AGENTS),
        ("docs/.templates/rfc.md", RFC_TEMPLATE),
        ("docs/.templates/adr.md", ADR_TEMPLATE),
        ("docs/.templates/context.md", CONTEXT_TEMPLATE),
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
