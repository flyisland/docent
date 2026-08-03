# docent

A command-line tool for checking mechanical consistency across project documentation (RFC / ADR / architecture overview / CONTEXT.md / AGENTS.md), and between those documents and the code.

## Language

**Rule**:
A single, independent piece of lint-check logic, identified by a unique rule ID (e.g., `adr-index-sync`).
_Avoid_: Check, Validator, Linter (use Rule consistently for "one piece of check logic"; Linter refers only to docent as a whole tool, never to an individual rule)

**Violation**:
A single instance where a Rule detected non-compliance at a specific file/location — the basic unit of `docent lint` output.
_Avoid_: Issue, Finding, Problem

**Fix**:
A mechanical correction automatically applied by docent in `--fix` mode, with a strictly limited scope (see Section 6 of the *docent Implementation Spec*).
_Avoid_: Repair, Correction, Patch

**Frontmatter**:
The YAML metadata block at the top of an RFC / ADR file.
_Avoid_: Metadata block, Header

**Superseded**:
An ADR status meaning the decision has been replaced by a newer ADR — there is a clearly identified replacement.
_Avoid_: Replaced, Overridden

**Partial amendment**:
A relation in which a newer ADR changes one explicitly named decision scope in
an Accepted ADR while the rest of that ADR remains in force.
_Avoid_: Partial supersession, Override

**Deprecated**:
An ADR status meaning the decision is no longer valid, but has no replacement. Distinct from Superseded — do not conflate the two. Deprecated has no corresponding new ADR; Superseded does.

**Fixture**:
A minimal sample project under `tests/fixtures/` used to verify a Rule, split into `valid-project` (should produce zero Violations) and `broken-project` (should trigger every Rule at least once).
_Avoid_: Sample project, Test data

**Severity**:
The level of a Violation — only `error` or `warning`, no third tier.
_Avoid_: Level, Priority
