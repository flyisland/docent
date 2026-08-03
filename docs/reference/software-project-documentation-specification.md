---
description: Complete lifecycle and placement rules for software-project documentation
policy_version: 2
status: active
type: evergreen
---

# Software Project Design and Documentation Management Specification

This is the canonical documentation policy maintained by Docent. A target
project receives a small operational policy, not a default copy of this file.
Humans set direction, accept decisions, and resolve conflicts. Agents assess
evidence, maintain living documents and code, and escalate disagreement rather
than silently choosing an intended state.

## Model and principles

The core governance chain has five layers: RFC, ADR, architecture overview,
detailed design, and Agent guardrails. `IDEAS.md` is an intake mechanism before
that chain, not a sixth layer. Guides, research, execution records, archives,
and runbooks are parallel artifact classes.

Write only information that code or a more appropriate owner cannot answer.
Each fact has one authority: downstream documents link to an upstream decision
instead of repeating it. Code is evidence of **actual state**; Accepted ADRs,
maintained public contracts, and human-confirmed acceptance criteria are
evidence of **intended state**. Report disagreement for human resolution. An
Agent must neither rewrite a maintained contract to legitimize a regression nor
rewrite code merely because obsolete history disagrees.

## Intake: `IDEAS.md`

`IDEAS.md` is a one-way funnel for unclaimed, half-formed ideas. Once claimed,
remove the entry and send it to direct implementation/design, a project PRD or
issue, or an RFC. Keep it only while scope remains unclear. It is neither a
backlog nor an archive.

## Layer 1: RFCs

RFCs in `docs/rfcs/` record a proposal, material alternatives, feasibility,
and the decision. Their statuses are Draft, Accepted, or Rejected. Closed RFCs
are sealed historical decision records, not living design.

Accepted means **approved and sealed**. It must either:

1. link, through `related_adr`, the ADR that captures a durable architectural
   decision; or
2. explicitly state `ADR not required` with a short reason in its outcome.

An ADR is warranted by a durable consequential architectural trade-off or a
responsibility/interface constraint. Several changed files or a bounded public
feature alone do not require one. The RFC index contains ID, title, status,
date, and linked ADR (or `—`).

## Layer 2: ADRs

ADRs in `docs/adrs/` record durable architectural decisions. An Accepted ADR
is current; Superseded means replaced by a newer ADR; Deprecated means no
longer applicable without a replacement. Keep implementation status separate
from decision status. ADR bodies are historical evidence: do not erase them.

A major or directional replacement requires a new ADR and supersession links.
If a project permits a narrowly scoped minor clarification or numeric change,
append an Amendment with date and rationale; do not rewrite history. ADRs have
Context, Decision, Non-goals, and Consequences sections and remain in their
normal index regardless of status.

## Layer 3: architecture overview

`docs/architecture.md` is the living module map: current responsibility
boundaries, dependencies, and links to detailed design and ADRs. Its module
table uses this form:

| Module | Path | Responsibility | Detailed design | Linked ADR |
|---|---|---|---|---|

`Module` is a stable logical identifier. `Path` is the declared
project-relative implementation owner and may identify a file or directory.
Do not infer module identity from top-level directories, from filesystem
siblings, or from the location of a detailed-design link. This aligns with
RFC-001 and Docent's `architecture-module-sync` contract: validate declared
paths only.

Examples:

```text
src/config.ts
src/config.design.md

src/layout/
src/layout/README.md
src/layout/flex-layout.ts
src/layout/flex-layout.design.md
```

Update the overview when a module is added, removed, merged, split, or its
responsibility/dependency boundary changes. Such a durable change normally
needs the ADR process first; then overwrite the living map.

## Layer 4: detailed design

Detailed design records current invariants, implicit contracts, rationale that
is not obvious from code, integration boundaries, and current Non-goals. It
does not restate types or line-by-line code behavior.

Place it beside its implementation owner:

- a directory-backed module uses `{module}/README.md`;
- a file-backed module uses adjacent `{stem}.design.md`;
- multiple independent mechanisms use the module README plus adjacent
  mechanism sidecars.

Split a mixed historical document by ownership rather than moving it whole to
preserve its old shape. `docs/design/` is exceptional: use it only for a
genuinely cross-cutting current contract with no honest single implementation
owner. `architecture.md` must link such a document from every affected module
path.

Living design is not a backlog or phase history. Unclaimed work belongs in
`IDEAS.md`; claimed product/interface work belongs in an RFC or project
PRD/issue; completed phase history belongs in an archive if retention is
useful. Avoid “first slice”, “initial stage”, “deferred”, and “future
capability” unless describing a current deliberate Non-goal that links to its
real owner.

### External dependency contracts

Place a dependency contract beside the project-owned Adapter or wrapper that
contains that dependency. Manifests, lockfiles, and patch files are
authoritative for exact versions, integrity hashes, and patch identities. The
owning detailed design records non-obvious upstream behavior, upgrade
constraints, and integration contracts. Costly-to-reproduce inspection
evidence may live in `docs/research/` and must be linked from that design.
Use `docs/dependencies/` only when a dependency truly has multiple equal
owners and no controlling seam.

## Layer 5: Agent guardrails and terminology

Root `AGENTS.md` contains only durable cross-cutting constraints. It points to
the operational policy, active ADRs, and detailed designs instead of copying
implementation detail. It tells Agents to read the applicable templates and
run mechanical validation. `CONTEXT.md`, at root or a module boundary,
defines project-specific canonical terms and Avoid terms; it is not a second
architecture map.

## Parallel artifact classes

| Class | Audience | Authority | Owner and update mode | Retirement |
|---|---|---|---|---|
| `docs/guide/` | Users | Task guidance, not architecture | Feature owner updates with user-facing behavior | Redirect, archive, or delete when obsolete and non-unique |
| `docs/research/` | Maintainers | Reproducible evidence, not a decision | Investigator updates while evidence matters | Archive or delete when replaced and Git/history is enough |
| `docs/agents/` | Agents and maintainers | Process instructions below root guardrails | Process owner updates with workflow | Retire when superseded |
| PRD/issue storage (for example `.scratch/`) | Delivery team | Acceptance and execution planning | Product/work owner updates during delivery | Archive after maintained contracts and tests carry the result |
| Runbooks | Operators | Current operations procedure | Operational owner updates after procedure/incident changes | Retire or archive when replaced |
| Archive locations | Maintainers | Historical evidence only | Original owner preserves unique rationale/evidence | Delete pure duplication when Git is sufficient |

`.scratch/` is one valid convention, never a required directory.

## Historical documents and archives

Put non-normative history—completed plans, roadmaps, review agendas, and
legacy design inputs—in `docs/archive/` or the nearest relevant `_archive/`.
Every archived document has a header with status, archive date, reason, and
the current authority/replacement when one exists. Maintained documents must
not depend on archived files for active behavior or constraints.

Closed RFCs and Superseded/Deprecated ADRs remain governed by their own status
and indexes. Do not mix arbitrary historical material into
`docs/adrs/_archive/`. Archive only unique rationale or costly evidence; pure
duplication may be deleted when Git history is sufficient.

## Operational templates

RFC front matter includes `id`, `title`, `status`, `created`, `updated`, and
`related_adr`; its body has Motivation, Options considered, Technical
feasibility analysis, Decision, and Outcome. The Outcome supplies the required
ADR link or `ADR not required` reason for an Accepted RFC.

ADR front matter includes `id`, `title`, `status`, `implementation`, dates,
supersession links, and `related_rfc`; its body has Context, Decision,
Non-goals, and Consequences. The architecture template is the five-column
table above.

## Cleanup and migration playbook

1. Inventory existing documents by apparent implementation owner; make no
   authority judgment yet.
2. Derive actual state from the relevant code and explicitly label unclear
   intent.
3. Compare each document with actual state **and** intended authorities
   (Accepted ADRs, maintained contracts, confirmed criteria). Flag conflicts
   for humans; do not resolve them by choosing code or history by default.
4. Migrate current module detail to its file or directory owner; split mixed
   documents by ownership. Put cross-cutting documents in `docs/design/` only
   under the exception rule.
5. Keep ADRs and RFCs in their indexes/statuses; archive other historical
   material with the required header, or delete duplication when Git is enough.
6. Update the architecture table, templates, Agent guardrails, and mechanical
   checks, then run validation.

Useful Agent prompt: “Describe actual state from the declared module path;
then identify conflicts with Accepted ADRs, maintained public contracts, or
human-confirmed criteria. Do not choose a winner. Propose ownership-based
placement for each current document and mark historical material for archive
or deletion using the retention rule.”

## Mechanical checks and human review

Automation should validate front matter, indexes, ADR supersession links,
active ADR references, declared architecture paths, and other structural
invariants. It must not guess whether prose is phase history, whether an ADR
was semantically warranted, or who the true owner of a dependency is. Human
review checks that current design does not duplicate code, that architecture
boundaries have appropriate decisions, and that active documents do not rely
on archives.

## Rollout checklist

- Create the five governance-layer locations and project operational policy.
- Declare each module using `Module` and file-or-directory `Path`.
- Move or split detailed design beside its implementation owner.
- Record dependency contracts at their Adapter/wrapper seam and link evidence.
- Classify non-current non-ADR material for archive or deletion.
- Ensure every Accepted RFC has an ADR link or a no-ADR reason.
- Remove backlog and phase history from living design.
- Run Docent validation and resolve only mechanical findings automatically.
