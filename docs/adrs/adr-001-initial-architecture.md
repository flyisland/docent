---
id: adr-001
title: Initial architecture selection
status: Accepted
implementation: pending
created: 2026-08-01
updated: null
supersedes: []
superseded_by: null
related_rfc: null
---

# ADR-001: Initial Architecture Selection for docent

## Context

docent is a companion CLI tool implemented alongside the *Software Project Design and Documentation Management Specification*, used to automatically check mechanical consistency between RFCs / ADRs / the architecture overview / CONTEXT.md / AGENTS.md, and between those documents and the code.

This tool is maintained by a single person / small team, and is an internal tool — it doesn't need a full multi-party RFC discussion process to decide its initial architecture. These architectural choices were already discussed and settled in the *docent Implementation Spec*; this ADR formally archives the architectural decisions already made in that spec document into the project's own decision history, rather than fabricating a fresh round of discussion — avoiding maintaining the same rationale in two places.

Resource constraints at decision time: the maintainer has limited bandwidth, and favors a mature ecosystem, low learning curve, and a build that's easy to distribute as a single binary, rather than over-engineering for extensibility needs that don't yet exist.

## Decision

- Language: Rust; CLI framework: `clap` (v4, derive-macro style).
- RFC / ADR front matter uses YAML, parsed via `serde` + `serde_yaml`; see Sections 3.1 / 3.2 of the *docent Implementation Spec* for the exact schema.
- v1's directory structure is a hardcoded convention (`docs/rfcs/`, `docs/adrs/`, `docs/architecture.md`, `AGENTS.md`, `IDEAS.md`, `CONTEXT.md`), with no custom path configuration support.
- Lint rules parse Markdown structure (headings, front matter, tables) using regular expressions and simple string matching, without a full Markdown AST parser.
- `--json` output is treated as a stable interface contract for Coding Agents to consume; future changes may only add fields, never remove or rename existing ones.
- The automated-fix scope of `--fix` is strictly limited to three mechanical operations: rebuilding index tables, rebuilding `index.json`, and completing Superseded bidirectional links — see Section 6 of the implementation spec for details.

## Non-goals

- No semantic checks (e.g., judging "has this RFC leaked tactical implementation detail") — this kind of context-dependent judgment is left to manual / Agent review, and is not docent's responsibility.
- No custom path configuration in v1.
- No GUI, no long-running service, no git integration.
- No full Markdown AST parser in v1 — the known limitations of the regex-based approach are accepted.

## Consequences

**Positive impact**:
- A single, simple tech stack keeps the learning curve and long-term maintenance cost low.
- `--json` output lets a Coding Agent automatically verify the documentation it writes within a "write docs → run checks → read structured errors → fix → run checks again" loop, with no human involvement required.
- The `--fix` scope is deliberately kept very small, so it will never accidentally alter semantic content — automated fixes are guaranteed to be safe.

**Known costs (deliberately accepted, not defects)**:
- Regex-based Markdown parsing is prone to misjudging documents that deviate even slightly from the expected format (e.g., non-standard heading styles). v1 accepts this limitation and will observe the real-world false-positive rate; if the problem becomes significant enough to affect usability, an AST parser can be considered — that requires opening a new ADR to discuss, and is not an upgrade to be made silently within the scope of this decision.
- Hardcoded paths mean docent can't be used if a project's directory structure doesn't match this specification's convention. This is a deliberate v1 simplification; configurable paths are a future RFC topic, out of scope for v1 implementation.
- The `architecture-module-sync` rule may have a relatively high false-positive rate, which is why it's defined as a warning rather than an error in the implementation spec. This is a known precision trade-off, not a bug, and should not be unilaterally changed to an error without new evidence.
