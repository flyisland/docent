# docent — source layout

`docent` is a single-purpose CLI binary. The crate has no library target; the
module layout is:

- `src/main.rs` — entry point, calls into `cli`.
- `src/cli.rs` — `clap` argument definitions (`init`, `lint`, `status`,
  `docs`).
- `src/commands/` — one file per subcommand; `lint.rs` executes rules and
  `docs.rs` owns compile-time bundled-spec rendering plus create-new export.
- `src/model/` — project discovery and parsing of the documents docent reads
  (ADR / RFC front matter, indexes, `docs/architecture.md`, `AGENTS.md`).
- `src/rules/` — one file per lint rule, plus `fix.rs` (the bounded `--fix`
  implementation) and `mod.rs` (rule registry / `run_all`).
- `src/output/` — `human.rs` (terminal output) and `json.rs` (the stable
  `--json` contract described in the implementation spec).

## Gotchas

- **`--json` exit code**: `lint --json` still exits `1` when errors are
  present, exactly like the human output. Consumers must read the exit code,
  not assume a successful parse implies a clean project.
- **`--fix` exit code**: `lint --fix` applies fixes, then reports the *post-fix*
  violations. It returns `0` only if no errors remain after fixing, so it is
  safe to run in a loop that repeats until exit `0`.
- **Regex `(?i)` case-insensitivity is ASCII-only**: multi-word terms are
  joined with `\s+`; non-ASCII terms or full-width punctuation (e.g. `：`)
  will not match the way you might expect. Keep `CONTEXT.md` terms plain ASCII.
- **`serde_yaml` 0.9 vs 0.8**: the front-matter schema uses serde `enum`
  variants, and unknown string variants produce the error
  `unknown variant '…', expected …` from the `serde_yaml` deserializer. Match
  against that message text rather than trying to introspect a parse error
  variant that differs between minor versions.
- **Index parsing is header-driven**: the index-sync rules locate table
  columns by their header cell text (`Title`, `Status`, `Last Updated`,
  `Date`, `Linked ADR`), never by fixed column position. Hand-edited index
  tables that reorder columns still work; a missing header cell yields a
  "malformed" violation.
- **`owo-colors` needs the `supports-colors` feature**: the human output uses
  `if_supports_color(Stream::Stdout, …)`; without the `supports-colors`
  feature enabled the `Stream` API is unavailable at compile time.
- **`id_from_rel` vs basename**: a violation `file` field carries a path
  relative to the project root, so rule code must extract the document id from
  the file *basename* (e.g. `adr-003-…`), not from the whole relative path.
