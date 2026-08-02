---
id: adr-003
title: Payment retry policy v2
status: Accepted
implementation: implemented
created: 2026-07-02
updated: null
supersedes: [adr-002]
superseded_by: null
related_rfc: null
---

# ADR-003: Payment Retry Policy v2

## Context

The fixed-delay policy from adr-002 caused thundering herds under load.

## Decision

Adopt exponential backoff with jitter, superseding adr-002.

## Non-goals

No distributed retry coordination.

## Consequences

Positive impact: smoother load under spikes.
Known costs: slightly longer worst-case retry latency.
