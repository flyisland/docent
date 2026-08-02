---
description: 'The execution order for implementing docent, used alongside the Software Project Design and Documentation Management Specification and the docent Implementation Spec'
status: draft
type: implementation-plan
---

# docent Implementation Plan

This document gives the implementation order, with a clear Definition of Done for each phase. Work through it in order — **don't skip a phase and jump ahead** — especially Phase 0, which is the foundation every later phase's self-verification depends on.

How the three companion documents relate:
- *Software Project Design and Documentation Management Specification*: the design rationale.
- *docent Implementation Spec*: the implementation contract (command behavior, rule definitions, schemas, output formats).
- This document: execution order and acceptance criteria.

If you find a detail in the implementation spec that conflicts with, or is ambiguous relative to, the actual implementation, stop and ask — don't decide on your own, and don't route around a rule's definition just to keep moving.

---

## Phase 0: Manual Bootstrap

**Goal**: docent itself should also be managed under this documentation specification (see the spec document's scope). But docent doesn't exist yet, so it can't generate spec-compliant documentation for itself — the output of this step is therefore **placed manually**, not produced by a scaffolding command.

**Tasks**:

1. Initialize the Rust project (`cargo init`, or manually set it up per the directory structure in Section 2 of the implementation spec).
2. Create the following directories and files in the project, using the content of the two files provided alongside this plan verbatim (no need to regenerate them, just copy them in):
   - `docs/adrs/adr-001-initial-architecture.md` (content is in the attached file `adr-001-initial-architecture.md`)
   - `CONTEXT.md` (project root; content is in the attached file `CONTEXT.md`)
3. Manually create `docs/adrs/README.md` with a single index entry (this is the one and only time the index table is maintained by hand — once `docent lint --fix` is working in a later phase, the tool should take over generating it):

   ```markdown
   |ID|Title|Status|Implementation|Last Updated|
   |---|---|---|---|---|
   |adr-001|Initial architecture selection|Accepted|Pending|2026-08-01|
   ```

4. Create an empty `docs/rfcs/README.md` (a header row is enough — there are no RFC entries yet).
5. Create a root-level `AGENTS.md`, based on the template in "Stage 5" of the spec document, with the project-specific example entries removed (e.g., "local cache implementation: see payment/README.md") — keep only the general architectural principles (route module splits through an ADR first, prefer code-level enforcement, check CONTEXT.md before naming anything, etc.).

**Acceptance criteria**: the repository contains five files — `docs/adrs/adr-001-*.md`, `docs/adrs/README.md`, `docs/rfcs/README.md`, `CONTEXT.md`, and `AGENTS.md` — matching the description above. At this point docent has no code yet; this step only builds the skeleton and requires no command to verify it.

---

## Phase 1: P0 — Minimum Viable Version

Corresponds to the P0 scope in Section 9 of the *docent Implementation Spec*.

**Tasks**:

1. Implement `docent init` (Section 4.1).
2. Implement `docent lint` (human mode only, no `--json`/`--fix`), with this rule set:
   - `frontmatter-schema-valid`
   - `rfc-index-sync`
   - `adr-index-sync`
   - `adr-missing-required-sections`
   - `superseded-backlink-consistency`
3. Implement `docent status` (basic stats only: IDEAS.md entry count, RFC status breakdown, ADR status breakdown — no architecture/CONTEXT-related lines yet, since those depend on rules that aren't implemented until Phase 2).

**Acceptance criteria**:

1. Run `docent lint` at the root of the docent repository set up in Phase 0 — **it should output 0 errors** (this is a self-verification that the manually-produced `adr-001` from Phase 0 strictly conforms to the schema — if it reports an error, first check whether it's a formatting problem in `adr-001.md` or a bug in the rule implementation; don't assume the document is at fault).
2. Per Section 8 of the implementation spec, build `tests/fixtures/valid-project/` and `tests/fixtures/broken-project/`, covering each of the five implemented rules with one positive and one negative case, and get all integration tests passing.
3. Running `docent status` against the docent repository itself should produce a sensible result (1 ADR, 0 RFCs).

Only proceed to Phase 2 once all three criteria are satisfied.

---

## Phase 2: P1 — Complete the Rule Set and Add Agent-Friendly Interfaces

Corresponds to the P1 scope in Section 9 of the implementation spec.

**Tasks**:

1. Implement `docent lint --json` (the JSON structure in Section 7.2).
2. Implement `docent lint --fix` (strictly limited to the three mechanical operations in Section 6).
3. Add the remaining rules:
   - `agents-adr-reference-valid`
   - `architecture-module-sync`
   - `context-avoid-term-violation`
   - `adr-pending-implementation-report`
   - `rfc-stale-draft`
4. Add two lines to `docent status`: architecture-overview sync status and CONTEXT violation count.

**Acceptance criteria**:

1. Run `docent lint --json` against the docent repository itself — the output structure should conform to the Section 7.2 schema, with the `errors`/`warnings`/`summary` fields all present.
2. Deliberately break index sync (e.g., hand-edit a field in `docs/adrs/README.md`) — `docent lint` should report an error; running `docent lint --fix` and then `docent lint` again should return to 0 errors.
3. Using the Avoid word list defined in `CONTEXT.md` (`Issue`, `Finding`, `Repair`, etc.), scan docent's own source code. **If the code is found to have accidentally used any of these banned words, fix the code to align terminology before continuing** — this is itself a real-world acceptance test of the `context-avoid-term-violation` rule.
4. Add a positive/negative case to `tests/fixtures/` for every new rule, and get all integration tests passing.

---

## Phase 3: Self-Check and Wrap-Up

**Tasks**:

1. Read through docent's own source files once and note down any implementation pitfall worth recording (e.g., an edge case a regex can't handle, or a surprising `serde_yaml` error-handling quirk), adding it to the `## Gotchas` section of the corresponding source directory's `README.md`. If there's nothing worth recording right now, skip this step — don't manufacture content just to have something to write.
2. Change the `implementation` field in `docs/adrs/adr-001-*.md`'s front matter from `pending` to `implemented`, and note the landing file (e.g., `implemented (src/main.rs)`).
3. Run `docent lint` one final time end-to-end, and confirm 0 errors.

**Acceptance criteria**: `docent lint` outputs 0 errors against the docent repository itself, and `adr-001`'s implementation field has been updated to `implemented`.

---

## Phase 4 (Optional): P2 Scope

Corresponds to the P2 scope in Section 9 of the implementation spec. This phase is not required for delivery — decide whether to pursue it based on time and actual need:

- The `design-doc-existence` rule
- The `adr-index-json-sync` rule
- The `--rule` single-rule debug flag
- Custom path configuration file support

If you decide to implement path configuration support, **this changes an architectural decision already locked into v1 (the Non-goal in ADR-001 stating "no custom path configuration in v1")**. Per the spec document's change criteria, this requires opening a new `adr-002` — you must not edit the body of `adr-001` directly, and you must not just quietly add a config option in code without leaving a trace.

---

## Reminder for Future Evolution

Once v1 ships, any decision with room for genuine discussion — "should we support TOML config," "should we replace the regex parser with an AST parser," "some rule's false-positive rate is too high, does it need a redesign" — should go through the normal RFC → ADR flow defined in the spec document. There's no need to realign with this implementation plan for every future change — this plan only covers the path to shipping v1.
