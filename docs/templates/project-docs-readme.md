---
managed_by: docent
policy_version: 3
generated_by: docent {tool_version}
---

# Documentation Policy

This is the operational documentation policy for this project. It belongs to
the project and is maintained here; `docent init` never overwrites it. Run
`docent docs show` for the complete canonical specification shipped with the
installed Docent build, or export it explicitly with
`docent docs export <directory-or-file>`.

## Authority and update rules

| Artifact | Authority and owner | Update rule |
|---|---|---|
| `docs/IDEAS.md` | Unclaimed ideas; anyone may add | Remove an entry once it has an owner. It is intake, not history or backlog. |
| `docs/rfcs/` | Claimed design exploration, proposal, and decision discussion with a bounded question; RFC author while Draft | Revise freely while Draft. Seal as Accepted or Rejected only after the decision is made; an Accepted RFC links its ADR or explains why no ADR is required. |
| `docs/adrs/` | Durable architectural decisions; human decision owner | Never erase historical bodies. Append a dated clarification only when meaning is unchanged; use a new scoped amendment ADR or whole-decision supersession otherwise. |
| `docs/architecture.md` | Current module map and responsibility/dependency boundaries; architecture owner | Update topology after its ADR. Detailed-design link maintenance is allowed when no declared module Path, responsibility, or dependency boundary changes. |
| Module README / sidecar | Current detailed design; implementation owner | Overwrite to match current intended behavior. Keep current invariants, implicit contracts, rationale, and Non-goals. |
| `AGENTS.md` | Durable cross-cutting Agent guardrails; maintainers | Keep concise. Point to this policy, active ADRs, and designs rather than copying implementation detail. |
| `CONTEXT.md` | Canonical project terminology; nearest owner | Add only project-specific terms and use the defined terms consistently. |

Run `docent lint` after documentation changes that it validates.

## Governance chain

Idea capture is intake, not a governance layer. Claimed work goes to direct
implementation/design, a project PRD or issue, or an RFC. Use a Draft RFC for
a claimed, durable design exploration once it has a bounded question, even
when its preferred option, feasibility, or implementation scope is not yet
settled. The Draft may accumulate evidence and change freely until the decision
owner accepts, rejects, or splits it. Use a PRD or issue for concrete acceptance
and delivery planning, not as the durable owner of an unresolved design
exploration.

Accepted means approved and sealed, not automatically “has an ADR.” Create an
ADR only for a durable consequential architectural trade-off or a
responsibility/interface constraint. Every Accepted RFC must either link that
ADR or record `ADR not required` with a short reason.

For ADR evolution, use a stable lowercase kebab-case name for every partially
amended decision scope. Only one Accepted ADR may amend a given target scope;
a later amendment supersedes the prior amendment ADR. `docent lint` validates
these relations but never invents or auto-fixes their semantic scope.

The architecture overview is the current global map. Its table declares a
logical `Module` and project-relative implementation `Path`; a path may be a
file or a directory. Do not infer modules from top-level directories or from a
detailed-design link.

## Detailed design and dependency contracts

Put detailed design beside the implementation it owns:

- a directory module uses `{module}/README.md`;
- a file-backed module uses adjacent `{stem}.design.md`;
- independent mechanisms use the module README plus adjacent mechanism
  sidecars.

Split mixed documents by ownership. `docs/design/` is exceptional: use it only
for a genuinely cross-cutting current contract with no honest single owner, and
link it from the architecture overview to every affected module path.

Keep an external dependency contract beside the project-owned Adapter or
wrapper that contains the dependency. Manifests, lockfiles, and patch files
are authoritative for exact versions, integrity hashes, and patch identities.
The owner’s design records non-obvious upstream behavior, upgrade constraints,
and integration contracts. Put costly-to-reproduce inspection evidence in
`docs/research/` and link it from that design. Use `docs/dependencies/` only
when there are genuinely multiple equal owners and no controlling seam.

## Parallel artifacts

These artifacts run alongside, not inside, the governance chain:

| Location or convention | Audience / authority | Owner, update, retirement |
|---|---|---|
| `docs/guide/` | Users; task-oriented guidance | Feature owner updates with user behavior; retire or redirect when obsolete. |
| `docs/research/` | Maintainers; reproducible evidence | Investigator owns it; preserve while evidence is costly or uniquely useful. |
| `docs/agents/` | Agents and maintainers; process instructions | Process owner updates when workflow changes; retire when superseded. |
| Project PRDs/issues (for example `.scratch/`) | Delivery team; acceptance and execution planning | Product/work owner updates during delivery; archive after results enter maintained contracts/tests. `.scratch/` is optional. |
| Runbooks | Operators; current operational procedure | Operational owner updates after incidents or procedure changes; retire or archive when replaced. |

## Current state, archives, and deletion

Code is evidence of actual state. Accepted ADRs, maintained public contracts,
and human-confirmed acceptance criteria are evidence of intended state. Report
their disagreement for human resolution. Do not silently rewrite a maintained
contract to legitimize a regression, and do not rewrite code merely because an
obsolete historical document disagrees.

Archive non-current history such as completed plans, roadmaps, review agendas,
and legacy inputs in `docs/archive/` or the nearest `_archive/`. An archive
header states status, archive date, reason, and the current authority or
replacement when one exists. Active documents must not rely on archived files
for behavior or constraints. Closed RFCs and Superseded/Deprecated ADRs stay
in their own indexes and status lifecycle; do not put arbitrary history under
`docs/adrs/_archive/`.

Archive only unique rationale or costly evidence. Delete pure duplication when
Git history is sufficient.

Living design is current, not backlog or phase history. Put unclaimed future
work in `docs/IDEAS.md`; put claimed, bounded, unresolved design exploration in
a Draft RFC; and put concrete accepted delivery work in a project PRD or issue.
Preserve valuable completed history in an archive. A current Non-goal may name
deliberately excluded work and link its real owner.
