---
id: rfc-001
title: Module directory discovery
status: Accepted
created: 2026-08-02
updated:
related_adr:
---

# RFC-001: Module directory discovery

## Motivation

`architecture-module-sync` currently treats each non-excluded directory directly below the repository root as a module candidate. This is a layout-specific heuristic: it cannot represent projects whose production modules live below `src/`, `packages/`, `apps/`, `services/`, or another container directory. Its hardcoded exclusions also mix universal filesystem noise with project-specific architecture choices.

Until module discovery has a clear, explicit contract, warnings about directories that are absent from the architecture table are unreliable and create documentation work that is unrelated to the project's actual module boundaries.

## Options considered

### Infer modules from top-level directories

Retain the current approach, filtering top-level directories with hardcoded exclusions and `.gitignore`.

This is zero-configuration, but it incorrectly equates repository layout with architecture and does not work for many conventional project layouts.

### Infer module roots from `Detailed design`

Use a value such as `src/layout/README.md` in the architecture table to infer that `src/layout/` is a module directory, then discover sibling modules below `src/`.

This provides a useful signal, but a detailed-design document is not necessarily colocated with code, one document may cover multiple modules, and scanning siblings reintroduces an implicit convention that every sibling directory is a module.

### Validate declared modules only

Treat the architecture table as authoritative. Verify that every declared module has a corresponding declared module path, but do not attempt to discover undocumented modules from the filesystem.

This avoids false positives and supports arbitrary repository layouts. It cannot automatically identify a newly created, undocumented module.

### Add an explicit discovery contract

Add an explicit `Path` column to the architecture table for each module. A path may name either a file or a directory. If automatic omission detection is needed later, introduce separate, opt-in discovery roots or marker files.

This preserves automatic omission detection without treating filesystem placement or `Detailed design` links as universal architecture semantics.

## Technical feasibility analysis

The current rule already parses the architecture table, but compares the `Module` column directly with root-directory names. Supporting arbitrary layouts requires separating a module's logical name from its implementation path. A `Path` column is sufficient for declared-module existence validation: the value is relative to the project root and may identify a file or directory. Optional discovery would require a separately specified configuration and matching algorithm.

`.gitignore` remains useful as a traversal filter, but it must not define module identity: it expresses version-control policy rather than architectural boundaries. Universal metadata and generated-artifact exclusions can remain traversal concerns independently of any module discovery policy.

## Decision

Remove the current reverse check that reports every inferred directory not listed in the architecture table. Retain validation for architecture-table entries, but validate an explicitly declared relative `Path` rather than infer a root-level path from the module name. `Module` is a stable logical name, not a filesystem path.

Do not implement automatic module discovery in this change. Defer it until a subsequent RFC defines an opt-in discovery contract. `Detailed design` links may inform that future proposal but are not a source of module-directory truth.

## Outcome

ADR not required: this RFC refines Docent's bounded validation behavior without
making a separate durable architectural trade-off.
