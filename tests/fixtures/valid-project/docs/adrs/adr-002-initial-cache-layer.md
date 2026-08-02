---
id: adr-002
title: Initial cache layer choice
status: Superseded
implementation: implemented
created: 2026-07-01
updated: null
supersedes: []
superseded_by: adr-001
related_rfc: null
---

# ADR-002: Initial Cache Layer Choice

## Context

The prototype cached results in a single global registry, with no retention policy.

## Decision

Use a global in-memory registry with no TTL.

## Non-goals

No eviction, no sizing, no observability hooks.

## Consequences

Known costs: unbounded memory growth, which motivated adr-001.
