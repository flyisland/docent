pub mod adr_index_sync;
pub mod adr_missing_sections;
pub mod adr_pending;
pub mod agents_adr_reference;
pub mod architecture_module_sync;
pub mod context_avoid_term;
pub mod fix;
pub mod frontmatter_schema;
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
        Box::new(rfc_index_sync::RfcIndexSyncRule),
        Box::new(adr_index_sync::AdrIndexSyncRule),
        Box::new(adr_missing_sections::AdrMissingSectionsRule),
        Box::new(superseded_backlink::SupersededBacklinkRule),
        Box::new(agents_adr_reference::AgentsAdrReferenceRule),
        Box::new(architecture_module_sync::ArchitectureModuleSyncRule),
        Box::new(context_avoid_term::ContextAvoidTermRule),
        Box::new(adr_pending::AdrPendingRule),
        Box::new(rfc_stale::RfcStaleDraftRule),
    ];
    let mut all = Vec::new();
    for r in rules {
        all.extend(r.run(project));
    }
    all.sort_by(|a, b| (&a.file, a.line, &a.rule).cmp(&(&b.file, b.line, &b.rule)));
    all
}
