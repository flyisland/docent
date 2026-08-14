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

# 5. List RFCs / ADRs, optionally filtered by status.
docent list rfc --status draft

# 6. Read or explicitly export this build's complete documentation policy.
docent docs show
docent docs export docs/reference/

# 7. Review or export the latest project operational-policy template.
docent docs project-policy show
docent docs project-policy export /tmp/docent-policy/
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
| `docs/IDEAS.md` | A one-way funnel for half-formed thoughts |
| `docs/rfcs/README.md` | RFC index table |
| `docs/adrs/README.md` | ADR index table |
| `docs/architecture.md` | Architecture overview with module breakdown |
| `docs/README.md` | Versioned, self-contained operational documentation policy |
| `AGENTS.md` | Coding-agent constraints, wired to the docs |
| `docs/.templates/rfc.md` | RFC front matter + section template |
| `docs/.templates/adr.md` | ADR front matter + section template |
| `docs/.templates/context.md` | CONTEXT.md term template |

The generated policy is project-owned and `init` never overwrites it. It
contains the documentation lifecycle, placement, archive, and conflict rules;
it is not a copy of Docent's complete canonical specification.

If it skips a Docent-managed policy from an older policy version, `init` prints
a warning and points to the latest template. It leaves the project file intact.

### `docent docs show` and `docent docs export`

`docent docs show` writes the complete canonical Software Project Design and
Documentation Management Specification bundled into the installed binary to
standard output. `docent docs export <directory-or-file>` explicitly writes
the same text to a new file. A directory receives
`software-project-documentation-specification.md`; missing parent directories
are created. Export never overwrites an existing file and ordinary `init`
never exports the specification. The displayed/exported content identifies the
Docent build, policy version, and its canonical-source role.

An extensionless destination is treated as a directory; use `--file` to export
to an extensionless filename explicitly.

### `docent docs project-policy show` and `export`

These commands display or export the latest generated project-level
`docs/README.md` template. They are intended for review and manual migration
of a project-owned policy, never for overwriting it. An export directory
receives `project-docs-readme.md`; the same create-new and `--file` behavior
applies.

### `docent lint`

Runs every rule against the current project and prints a human-readable
report. The exit code is:

- `0` when there are no errors (warnings are allowed),
- `1` when at least one error is present.

`docent lint --json` prints a machine-readable report instead (see
[JSON output](#json-output)). The `--json` exit-code semantics are the same:
`1` if any errors exist, `0` otherwise.

Violations are grouped in the same order `docent status` displays its lines:
RFC files, then ADR files, then `docs/architecture.md`, then `AGENTS.md`.
Within a group they are ordered by file path, then line, then rule ID.

`docent lint --fix` first applies every supported automatic fix, then reports
the *post-fix* violations. It returns `0` only if no errors remain after
fixing, so it is safe to run repeatedly:

```sh
until docent lint --fix --json | jq -e '.summary.errors == 0'; do :; done
```

### `docent status`

Prints a summary of the project's documentation state: unclaimed IDEAS
entries, RFC counts by status, ADR counts by status, and architecture
mismatches. Human output only; always exits `0`.

Each line distinguishes a missing source from an empty one: when the relevant
file (`docs/IDEAS.md`, `docs/architecture.md`) or directory
(`docs/rfcs`, `docs/adrs`) does not exist, the line reports
`not found (<path>)` instead of a misleading zero count.

Below the last line, `docent status` prints a `Violations by rule` breakdown
with a count per lint rule (rules with zero violations are omitted), sorted by
count descending.

### `docent list`

Prints one line per RFC or ADR document in the current project. Human output
only; always exits `0` unless the arguments are invalid.

```sh
docent list rfc                  # every RFC
docent list rfc --status draft   # only Draft RFCs
docent list adr --status accepted --implementation pending
```

The positional `rfc|adr` argument selects the document type. `-s, --status`
takes the type's status enum (case-insensitive; `--help` lists the possible
values): `Draft | Accepted | Rejected` for RFCs,
`Accepted | Superseded | Deprecated` for ADRs. `-i, --implementation` takes
`implemented | pending` and is only valid for ADRs.

Each row shows the document id, title, status, and its index date (`updated`
when present, otherwise `created`); ADR rows also show the implementation
state. The header line names the applied filters and the row count. Documents
whose front matter fails to parse are skipped — those are surfaced by
`docent lint` instead. A missing `docs/rfcs` / `docs/adrs` directory simply
yields an empty list.

## The rules

Each rule has a unique ID, a severity (`error` or `warning`), and an optional
`--fix` capability.

| Rule ID | What it detects | Severity | `--fix` |
|---|---|---|---|
| `frontmatter-schema-valid` | RFC / ADR front matter deviates from the required schema | error | — |
| `adr-missing-required-sections` | An ADR is missing a required section (Context / Decision / Non-goals / Consequences) | error | — |
| `adr-index-sync` | `docs/adrs/README.md` table disagrees with the ADR front matter | error | ✓ |
| `rfc-index-sync` | `docs/rfcs/README.md` table disagrees with the RFC front matter | error | ✓ |
| `rfc-accepted-outcome` | An Accepted RFC without a linked ADR lacks an explicit no-ADR reason | error | — |
| `superseded-backlink-consistency` | A `supersedes` reference has no matching `superseded_by` backlink | error | ✓ |
| `amendment-backlink-consistency` | A partial ADR amendment lacks a matching `amends`/`amended_by` relation or targets a non-Accepted ADR | error | — |
| `agents-adr-reference-valid` | `AGENTS.md` references an ADR whose status makes that reference invalid | error | — |
| `required-source-missing` | A required source (`docs/rfcs`, `docs/adrs`, `docs/architecture.md`, `AGENTS.md`) does not exist | warning | — |
| `architecture-module-sync` | A module table entry has no safe relative `Path`, or its declared file or directory does not exist | warning | — |
| `adr-pending-implementation-report` | An Accepted ADR still records `implementation: pending` | warning | — |
| `rfc-stale-draft` | An RFC has stayed in Draft past the staleness threshold | warning | — |
| `project-policy-version` | A Docent-managed project policy differs from the installed policy version | warning | — |

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
number when the location can be pinpointed, and `null` otherwise.

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
- `AGENTS.md` — references to ADRs

A missing source is reported explicitly instead of silently skipped: the
`required-source-missing` warning fires once per absent entry above
(`docs/rfcs`, `docs/adrs`, `docs/architecture.md`, `AGENTS.md`). `docs/IDEAS.md`
is only counted by `docent status`. `CONTEXT.md` remains terminology guidance
for people and Agents; Docent does not lint it.

New RFCs / ADRs are expected to be written from the templates in
`docs/.templates/`, which already carry a valid front matter skeleton.

PRD/issue storage (e.g. .scratch/) is intentionally outside Docent's checks — see the canonical specification's "PRD/issue and the governance chain" section for why.

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
