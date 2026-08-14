pub mod adr_index_sync;
pub mod adr_missing_sections;
pub mod adr_pending;
pub mod agents_adr_reference;
pub mod amendment_backlink;
pub mod architecture_module_sync;
pub mod document_filename_id;
pub mod document_id_format;
pub mod document_id_unique;
pub mod fix;
pub mod frontmatter_schema;
pub mod project_policy_version;
pub mod required_source;
pub mod rfc_accepted_outcome;
pub mod rfc_index_sync;
pub mod rfc_stale;
pub mod superseded_backlink;

use crate::model::{Project, Violation};

pub trait Rule {
    fn id(&self) -> &'static str;
    fn run(&self, project: &Project) -> Vec<Violation>;
}

pub fn run_all(project: &Project) -> Vec<Violation> {
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(frontmatter_schema::FrontmatterSchemaRule),
        Box::new(document_filename_id::DocumentFilenameIdRule),
        Box::new(document_id_format::DocumentIdFormatRule),
        Box::new(document_id_unique::DocumentIdUniqueRule),
        Box::new(rfc_index_sync::RfcIndexSyncRule),
        Box::new(rfc_accepted_outcome::RfcAcceptedOutcomeRule),
        Box::new(rfc_stale::RfcStaleDraftRule),
        Box::new(adr_index_sync::AdrIndexSyncRule),
        Box::new(adr_missing_sections::AdrMissingSectionsRule),
        Box::new(superseded_backlink::SupersededBacklinkRule),
        Box::new(amendment_backlink::AmendmentBacklinkRule),
        Box::new(adr_pending::AdrPendingRule),
        Box::new(required_source::RequiredSourceMissingRule),
        Box::new(architecture_module_sync::ArchitectureModuleSyncRule),
        Box::new(agents_adr_reference::AgentsAdrReferenceRule),
        Box::new(project_policy_version::ProjectPolicyVersionRule),
    ];
    let mut all = Vec::new();
    for r in rules {
        all.extend(r.run(project));
    }
    all.sort_by(|a, b| {
        (doc_group(&a.file), &a.file, a.line, &a.rule).cmp(&(
            doc_group(&b.file),
            &b.file,
            b.line,
            &b.rule,
        ))
    });
    all
}

fn doc_group(file: &str) -> u8 {
    if file == "docs/IDEAS.md" {
        0
    } else if file.starts_with("docs/rfcs") {
        1
    } else if file.starts_with("docs/adrs") {
        2
    } else if file == "docs/architecture.md" {
        3
    } else if file == "AGENTS.md" {
        4
    } else {
        5
    }
}
