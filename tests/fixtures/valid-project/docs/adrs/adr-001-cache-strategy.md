---
id: adr-001
title: Cache strategy selection
status: Accepted
implementation: implemented
created: 2026-07-15
updated: 2026-07-20
supersedes: [adr-002]
superseded_by: null
related_rfc: rfc-001
---

# ADR-001: Cache Strategy Selection

## Context

The service relies on repeated expensive lookups. A cache layer is needed to keep latency low, while the maintainer has limited bandwidth and favors the simplest viable solution.

## Decision

Use a local in-memory cache with a fixed TTL, per rfc-001.

## Non-goals

No distributed cache, no cache invalidation bus in v1.

## Consequences

Positive impact: lower latency and fewer upstream calls.
Known costs: bounded staleness is accepted as a trade-off.
