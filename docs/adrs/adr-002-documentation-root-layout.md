---
id: adr-002
title: Documentation-root layout
status: Accepted
implementation: implemented
created: 2026-08-03
updated:
supersedes: []
superseded_by:
related_rfc:
---

# ADR-002: Documentation-root layout

## Context

ADR-001 established Docent's initial architecture, including a root-level idea
intake path. That otherwise broad decision remains current: its technology,
parsing, JSON, and bounded-fix choices are unaffected. This ADR only refines
the idea-intake location.

## Decision

Use `docs/IDEAS.md` as the fixed idea-intake path. `docent init` creates it,
and `docent status` counts and reports that path. This refines only the
root-level `IDEAS.md` location in ADR-001; it does not supersede ADR-001.

## Non-goals

- Do not auto-migrate or delete an existing root-level `IDEAS.md` in a target
  project.
- Do not make idea intake a required lint source; it remains status-only.
- Do not make documentation paths configurable in this change.

## Consequences

New projects have all documentation lifecycle artifacts under `docs/` except
the cross-cutting root `AGENTS.md` and optional root `CONTEXT.md`. Existing
projects can move their idea file explicitly when they adopt policy version 2.
