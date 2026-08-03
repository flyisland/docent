# docent

A command-line tool for checking mechanical consistency across project
documentation (RFC / ADR / architecture overview / CONTEXT.md / AGENTS.md),
and between those documents and the code.

docent is designed to be used by both humans and Coding Agents in a
"write docs → run checks → read structured output → fix → run checks again"
loop. It never makes semantic judgments — it only detects mechanical
inconsistencies (missing sections, stale indexes, broken links, schema
violations) and, in a strictly limited set of cases, applies safe automatic
fixes.

## Build

```sh
cargo build --release
```

The binary is `target/release/docent`. It has no runtime dependencies and can
be copied anywhere.

## Quick start

```sh
# 1. Scaffold a project's documentation skeleton (safe: never overwrites).
docent init

# 2. Run all rules.
docent lint

# 3. Let docent apply the mechanical fixes it's allowed to make.
docent lint --fix

# 4. Summarize the current state of the documentation.
docent status
```

All commands operate on the current working directory — run them from the
project root.

## Commands

### `docent init`

Creates the standard directory structure and template files in the current
directory. Existing files are left untouched (it prints `skipped … (already
exists)` for them and never overwrites). The exit code is always `0`.

It creates:

| Path | Purpose |
|---|---|
| `IDEAS.md` | A one-way funnel for half-formed thoughts |
| `docs/rfcs/README.md` | RFC index table |
| `docs/adrs/README.md` | ADR index table |
| `docs/architecture.md` | Architecture overview with module breakdown |
| `docs/README.md` | Self-contained documentation lifecycle and archiving guide |
| `AGENTS.md` | Coding-agent constraints, wired to the docs |
| `docs/.templates/rfc.md` | RFC front matter + section template |
| `docs/.templates/adr.md` | ADR front matter + section template |
| `docs/.templates/context.md` | CONTEXT.md term template |

### `docent lint`

Runs every rule against the current project and prints a human-readable
report. The exit code is:

- `0` when there are no errors (warnings are allowed),
- `1` when at least one error is present.

`docent lint --json` prints a machine-readable report instead (see
[JSON output](#json-output)). The `--json` exit-code semantics are the same:
`1` if any errors exist, `0` otherwise.

Violations are grouped in the same order `docent status` displays its lines:
RFC files, then ADR files, then `docs/architecture.md`, then `AGENTS.md`, then
code files. Within a group they are ordered by file path, then line, then rule
ID.

`docent lint --fix` first applies every supported automatic fix, then reports
the *post-fix* violations. It returns `0` only if no errors remain after
fixing, so it is safe to run repeatedly:

```sh
until docent lint --fix --json | jq -e '.summary.errors == 0'; do :; done
```

### `docent status`

Prints a summary of the project's documentation state: unclaimed IDEAS
entries, RFC counts by status, ADR counts by status, and any architecture /
terminology mismatches. Human output only; always exits `0`.

Each line distinguishes a missing source from an empty one: when the relevant
file (`IDEAS.md`, `docs/architecture.md`, `CONTEXT.md`) or directory
(`docs/rfcs`, `docs/adrs`) does not exist, the line reports
`not found (<path>)` instead of a misleading zero count.

Below the last line, `docent status` prints a `Violations by rule` breakdown
with a count per lint rule (rules with zero violations are omitted), sorted by
count descending.

## The rules

Each rule has a unique ID, a severity (`error` or `warning`), and an optional
`--fix` capability.

| Rule ID | What it detects | Severity | `--fix` |
|---|---|---|---|
| `frontmatter-schema-valid` | RFC / ADR front matter deviates from the required schema | error | — |
| `adr-missing-required-sections` | An ADR is missing a required section (Context / Decision / Non-goals / Consequences) | error | — |
| `adr-index-sync` | `docs/adrs/README.md` table disagrees with the ADR front matter | error | ✓ |
| `rfc-index-sync` | `docs/rfcs/README.md` table disagrees with the RFC front matter | error | ✓ |
| `superseded-backlink-consistency` | A `supersedes` reference has no matching `superseded_by` backlink | error | ✓ |
| `agents-adr-reference-valid` | `AGENTS.md` references an ADR whose status makes that reference invalid | error | — |
| `required-source-missing` | A required source (`docs/rfcs`, `docs/adrs`, `docs/architecture.md`, `AGENTS.md`) does not exist | warning | — |
| `context-avoid-term-violation` | Code uses a term CONTEXT.md marks as Avoid | error | — |
| `architecture-module-sync` | A module table entry has no safe relative `Path`, or its declared file or directory does not exist | warning | — |
| `adr-pending-implementation-report` | An Accepted ADR still records `implementation: pending` | warning | — |
| `rfc-stale-draft` | An RFC has stayed in Draft past the staleness threshold | warning | — |

The architecture module table uses `Module` as a stable logical name and
`Path` as its implementation location relative to the project root. A path may
refer to either a file or directory; it must not be absolute or traverse out of
the project when resolved. `docent` validates only paths declared in this table and never
infers additional modules from the directory tree.

### `--fix` scope

`--fix` is strictly limited to three mechanical operations and never touches
document body content or makes a semantic judgment:

1. Rebuilding the ADR index table from front matter.
2. Rebuilding the RFC index table from front matter.
3. Completing Superseded bidirectional links (filling a missing `superseded_by`
   field and the corresponding status value).

Everything else is deliberately left to human judgment.

## JSON output

`docent lint --json` emits the following structure. The `line` field is a
number when the location can be pinpointed (e.g. a term violation in code),
and `null` otherwise.

```json
{
  "errors": [
    {
      "rule": "adr-missing-required-sections",
      "file": "docs/adrs/adr-011-xxx.md",
      "line": null,
      "message": "Missing Non-goals section"
    }
  ],
  "warnings": [
    {
      "rule": "rfc-stale-draft",
      "file": "docs/rfcs/rfc-004-xxx.md",
      "line": null,
      "message": "Status has been Draft for over 60 days without an update"
    }
  ],
  "summary": {
    "errors": 1,
    "warnings": 1
  }
}
```

This structure is a stable interface contract: it may grow new optional fields,
but existing fields will never be removed or renamed.

## How a project is checked

docent reads the following from the project root:

- `docs/rfcs/*.md` (excluding `README.md`) — RFC front matter
- `docs/adrs/*.md` (excluding `README.md`) — ADR front matter
- `docs/rfcs/README.md` and `docs/adrs/README.md` — the index tables
- `docs/architecture.md` — the module breakdown table
- `CONTEXT.md` — canonical terminology (terms and their Avoid words)
- `AGENTS.md` — references to ADRs
- source files (`.rs`, `.py`, `.js`, `.ts`, and other common extensions)
  under any module directory, respecting the repository's own `.gitignore`
  (and only `.gitignore` — never `.git/info/exclude` or the global
  `core.excludesFile`, which are machine-local and would make results differ
  between environments). Each scanning rule also keeps its own hardcoded
  fallback list (notably `.git`, `target`, `docs`, `node_modules`, and test or
  build fixtures) that is always excluded; outside a git repository, or when
  the tree has no `.gitignore`, docent degrades to that fallback alone. Hidden
  files and directories are never scanned. Note docent is not git-index aware:
  a file that is committed but lives under a gitignored path is still skipped.

A missing source is reported explicitly instead of silently skipped: the
`required-source-missing` warning fires once per absent entry above
(`docs/rfcs`, `docs/adrs`, `docs/architecture.md`, `AGENTS.md`). `IDEAS.md`
and `CONTEXT.md` are not linted — `IDEAS.md` is only counted by `docent
status`, and `CONTEXT.md` is optional (its absence simply disables the
terminology rule).

New RFCs / ADRs are expected to be written from the templates in
`docs/.templates/`, which already carry a valid front matter skeleton.

## Development

```sh
cargo test          # unit + integration tests
cargo clippy        # lint the Rust code
cargo fmt --check   # formatting check
```

Test fixtures live in `tests/fixtures/`:

- `valid-project/` — a project that complies with every rule; `docent lint`
  must report 0 errors and 0 warnings.
- `broken-project/` — a project with exactly one violation per rule.

docent is dogfooded: running `docent lint` in this repository's own root should
report 0 errors.

Implementation notes live in `src/README.md` (see the `## Gotchas` section for
known parser edge cases). The full behavior contract is specified in
`docs/reference/docent-implementation-spec.md`.
