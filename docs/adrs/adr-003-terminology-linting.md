---
id: adr-003
title: Terminology lint removal
status: Accepted
implementation: implemented
created: 2026-08-03
updated:
supersedes: []
superseded_by:
amends:
- adr: adr-001
  decision: terminology-linting
amended_by: []
related_rfc:
---

# ADR-003: Terminology lint removal

## Context

The original terminology rule treated every `CONTEXT.md` Avoid entry as a
global ban on source-code tokens. An Avoid entry describes a misleading name
when referring to one specific concept, not a word that can never appear in a
program. The same word may also be the canonical name for another concept.

Consequently, neither a full-text source scan nor comparing Avoid entries to
other canonical terms can yield reliable violations: both require a semantic
judgment Docent cannot make.

## Decision

Do not lint `CONTEXT.md` Avoid entries. In particular, do not scan business
source files for them and do not report a conflict merely because an Avoid
entry is another canonical term. Avoid entries remain human and Agent guidance
for distinguishing concepts.

This ADR amends only ADR-001's `terminology-linting` decision scope. All other
initial architecture choices remain in force.

## Non-goals

- Do not determine whether a word is semantically misused in prose or code.
- Do not automatically rewrite terminology lists or source code.
- Do not impose a global reserved-word vocabulary on projects using Docent.

## Consequences

`docent lint` and `docent status` have no terminology Avoid rule. An ordinary
source token or a deliberate cross-reference between terms cannot produce a
false diagnostic.
