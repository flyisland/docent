---
id: rfc-001
title: Cache strategy selection
status: Accepted
created: 2026-07-15
updated: 2026-07-20
related_adr: adr-001
---

# RFC-001: Cache Strategy Selection

## Motivation

Repeated expensive lookups hurt latency.

## Options considered

Local in-memory cache versus a remote cache service.

## Technical feasibility analysis

A remote cache adds a dependency and operational burden that v1 does not need.

## Decision

Adopt a local in-memory cache, formalized in adr-001.
