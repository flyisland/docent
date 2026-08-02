# Core Technical Constraints for This Project (System Instructions for Code Generation)

## Architectural principles

- Any change that adds/removes/merges/splits a module or adjusts
  responsibility boundaries between modules must first read
  docs/architecture.md to confirm it doesn't violate the existing module
  breakdown. If there's a conflict, go through the ADR process rather than
  editing the code directly.
- When adding any range-of-values or state-transition constraint, prefer
  enforcing it in code — via the type system, enums, validation, or
  assertions — rather than only describing it in a comment or a document.
  Only write a constraint into a module README once it's confirmed that it
  can't be enforced in code, or that the cost of enforcing it isn't
  worthwhile.
- When naming anything project-specific, first check whether a CONTEXT.md
  exists at the root or in the relevant module. If it does, you must use the
  canonical term defined there — using a synonym listed under Avoid, or
  inventing a new synonym, is not permitted.

## Documentation maintenance rules

- Strictly follow the documentation lifecycle defined in docs/README.md.
- When updating an architectural decision, only the module README may be edited — erasing the body of an ADR is prohibited.

## Rule maintenance

- Review and trim this file once a quarter, removing entries superseded by newer ADRs or no longer applicable.
