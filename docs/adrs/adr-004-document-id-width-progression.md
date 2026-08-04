---
id: adr-004
title: Document ID width progression
status: Accepted
implementation: implemented
created: 2026-08-04
updated:
supersedes: []
superseded_by:
amends:
- adr: adr-001
  decision: document-id-format
amended_by: []
related_rfc:
---

# ADR-004: Document ID width progression

## Context

The `document-id-format` Rule treated three- and four-digit document IDs as
two mutually exclusive project-wide formats. A project that begins with
three-digit IDs will naturally retain those documents when its sequence grows
from `999` to `1000`, so this rule would incorrectly report a Violation at the
normal numbering boundary. A project may also choose four-digit IDs from its
first document.

## Decision

Accept both three- and four-digit document IDs. A project may use either width
exclusively. It may also mix three-digit IDs with four-digit IDs from `1000`
through `9999`, which is the natural progression after `999`.

Keep `document-id-format` to report four-digit IDs below `1000` only when the
project also has three-digit IDs. This catches an inconsistent combination
such as `001` and `0011`, without rejecting a project that consistently chose
four-digit IDs from the beginning.

This ADR amends only ADR-001's `document-id-format` decision scope.

## Non-goals

- Do not renumber existing RFCs or ADRs.
- Do not permit five-digit document numbers in this version.
- Do not infer a separate number-width convention for each document type.

## Consequences

Projects can grow beyond document number `999` without changing historical
IDs or receiving false-positive Violations. Projects that begin with IDs such
as `rfc-0001` remain valid when they use that width consistently.
