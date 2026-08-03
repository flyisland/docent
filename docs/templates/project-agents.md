# Core Technical Constraints for This Project (System Instructions for Code Generation)

Read `docs/README.md` before changing project documentation. Before creating
or editing any RFC, ADR, or `CONTEXT.md`, read its template in
`docs/.templates/`. After finishing, run `docent lint` until it reports no
errors.

## Architectural principles

- Before changing module topology, responsibility boundaries, or declared
  module paths, read `docs/architecture.md`. Resolve a conflict through the
  ADR lifecycle before changing the implementation.
- Prefer enforcing value ranges and state transitions in types, validation, or
  assertions rather than prose alone.
- Use the canonical terminology in the nearest applicable `CONTEXT.md`.

## Documentation maintenance

- Follow the policy in `docs/README.md`; keep implementation detail out of
  this file.
- A major or directional architectural-decision change needs a new ADR with
  supersession links. Historical ADR bodies are not erased. A permitted minor
  clarification or numeric change is appended as an Amendment when the
  project’s ADR convention allows it.
- Living module designs are overwritten to describe current intended behavior;
  use the placement rules and templates in `docs/README.md`.

## Rule maintenance

- Review and trim this file quarterly, removing rules superseded by current
  ADRs or no longer applicable.
