# Documentation Guide

This directory follows the lifecycle defined in the [Software Project Design
and Documentation Management Specification](reference/software-project-documentation-specification.md).
The specification is the project-level source for lifecycle principles; this
guide records how that lifecycle is applied in docent.

## Documentation lifecycle

1. Capture an unformed idea in `IDEAS.md`. It is a one-way funnel: once an
   idea has a destination, remove it from that file.
2. Record proposals and their discussion in `rfcs/`. Close each RFC as
   `Accepted` or `Rejected` and retain it as history.
3. Record accepted architectural decisions in `adrs/`. ADR bodies are
   immutable; amend a decision with a new ADR and mark the older one
   `Superseded` or `Deprecated` as appropriate.
4. Keep the current module map in `architecture.md` and implementation
   details in the relevant module `README.md` or `{name}.design.md`.
5. Keep only durable, cross-cutting constraints in the root `AGENTS.md`.
   Canonical terminology belongs in the nearest applicable `CONTEXT.md`.

Run `docent lint` after changing documentation that it validates.

## Reference documents

Files in `reference/` are supporting material, not an alternative decision
record. A reference document may be current only when another maintained
document explicitly identifies it as authoritative for its subject.

| Document | Role | Status |
|---|---|---|
| [docent Implementation Spec](reference/docent-implementation-spec.md) | Current behavioral contract for the CLI | Active |
| [Software Project Design and Documentation Management Specification](reference/software-project-documentation-specification.md) | Lifecycle and documentation-management principles | Active |

## Archiving

Do not delete a document merely because its active work is complete or its
content no longer describes the current system. Move it to the closest
relevant `_archive/` directory and add a short header stating its status,
archive date, and reason. Archived documents are historical evidence, not
current authority; maintained documents must not rely on them for an active
contract or decision.

Completed implementation plans are archived once their outcomes have been
captured by the implementation, its tests, and the maintained contract or
ADRs. They should not be updated to direct subsequent feature work.

| Document | Archived | Reason |
|---|---|---|
| [docent Implementation Plan](reference/_archive/docent-implementation-plan.md) | 2026-08-03 | v1 delivery plan completed; the implementation spec and ADR-001 remain authoritative |
