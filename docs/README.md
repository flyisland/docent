---
managed_by: docent
policy_version: 2
generated_by: docent 0.1.0
---

# Documentation Policy

Docent owns the full canonical specification in
`reference/software-project-documentation-specification.md`. This project
policy is the operational interface: it is self-contained for daily work and
does not replace the canonical source. The installed build exposes the full
text through `docent docs show` and `docent docs export <directory-or-file>`.

## Governance and ownership

`docs/IDEAS.md` is one-way intake; remove an idea once it has a destination. RFCs
in `rfcs/` are proposals and seal as Accepted or Rejected. An Accepted RFC
links its durable ADR or records `ADR not required` with a short reason. ADRs
in `adrs/` are durable decisions. Append a dated Amendment only when meaning
does not change. A new ADR uses `amends`/`amended_by` to replace one named
decision scope while the target remains Accepted, or `supersedes` to replace
the target ADR as a whole. Never erase history. `architecture.md` is the
living module map, and each module README or `.design.md` is living current
design that may be overwritten to match intended behavior. `AGENTS.md` carries
only durable cross-cutting guardrails; `CONTEXT.md` owns canonical terminology.

Partial-amendment scopes use stable lowercase kebab-case names. Only one
Accepted ADR may amend a given target scope; a later amendment supersedes the
previous amendment ADR.

Run `docent lint` after changing documentation it validates.

## Module and dependency placement

The architecture table declares each logical `Module` and project-relative
`Path`. A path can name a file or directory; do not infer modules from
top-level directories or detailed-design links. Directory modules use
`{module}/README.md`; file-backed modules use adjacent `{stem}.design.md`.
Use module README plus mechanism sidecars when appropriate. Split mixed
documents by ownership. `docs/design/` is only for a genuinely cross-cutting
current contract with no honest owner, linked from every affected module path.

Dependency contracts belong beside the Adapter/wrapper that contains the
dependency. Manifests, lockfiles, and patches own exact pins, hashes, and patch
identities. The owner design holds non-obvious behavior and upgrade contracts;
costly evidence may live in `research/` and be linked from the design.
`docs/dependencies/` requires multiple equal owners and no controlling seam.

## Parallel artifacts, history, and conflict handling

`guide/` is user guidance; `research/` is reproducible evidence; `agents/` is
process instruction; PRD/issue storage such as optional `.scratch/` is delivery
planning; runbooks are operational procedure. Their owner updates them for the
relevant audience and retires them when replaced or obsolete.

Code is evidence of actual state. Accepted ADRs, maintained contracts, and
human-confirmed acceptance criteria are evidence of intended state. Report
conflicts for human resolution; do not silently rewrite either authority.

Archive completed plans, roadmaps, agendas, and legacy inputs in
`docs/archive/` or the nearest `_archive/`, recording status, date, reason, and
current replacement/authority where available. Active documents cannot depend
on archives. Closed RFCs and Superseded/Deprecated ADRs stay in their own
indexes; arbitrary history does not belong in `adrs/_archive/`. Preserve unique
rationale or costly evidence; delete pure duplication when Git history is
enough.

Living design is current, not backlog or phase history. Put unclaimed future
work in `docs/IDEAS.md`, claimed work in an RFC or PRD/issue, and valuable completed
history in an archive. A deliberate current Non-goal may link to the real
owner of related planned work.

## Reference documents

| Document | Role | Status |
|---|---|---|
| [docent Implementation Spec](reference/docent-implementation-spec.md) | Current behavioral contract for the CLI | Active |
| [Software Project Design and Documentation Management Specification](reference/software-project-documentation-specification.md) | Canonical lifecycle specification | Active |
