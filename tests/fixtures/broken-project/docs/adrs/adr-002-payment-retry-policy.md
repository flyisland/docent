---
id: adr-002
title: Payment retry policy
status: Superseded
implementation: implemented
created: 2026-07-01
updated: null
supersedes: []
superseded_by: null
related_rfc: null
---

# ADR-002: Payment Retry Policy

## Context

The original retry policy was defined before the retry limits were revisited.

## Decision

Retry each attempt up to three times with a fixed delay.

## Non-goals

No exponential backoff, no jitter.

## Consequences

Known costs: fixed delays under load, which motivated adr-003.
