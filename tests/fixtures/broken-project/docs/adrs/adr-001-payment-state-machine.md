---
id: adr-001
title: Payment state machine
status: Accepted
implementation: pending
created: 2026-07-15
updated: null
supersedes: []
superseded_by: null
related_rfc: null
---

# ADR-001: Payment State Machine

## Context

Payments can move through several states, and the transition rules are non-trivial.

## Decision

Use a state machine with explicit transition definitions.

## Consequences

Positive impact: explicit transitions and easy auditing.
Known costs: more boilerplate per state.
