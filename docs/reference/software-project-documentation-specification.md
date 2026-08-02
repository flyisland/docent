---
description: 'Defines the full lifecycle of project documentation, from a vague idea to a finished implementation'
last_reviewed: ''
related: []
source_urls: []
status: growing
tags: []
templates:
- '[[evergreen]]'
type: evergreen
---
# Software Project Design and Documentation Management Specification

This specification defines the full lifecycle of project documentation, from a vague idea to a finished implementation. It draws a clear line between human and Agent responsibilities and provides actionable criteria and collaboration patterns.

- **Human responsibilities**: set overall direction, make decisions, define hard constraints, resolve conflicts.
- **Agent responsibilities**: assess feasibility against the codebase, refactor code per approved decisions, keep living documentation in sync, and stop and escalate when a conflict is detected.

---

## One-Page Overview: The Documentation Lifecycle

```
 [Idea in someone's head / vague notion]
     │
     ▼
 0. Idea capture (IDEAS.md)          ───> Jot it down, one-way funnel, never archived
     │
     ▼
 1. RFC stage (docs/rfcs/)          ───> Open discussion, Agent feasibility checks, option comparison (sealed once closed)
     │
     ▼
 2. ADR stage (docs/adrs/)          ───> Crystallizes into immutable law, captures the final constraint (amend like a constitution)
     │
     ▼
 3. System architecture overview (docs/architecture.md) ───> Global module map, ownership, dependencies (living doc, root level)
     │
     ▼
 4. Detailed design (module README / {name}.design.md)  ───> Internal implementation detail per module (living doc, module level)
     │
     ▼
 5. AI rules (AGENTS.md)            ───> The hardest, most load-bearing principles, distilled into a cross-tool guardrail for Agents
```

These are the six core stages, and each has a different update cadence: IDEAS.md is a one-way funnel that never accumulates; RFCs change; ADRs don't (only amendments are appended); the architecture overview changes with module topology; module READMEs / design.md files change with implementation; AGENTS.md states principles only, never implementation detail.

Stage 4 can optionally hang an operational runbook (RUNBOOK.md) off it, for accumulating operational know-how over time. This isn't a seventh layer — it's an optional artifact that Stage 4 may need once a project enters long-term maintenance. See the end of Stage 4 for details.

There's also a lightweight companion mechanism — a project terminology consistency file (CONTEXT.md) — used to standardize how key nouns are named across a project. It also doesn't count as one of the six core stages, and isn't a "decision" or "implementation" document belonging to any single layer; it's an independent thread that cuts across Stages 3 and 4. See the supplementary section after Stage 3 for details.

---

## Core Principles

The following two principles run through every layer. Later sections don't re-argue them — they simply point back here with "see Core Principles."

**1. Write only what the next layer, or the code itself, cannot answer.** Content that a downstream document naturally carries, or that the code itself already expresses, should not be repeated at the current layer. The default action when writing documentation should be "write less" — every additional sentence needs a justification, not the other way around.

**2. Each piece of information has exactly one source of truth; downstream documents cite the upstream conclusion instead of re-arguing it.** For example, when a design document mentions a technology choice, it should say "see adr-XXX for rationale" rather than repeating the discussion that happened in the ADR or RFC. A simple test for whether a passage violates this: delete the upstream document — does the downstream passage still stand on its own? If it doesn't (it depends on an argument that isn't restated), the reference is appropriate. If it still stands (it has silently re-derived the same argument), that's unnecessary duplication.

These two principles also apply to this specification document itself — if the same point is found fully restated across multiple stages, it should be distilled into a Core Principle here, with the rest of the document reduced to a brief pointer.

---

## Stage 0: Idea Capture (IDEAS.md) — The First Stop for Inspiration

- **Purpose**: raw, half-formed thoughts — a one-liner, a code fragment, a fuzzy direction — that haven't yet been thought through.
- **Core rule**: a one-way funnel, not a knowledge base and not an archive. Every idea stays here only until it's been thought through enough to know what to do with it, then it must leave. It must not accumulate into a second, unmanaged pile of documentation.
- **Location**: `IDEAS.md` at the project root, free-form — write it however is convenient.

This stage is deliberately lightweight and gate-free, because it mirrors a real work habit: "write it down now, don't break your train of thought." It doesn't need to be formalized up front. Formalization happens at the next step — when an idea is claimed and its destination decided.

### Claiming rule: decide where an idea goes before it leaves IDEAS.md

Run the same test used in Stage 2's "ADR change criteria" in reverse (the two sections share one set of criteria rather than maintaining two copies — see that section). This sorts an idea into roughly four destinations:

|Nature of the idea|Destination|
|---|---|
|Changes an external interface / data contract, adds a new dependency, or affects multiple modules|Needs an RFC — go through RFC → ADR|
|Purely an implementation detail or bug fix|Change the code directly, and update the relevant module's README or `.design.md` in passing|
|Scope still unclear, can't tell which category it belongs to|Keep it in IDEAS.md, tagged "needs refinement" — clarify scope next time it's claimed|
|Actually already-known knowledge or an already-resolved pitfall, not a "should we do this" question|Not a TODO — move it straight to the relevant module's Gotchas section, don't wait for a cleanup pass|

When claiming an idea, add a one-line destination note right after it for traceability, e.g.:

```markdown
- Axis-Aligned Connector...
  → Needs an RFC (new schema field)
```

Category four deserves special attention: if something like "a specific gotcha with a specific library" ends up in IDEAS.md, the biggest risk is that after a while everyone forgets it was already resolved and mistakes it for an open TODO. The test is simple: if the item isn't a "should we do this" question but an "we already know this" statement, it doesn't belong as a TODO — move it the moment you notice, no need to wait for the next general cleanup.

### Handling rules

- **Once claimed, delete it from IDEAS.md** — don't mark it "done" and leave it there. Its destination is an RFC file, a direct code change, or the relevant module's `.design.md`/README. IDEAS.md itself is never responsible for archiving.
- **Sweep it periodically** (e.g., at the start of each new development cycle): process what can be processed, push for clarification on anything vague, and delete anything long unclaimed and no longer relevant. Deleting loses nothing of value — anything genuinely worth keeping was already converted into an RFC or ADR at claim time and lives there. IDEAS.md was never a formal document and requires no archival process.

One-line summary: **IDEAS.md holds only "raw thoughts not yet thought through." The moment any entry's path becomes clear, it must leave this file immediately.**

---

## Stage 1: RFC (Request for Comments) — Exploration and Brainstorming

- **Purpose**: a living discussion proposal that records which options were considered and why they were dropped.
- **Location**: `docs/rfcs/rfc-XXX-[name].md`

### Status values

|Status|Meaning|
|---|---|
|Draft|Under discussion, no decision yet|
|Accepted|Adopted, converted into an ADR|
|Rejected|Not adopted, archived for the record|

An RFC only closes as "accepted" or "rejected" — no intermediate state is needed.

### Content criteria: separate strategic rejections from tactical ones

An RFC only records rejected options at the **strategic level** — changes to an external interface / data contract, or the introduction / replacement of a core dependency — for example, "why we didn't use distributed transactions and used a local outbox table instead." This kind of discussion is worth preserving in full, because it shapes the boundary every subsequent implementation has to respect.

**Tactical-level trade-offs** made during implementation (e.g., "why we used CDC on the binlog instead of polling the table on a timer") should not be backfilled into the RFC. They belong in the relevant module's detailed design document (Stage 4) instead. The reason is simple: almost nobody revisits an RFC once it's finalized, so information like this ages far better living close to the code — stuffed into an RFC, it just becomes a dead archive nobody maintains.

The test: **would this trade-off ever be reopened by a future RFC?** If yes (architectural, cross-team), keep it in the RFC. If no — it's just an internal implementation choice for one module — put it in the design document instead.

### Human-Agent collaboration flow

The human writes the pain point and rough proposal (Option A / Option B) in plain language in the RFC file, then:

> @Codebase Here's a new idea (RFC attached). Given the current codebase, which of Option A and Option B requires the least change to the existing architecture? Are there any technical risks I haven't considered?

Agent's job: read the codebase, fill in the "technical feasibility analysis" section of the RFC, list potential risks, and help the human decide.

### RFC index

Maintain a summary table in `docs/rfcs/README.md` so RFCs remain discoverable after they're archived:

|ID|Title|Status|Date|Linked ADR|
|---|---|---|---|---|
|rfc-001|Cache strategy selection|Accepted|2026-01|adr-005|
|rfc-002|Payment retry policy|Rejected|2026-02|—|

---

## Stage 2: ADR (Architecture Decision Record) — The Architectural History

- **Purpose**: the static decision record — the project's highest law.
- **Core rule**: immutability. Once accepted and implemented, the core conclusion in the body text cannot be rewritten directly.
- **Location**: `docs/adrs/adr-XXX-[name].md`

### Status values

|Status|Meaning|
|---|---|
|Accepted|In effect, amendments may be appended|
|Superseded|Replaced by a newer ADR, kept for historical reference only|
|Deprecated|The decision is no longer valid but has no replacement (e.g., the related feature was retired)|

### Implementation status field

An ADR records that "the decision has been made," not that "the code has landed" — the two need to be tracked separately. To avoid splintering the primary status field, add a separate implementation status field at the top of the file:

```markdown
Status: Accepted
Implementation: Implemented (payment/service.py)
```

When not yet implemented:

```markdown
Status: Accepted
Implementation: Pending
```

Keep it to two values — "Implemented" / "Pending." Finer-grained progress (in development, in testing) belongs in issues/PRs, not in the ADR.

Benefits:

- When an Agent looks up a constraint, it can tell whether the rule is on paper only or already live in the code, avoiding a mistaken read of current state.
- Audits can specifically surface ADRs that are "Accepted but not yet implemented" as a prompt to push them to completion.

### Standard ADR body structure

Many teams write ADRs with wildly inconsistent depth — some are a single-sentence conclusion, others copy the entire RFC discussion verbatim. To avoid this polarization, the body always uses the following four fixed sections:

```markdown
## Context
Don't just state the problem being solved — also record the organizational /
resource constraints at the time of the decision: team size, existing
infrastructure, time pressure, etc. This contextual information is completely
invisible in the code, yet it directly determines whether the decision was
reasonable *at the time*. Recording it prevents later readers (including AI)
from judging "was this design bad" using today's resources and hindsight,
and making an unnecessary reversal as a result.

## Decision
State what was ultimately chosen, in one sentence. Don't repeat the discussion
— just cite the source (see "Core Principles").

## Non-goals
What this decision explicitly does not cover, and what should not be scoped
in later just because it's convenient. This is the section most often
skipped, yet it's the single most important piece for preventing scope
creep — without it, a future refactor (human or AI) will happily reason
"well, the infrastructure is already there" and pull in things that were
never in scope.

## Consequences
- Positive impact: the concrete benefit this decision produces.
- Known costs: negative effects that were knowingly accepted as a trade-off
  — not defects. These must be stated explicitly, so they aren't later
  mistaken for "a bug to be fixed" — this kind of cost is usually a
  deliberate compromise, not unfinished work.
```

**Why there's no "can this be enforced by code" step here, unlike Stage 4**: an ADR records macro-level architectural decisions (which middleware to use, which consistency model to adopt) — conclusions that can't be enforced by a type system or validation rule, and can only live as prose. This filter only makes sense for the implementation-level constraints in Stage 4 that can be pushed down into specific fields or interfaces.

### Change criteria

Any one of the following requires a new ADR, with the old one marked Superseded:

1. Changes an external interface / data contract (fields, protocol, calling convention).
2. Reverses or overturns the core conclusion of the original decision (e.g., switching from "use a cache" to "don't use a cache").
3. Introduces or replaces a core dependency / piece of middleware (e.g., Redis, a message queue).
4. Affects multiple modules and requires cross-team realignment.
5. Adds, removes, merges, or splits modules, or adjusts responsibility boundaries between modules (see the update triggers in Stage 3).

Only the following requires an amendment (appended at the end) instead:

1. Adjusting a numeric parameter (timeout, cache TTL, retry count, etc.).
2. Fixing a typo or adding clarifying prose, without changing the decision itself.
3. Adding scope notes to the decision without overturning the original conclusion.

When in doubt, default to creating a new ADR — the cost of one extra document is far lower than the cost of a historical decision being quietly rewritten.

This same set of criteria is also what Stage 0 uses in reverse to decide whether an idea needs an RFC; it's defined once here and only referenced from Stage 0, not duplicated.

### Change handling procedure

Major / directional changes:

1. Create `adr-YYY.md`.
2. Change the status of the original `adr-XXX.md` to `Superseded`.
3. Add bidirectional links at the top of both files (e.g., "This decision has been superseded by adr-YYY").

Minor, non-core tweaks:

1. Keep the original status as `Accepted`.
2. Add a marker next to the affected paragraph (e.g., "Note: parameter updated, see Amendment 1 at the end of this document").
3. Append an `## Amendments` section at the end, recording the date, the new parameter, and the reason for the change.

### Human-Agent collaboration flow

> We just changed the cache duration to 300 seconds. Go to the end of docs/adrs/adr-005.md and append an Amendment recording today's date and the reason for the change. Don't modify the original body text.

### ADR index

Maintain a summary table in `docs/adrs/README.md`:

|ID|Title|Status|Implementation|Last Updated|
|---|---|---|---|---|
|adr-005|Local cache selection|Accepted|Implemented|2026-07|
|adr-008|Payment state machine design|Accepted|Pending|2026-05|

---

## Stage 3: System Architecture Overview — The Global Module Map

- **Purpose**: which modules the system is currently divided into, what each one is responsible for, and how they collaborate. Answers "what does the system look like," not "how is a given module implemented internally."
- **Core rule**: a living document, but with a much slower update cadence than a module README — update it only when module topology changes (added / removed / merged / split / responsibility changes). Internal implementation adjustments inside a module never touch this document.
- **Location**: `docs/architecture.md` (or `ARCHITECTURE.md`) at the project root.

This layer fills the gap between the ADR and the module README: the ADR records *why* the modules are divided this way (a decision, immutable); the architecture overview records *how they're actually divided right now* (current state, living document, root level); the module README records *how a given module is actually implemented* (current state, living document, module level).

### Content structure

```markdown
# System Architecture Overview

## Module breakdown
| Module | Responsibility | Detailed design | Linked ADR |
|---|---|---|---|
| payment | Payment flow and state machine | payment/README.md | adr-008 |
| auth | Authentication and session management | auth/README.md | adr-003 |
| notification | In-app messages and push notifications | notification/README.md | adr-011 |

## Inter-module dependencies
(Dependency graph — who calls whom, whether reverse dependencies are allowed)

## Boundary principles
(Which interactions must go through an interface layer; cross-module access to internal data structures is prohibited, etc.)
```

It's essentially a table of contents and map for all the module READMEs. It doesn't repeat internal module detail — its only job is to let a human or Agent build a mental picture of the whole system in a short amount of time.

### Update triggers

- **Must update**: a module is added, removed, merged, split, or its responsibility boundary changes. This kind of change already satisfies Stage 2's "change criteria," so **there must be a corresponding ADR explaining the rationale first, and the architecture overview updates as an overwrite afterward** — you cannot skip the ADR and edit the overview directly.
- **Should not update**: internal adjustments within a single module, however large, as long as its external responsibility boundary hasn't changed — only the module README changes, not this document. This boundary keeps the overview from being disturbed by every internal refactor, preserving its "glance-and-understand-the-whole-system" value.

### Human-Agent collaboration flow

> We split the notification module into notification-core and notification-channel. First draft an ADR explaining the rationale for the split; once I confirm it, update the module table and dependency graph in docs/architecture.md, and create matching READMEs for the two new modules.

---

## Supplementary Mechanism: Project Terminology Consistency (CONTEXT.md)

- **Purpose**: standardize the naming of key nouns across a project, preventing the same concept from being called different things in different places (e.g., `Order` in one place, `Purchase` in another), and preventing the same word from quietly denoting different concepts in different modules.
- **Core rule**: naming consistency cannot be enforced by a type system — it can only be pinned down by a document that "makes a decision and lists the banned synonyms." This is a canonical case where documentation earns its keep and code can't self-explain (see "Core Principles") — looking at a handful of variable names in isolation never reveals that "the team decided on this word and rejected those others" as a deliberate decision.
- **Location**: for a project with a single semantic boundary, a single root-level `CONTEXT.md` is enough. For a project with clearly divided modules that each have their own vocabulary, maintain a `{module}/CONTEXT.md` per module.
- **Do not maintain a separate cross-module relationship map**: CONTEXT.md is only responsible for "what we call things." Relationships between modules (who publishes events to whom, what types are shared) already belong in Stage 3's "inter-module dependencies" section — don't build a separate CONTEXT-MAP and end up maintaining the same relationship graph in two places that drift out of sync.

### Format

```markdown
# {Module / project name}

{One or two sentences describing what this semantic boundary is and why it exists}

## Language

**Order**:
The customer's purchase request, covering everything from placement through
pre-shipment.
_Avoid_: Purchase, Transaction

**Invoice**:
The document sent to a customer after shipment, requesting payment.
_Avoid_: Bill, Payment Request
```

### Inclusion criteria

- **Only include terms specific to this project / module — not general programming concepts.** Timeouts, error types, common design patterns — even if heavily used in the project — don't belong here. They mean the same thing in any codebase and need no project-specific explanation. This is the same test as Stage 4's "can it be derived from the code": general concepts can be understood from common sense; project-specific terms are where documentation is actually earning its keep.
- **Be opinionated**: when a concept has multiple names in use, pick exactly one as the canonical form and list the rest under `_Avoid_`. This isn't a "discussion aid" — it's a decision the team has already made. New code is not permitted to use a term listed as Avoid.
- **Keep definitions short**: one or two sentences explaining what the term *is*, not how it's used (usage belongs in the Stage 4 module design document).
- **Create lazily, don't pre-build scaffolding**: a project doesn't need to be forced to write a glossary at kickoff. Create or extend this file the first time naming confusion actually appears and a term needs to be settled — consistent with IDEAS.md's "archive only once it's clear" philosophy of staying lightweight.

### Human-Agent collaboration flow

> I noticed `Order` and `Purchase` are used interchangeably in the code and seem to refer to the same concept. Search the codebase for how each is used and confirm whether they're semantically identical. If they are, help me set `Order` as the canonical term and list `Purchase` under Avoid in CONTEXT.md, then list every location that needs renaming — I'll confirm before you apply the changes uniformly.

> From now on, before any naming change, read `{module}/CONTEXT.md` first (if it exists) — don't invent a new synonym on your own.

---

## Stage 4: Detailed Design — The Living Manual for the Code

- **Purpose**: concrete interface contracts, database schemas, sequence diagrams, pseudocode. Answers "how is a given module actually implemented internally."
- **Core rule**: an absolutely living document. Overwrite and edit the body directly as the code changes, keeping it 100% in sync. Historical changes are left to Git — don't keep old versions inside the document.
- **Location**: as close to the code as possible, e.g., `src/modules/payment/README.md` inside the module directory.

### Content criteria: can it be carried by code, and can it be derived from code?

This is the layer of documentation most easily misused in the age of AI coding — write it too fine-grained and you've just translated the code into prose (an Agent reading the code directly will always have fresher, more accurate information than the document); write it too shallow and you lose the information that genuinely only comes from human experience.

**Before getting into the criteria, one important blind spot needs to be named first**: the criteria below answer "can this information be obtained from the code," not "is the code itself correct." If a piece of information can **both be derived from the code and only exists in the code**, then verifying "does the code correctly implement this" becomes circular — you're using the code to prove the code, which is as unconvincing as someone asserting "everything I say is true" as their own proof. Any judgment of correctness needs an anchor independent of the implementation: something that expresses "what should be" ahead of time and independently, used to check "what actually is."

That anchor shouldn't be a static document fighting code drift (that just reintroduces the old "doc and code out of sync" problem) — it should be a **test**. A test and an implementation are two separate artifacts, one expressing "expected," the other expressing "actual"; agreement between the two, when run, is what verification means. This is really the same "prefer code enforcement" logic extended into another dimension: structural constraints (which values a field can take) are enforced by the type system; behavioral correctness (whether a given input should produce a given output) is enforced by tests.

There's a trap worth watching for specifically in the age of AI coding: if the same Agent, in the same task, writes both the implementation and the test based on its own understanding of the requirement, the two will share the same misunderstanding — if the requirement was misread, the implementation and the test will be "consistently wrong together," passing every check while having drifted from the actual requirement, without ever truly breaking the circularity. **Acceptance criteria / expected behavior must be settled before implementation begins** — even if it's just a few bullet points — so it serves as the shared target both the implementation and the test are checked against, rather than something summarized retroactively after the implementation is written (this maps to the fifth content category added below).

The decision of whether a constraint belongs in the document is a two-step filter:

**Step 1: can this constraint be enforced directly by code, rather than merely described?**

A constraint written only in prose is a gentleman's agreement — people forget, and an Agent may violate it unknowingly during some future refactor, with no mechanism guaranteeing the document and the code stay in sync. A constraint enforced by code cannot be violated: a type system, an enum, a validator, or an assertion will fail to compile or throw at runtime the moment it's broken — nobody needs to have read a document first to know "you can't do this." So: **wherever code enforcement is possible and the cost of enforcing it is reasonable, prefer code enforcement over falling back to documentation.**

> Example: if a field can only take one of three string values, don't just write "this field must be A, B, or C — do not pass anything else" in the documentation. Use an enum, a union type, or a database CHECK constraint to enforce the range directly in code, so anything out of range fails immediately. Once that's done, the constraint doesn't even need to appear in the design document — it's self-explanatory to both Agents and humans (read the type definition and you know), and it cannot be violated.

This principle has a necessary cost boundary: not every constraint is worth building a dedicated enforcement mechanism for. If enforcing it in code requires pulling in an extra framework or adds ongoing operational burden, and the constraint itself has a low probability and low cost of being violated, a single line of documentation is the better trade. The test is **whether the enforcement cost is clearly lower than the risk of documentation drift causing the constraint to be violated** — not "add validation logic to the code the moment any constraint appears."

Conversely, this is also the signal for identifying which invariants genuinely must be written down: **if an invariant currently can only be described in documentation and cannot be enforced by code** (e.g., "these two fields must be written in the same database transaction," a cross-table or even cross-service constraint that typically exceeds what a single type system can express), that's exactly where documentation earns its keep — and it's also a hint that this may be technical debt: if there's ever a way to fold it into a code-level constraint, that should be prioritized over continuing to rely on a document to remind people.

**Step 2: for constraints that can't be enforced by code and can only live in documentation, run them through the "can an AI derive this by reading the code" test:**

> If you handed the entire module's code to a sufficiently capable Agent, could it work out this conclusion on its own?

- **If it can be derived, don't write it down**: how a given function is implemented, what type a given field is, which layers a call chain passes through — leave these to the code itself. Writing them in the document is redundant work that will inevitably drift, and it creates confusion about which one to trust when the two disagree.
- **If it can't be derived, it's worth writing**:
  - **Invariants**: the difference between "the code just happens to be written this way" and "the code must be written this way — changing it causes serious problems" isn't visible from reading the code alone, and it can't be enforced by the type system, so it must be stated explicitly. Example: "writes to `order_outbox` must occur in the same database transaction as the related business change; cross-database deployment is prohibited."
  - **Implicit contracts with external dependencies**: real-world behavior discovered only through integration testing or hard-won experience, that the other side's API docs never mention. Example: "if the SMS service times out without responding, there's still roughly a 15% chance the message is delivered asynchronously anyway — consumers must be idempotent."
  - **Quantitative rationale behind a non-functional target**: a parameter or capacity value that wasn't picked arbitrarily — record which metric it was derived from, so future maintainers know whether it's safe to change. Example: "the Kafka topic is set to 12 partitions, calculated from an estimated peak of 5,000 TPS at 500 TPS per partition."
  - **Scope & Non-goals**: what this module / piece of implementation deliberately does not do, to prevent it from being casually extended later. Example: "this async pipeline serves eventual-consistency use cases only — strong-consistency operations like inventory deduction must never be routed through it."
  - **Acceptance criteria / expected behavior**: concrete "given input X, should produce Y" statements, or the decision logic behind a key business rule. This category inherently satisfies "cannot be derived from code" — "what should be" can never be reverse-derived from "what actually is"; that would be circular by definition. Once written down, it should be turned into an automated test as quickly as possible; the document should retain only *why the rule exists* (the business rationale), while the concrete input/output expectations live in the test itself — don't maintain a separate list of expected values in the document that will inevitably drift from the tests.

For correctness judgments that genuinely can't be automated (e.g., whether an interaction feels smooth, or whether an architectural decision truly matches business intent), there's no code-based anchor available — fall back to the mechanism defined later in "Understanding Checkpoints." Have a human genuinely understand and confirm it, rather than letting an Agent define the requirement, implement it, and verify it all by itself, acting as both player and referee.

This same standard also applies to deciding how much detail belongs in Gotchas — record only "why the problem happened, where the boundary is, how to prevent it at the code level"; there's no need to reconstruct the fix line by line, Git log/PR history already covers that. Also, if a Gotcha entry describes a fix that relies on "a convention" or "a comment as a reminder," ask whether it could instead become a type constraint or a runtime check that eliminates the Gotcha entirely, rather than leaving the team to avoid the pitfall from memory.

### Human-Agent collaboration flow (bidirectional sync loop)

Loop A (document-driven): a human directly edits an interface field in the module README.md.

> I've updated the latest interface spec in payment/README.md. Review and refactor payment/service.py, find every piece of code that's now inconsistent, and make it match the new document exactly.

Loop B (code-driven): an Agent changes some internal logic while refactoring.

> That refactor looks good. Now update the README.md in this directory so the sequence description and data structures fully match the code you just wrote. Before writing, ask yourself: can this content be enforced by code instead of documented? For anything that can't be enforced, ask whether it can be derived by reading the code. If it can be derived, don't write it — only write invariants, implicit contracts, quantitative rationale, and Non-goals: the things the code itself can't answer.

### Gotchas section

Every module README keeps a fixed section at the end for brief warnings tied to significant production bug fixes:

```markdown
## Gotchas
- 2026-06: A race condition on the status field caused corrupted concurrent
  state because it wasn't locked. Fixed via the State state-machine class.
  Never modify the status field directly at the Controller layer.
```

Gotchas records only "how to prevent this at the code level" — the list should stay short and stable.

### Runbook (RUNBOOK.md): low priority, expand only as needed

If a module's operational troubleshooting knowledge (how to diagnose an incident, how to stop the bleeding, what the monitoring thresholds are) starts accumulating continuously and updating frequently, it can in principle be split into an independent `RUNBOOK.md`, kept separate from the more stable Gotchas — Gotchas answers "how should the code change," a runbook answers "how should a person respond."

This isn't the current focus of this specification, though: most projects haven't yet reached the long-term maintenance phase that needs a dedicated operations function with continuously accumulating failure patterns. Forcing a RUNBOOK.md split at this stage just produces an empty document nobody maintains. **The default is to record operational knowledge in Gotchas too, without distinguishing it; only split it out once a module's Gotchas section is visibly dominated by operational entries and it's starting to slow down the search for design constraints.** That's the point to come back and flesh out this section's specifics (content structure, ownership for updates, etc.).

### Sub-layering within a module: when a module has multiple parallel mechanisms

The "overview separated from detail" principle doesn't only apply at the "project vs. module" layer — it's recursive. At the project level there's a system architecture overview (the global module map) plus module READMEs (per-module detail). When a module itself contains multiple parallel mechanisms with clear responsibility boundaries that evolve independently, that module should apply the same structure to itself, rather than cramming every detail into one README.

Typical scenario: a `layout` module containing `basic-layout.ts`, `flex-layout.ts`, `grid-layout.ts`, `stack-layout.ts`, `taffy-layout.ts` — multiple layout strategies exposed through a shared interface but with completely independent internal algorithms.

**Directory structure**: put the design doc right next to its implementation file, named `{name}.design.md`, rather than collecting them under a single `docs/` subdirectory:

```
layout/
├── README.md                    ← module-level overview
├── basic-layout.ts
├── basic-layout.design.md
├── flex-layout.ts
├── flex-layout.design.md
├── grid-layout.ts
├── grid-layout.design.md
├── stack-layout.ts
├── stack-layout.design.md
├── taffy-layout.ts
├── taffy-layout.design.md
├── geometry.ts                  ← no standalone doc, cross-cutting infrastructure
├── intrinsic-size.ts
├── positioning.ts
└── layout-resolution-error.ts
```

This way, when an Agent edits `flex-layout.ts`, it can locate `flex-layout.design.md` purely from the filename, without needing to learn a separate directory-mapping convention first.

**The module README's role is upgraded accordingly**: it no longer carries every implementation detail — it becomes a miniature architecture overview scoped to this one module:

```markdown
# layout module overview

## Responsibility
Accepts layout constraints in format XXX, produces a unified LayoutResult.

## Layout strategies
| Strategy | Use case | Design doc |
|---|---|---|
| basic-layout | Simple absolute positioning | basic-layout.design.md |
| flex-layout | Flexbox semantic layout | flex-layout.design.md |
| grid-layout | Grid semantic layout | grid-layout.design.md |
| stack-layout | Z-axis stacking | stack-layout.design.md |
| taffy-layout | Layout via the Taffy engine | taffy-layout.design.md |

## Shared infrastructure
geometry.ts / intrinsic-size.ts / positioning.ts are reused by every strategy.
Confirm changes to these files won't break any strategy's assumptions before
editing (see the boundary notes in each strategy's own doc).

## Gotchas (cross-strategy issues)
- ...
```

A pitfall specific to a single strategy goes in that strategy's own `.design.md`; a pitfall in shared infrastructure that spans strategies stays in the module overview.

**Criteria**: not every module needs to be split this finely — avoid swinging to the opposite extreme of over-fragmentation.

Signals it's worth splitting (any one of these):

- The module has multiple strategies or algorithms exposed through a shared interface but with independent implementations.
- A single implementation is complex enough on its own (data structures, algorithm steps, edge cases) that it either gets written too shallowly if crammed into a shared doc, or bloats the overview if written in full.
- Different implementations evolve at clearly different rates, so a merged document means even a small change to one requires re-reviewing the whole thing — increasing the odds of a missed edit or a missed review.

Cases where it shouldn't be split:

- Plain utility functions or type definition files — the overview or inline code comments are enough; a standalone doc isn't warranted.
- A module has many files, but they're actually sequential stages of a single flow (not parallel strategies) — splitting them loses the flow context; describe the sequence in the overview instead.

---

## Stage 5: Codifying AI Rules (AGENTS.md) — The Last Line of Defense

- **Purpose**: distill the hard architectural constraints from ADRs into principles, forming a unified standard for multiple Agents working on the codebase.
- **Location**: `AGENTS.md` at the project root (**the filename must be strictly uppercase** — this isn't a stylistic preference, it's the fixed filename of the [agents.md](https://agents.md/) cross-tool open standard. Major coding agents — OpenAI Codex, Google Jules, Cursor, and others — automatically look for this file at the project root using this exact casing. A lowercase `agents.md` will simply go unread by those tools). Larger projects can place a local `AGENTS.md` in a subdirectory to cover supplementary rules scoped to that directory only, keeping the root file from growing without bound.

### AGENTS.md states principles only, never implementation detail

AGENTS.md and module READMEs are the pair most prone to "the same thing described in two places, drifting apart over time." To avoid that conflict:

- AGENTS.md states only "what the principle is and where to look for the detail" — it never copies class names, parameter values, or library versions.
- Implementation detail is maintained in exactly one place: the module README.

Bad example (don't do this):

```markdown
- [See ADR-008] All local in-memory caching must globally use Python's cachetools.TTLCache.
```

Good example (do this):

```markdown
- [See ADR-008] Local cache implementation: see payment/README.md. Do not bypass that document to pick your own implementation.
- [See ADR-008] Introducing external middleware such as Redis is prohibited unless a newer ADR authorizes it.
```

### Sample AGENTS.md template

The content in the next section, "Read-Only Directive for the Coding Agent," is designed to be pasted directly as the fixed opening block of AGENTS.md — the two aren't maintained as separate documents. AGENTS.md = the directive below + a project's own specific principles appended underneath it.

```markdown
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
- [See ADR-008] Local cache implementation: see payment/README.md. Do not bypass that document to pick your own implementation.
- [See ADR-008] Introducing external middleware such as Redis is prohibited unless a newer ADR authorizes it.
- [See Detailed Design — Payment] Payment state transitions must go through the State state-machine class. Hardcoding a change to the status field at the Controller layer is prohibited.

## Documentation maintenance rules
- Strictly follow the documentation lifecycle defined in docs/README.md.
- When updating an architectural decision, only the module README may be edited — erasing the body of an ADR is prohibited.

## Rule maintenance
- Review and trim this file once a quarter, removing entries superseded by newer ADRs or no longer applicable.
```

---

## Read-Only Directive for the Coding Agent (For Agent Input)

This block is the fixed opening section of AGENTS.md — it isn't maintained as a separate file. Paste it at the top of the project's AGENTS.md, followed by the project's own specific principles.

```
[System Instruction for Agent]

When a human asks you to modify the project's architecture, change an interface, or edit core configuration, you must:

1. Read the constraints first: proactively check the root AGENTS.md, any
   local AGENTS.md in the current directory or a parent directory,
   docs/architecture.md (if the change crosses module boundaries), and the
   README.md in the directory you're working in. Do not write code that
   violates existing design principles. Before modifying
   `{module}/{specific-file}.ts`, check whether a `{specific-file}.design.md`
   exists in the same directory. If it does, treat it as authoritative —
   don't implement based solely on the high-level description in the module
   overview README.

2. Handle conflicts: if a new requirement conflicts with an existing ADR,
   the architecture overview, or constraints in a README, stop coding
   immediately. Clearly explain the conflict to the human (citing the
   specific ADR number or document section) and suggest opening an RFC to
   revisit the discussion. Deciding on your own that "this case is a
   special exception" and proceeding anyway is prohibited.

3. Treat documents differently depending on type:
   - To update an ADR: only append Amendments at the end, or establish a
     Superseded bidirectional link as directed by the human. Erasing
     historical body text directly is prohibited.
   - To update docs/architecture.md: update it only when module topology or
     inter-module responsibility boundaries have actually changed, and only
     after a corresponding ADR exists — never skip the ADR and edit the
     overview directly.
   - To update a detailed design / module README: edit and overwrite the
     body directly, ensuring the document matches the latest code you
     generated 100%. Before writing, ask yourself two questions: (1) Can
     this constraint be enforced in code — via the type system, enums, or
     validation — rather than relying on documentation? If it can be
     enforced at reasonable cost, prefer changing the code over writing the
     document. (2) For anything that genuinely can't be enforced in code,
     or where enforcement isn't worth the cost, ask whether it can be
     derived by reading the code. If it can, don't write it — write only
     invariants, implicit contracts, quantitative rationale, and Non-goals:
     the things the code itself can't answer.

4. Acceptance criteria come first: before implementing anything involving
   specific business logic or rule-based decisions, check whether clear
   acceptance criteria already exist (expected inputs/outputs, key decision
   conditions). If not, confirm with the human first — don't infer the
   requirement, write the implementation, and write the tests all by
   yourself. All three would stem from the same understanding; tests
   passing doesn't mean the understanding was correct, only that the
   implementation and the tests are internally consistent with each other.
   Once confirmed, acceptance criteria should be turned into automated
   tests as soon as possible — don't leave them living only in chat history
   or comments.

5. Preserve pitfall history: if a code change addresses a significant
   production bug, append a brief warning to the module's README.md under
   ## Gotchas, to prevent the same mistake from recurring.

6. Maintain indexes: when creating or archiving an RFC / ADR, update the
   index table in the corresponding directory's README.md, and keep the
   implementation status field in the ADR header accurate.
```

---

## Rollout Checklist

Split into two groups by whether the check can be automated. When wiring this into CI, the first group can be converted directly into scripted rules; the second is a prompt list for periodic manual review.

### Automatable checks (suitable as CI scripts)

- Are the index tables in `docs/rfcs/README.md` and `docs/adrs/README.md` in sync with the actual files?
- Do the ADR numbers referenced in AGENTS.md still have status Accepted (not Superseded)?
- Does the module table in docs/architecture.md match the actual code directory structure? Are there new modules that haven't been recorded?
- Are there any ADRs that have been stuck at "Accepted but not yet implemented" for a long time and need to be pushed to completion or re-evaluated?
- Does every ADR body include Non-goals and known costs (the negative part of Consequences), rather than being a one-sentence conclusion?
- Does each module's `{name}.design.md` still match its corresponding `{name}.ts` implementation? Does the strategy/mechanism list in the module overview README match the actual files one-to-one?
- Does any code in the codebase use a word listed under `_Avoid_` in a CONTEXT.md, indicating the naming convention wasn't followed?

### Manual review (periodic audit items)

- Is the same detail duplicated between AGENTS.md and a module README, and has it already drifted out of sync?
- Does each module README's interface description still match the current code?
- Has a module README accumulated large chunks of implementation detail that could be derived directly from the code (should be trimmed down to invariants / contracts / rationale / boundaries)?
- Among the "invariants" recorded in module READMEs, is there anything that could reasonably be enforced by the type system / enums / validation at a reasonable cost, but is still only described in prose? Is it worth converting to a code-level constraint and deleting the doc entry?
- Has an RFC accumulated tactical implementation detail that should have been pushed down to the design document, leaving the RFC unmaintained once finalized?
- Can every module-topology change in docs/architecture.md be traced back to a corresponding ADR?
- Has AGENTS.md grown too large, and does it need a quarterly trim or a split into local rule files?
- Has a module's Gotchas section become visibly dominated by operational entries (incident diagnosis, mitigation steps), signaling it's time to split off a RUNBOOK.md (see Stage 4)?
- Is there an obvious synonym mix-up in the codebase (e.g., Order/Purchase used interchangeably) that hasn't yet been captured in the relevant CONTEXT.md?
- For implementations involving specific business rules, were the tests written retroactively in the same task as the implementation (sharing the same potential misunderstanding), or did independent acceptance criteria / tests exist ahead of the implementation?
- Has anything already-resolved knowledge or pitfall crept into IDEAS.md (should be moved to the relevant module's Gotchas), or are there claimed-but-not-deleted stale entries?
- Does this specification document itself have the same principle fully restated in multiple places, and does it need to be distilled into the "Core Principles" section?

---

## Structural Recommendations for Automated Management

For the specification above to be reliably parseable by scripts or CI, the document format itself needs some structural constraints — it can't be entirely free-form Markdown. Below are a few directions.

### 1. Unified front matter metadata

Use YAML front matter at the top of every RFC / ADR file, separating structured fields (status, dates, relationships) from the body text, so scripts can parse them directly without regex-guessing at heading formats:

```yaml
---
id: adr-008
title: Payment state machine design
status: Accepted
implementation: pending   # implemented | pending
created: 2026-05-10
updated: 2026-07-20
supersedes: []
superseded_by: null
related_rfc: rfc-003
---
```

Field names and value ranges should be fixed (e.g., `status` can only be one of Draft/Accepted/Rejected/Superseded/Deprecated), so a validation script can do a straightforward enum check.

### 2. Strict file naming and directory structure

- Fixed filename format: `adr-008-payment-state-machine-design.md`, with a consistent three- or four-digit number. A project must use one width consistently; three digits is the default, so scripts can scan sequentially and detect gaps or collisions.
- The index table (`docs/adrs/README.md`) shouldn't be maintained by hand — generate it from each file's front matter via script, to keep the index and the body from drifting apart.
- The module table in `docs/architecture.md` should also carry a lightweight structural marker (e.g., a module list wrapped in HTML comments, or a separate `modules.yaml`), so a script can diff it against the actual directory structure.

### 3. Rules a validation script can cover

Once structured, the following checks can be written as CI scripts and enforced at PR time (corresponding to the first group in the Rollout Checklist):

- Are all required front matter fields present, and are their values within the allowed enum range?
- Does the ADR pointed to by `superseded_by` actually exist, and is the bidirectional link consistent (if A says it's been superseded by B, does B also declare that it supersedes A)?
- For every ADR number referenced in AGENTS.md, is the corresponding file still Accepted? Flag an error and prompt for an update if it's already Superseded or Deprecated.
- Does the index table content match each file's front matter? If not, regenerate the index table via script rather than allowing both to be maintained by hand and drift apart.
- Does the module table in docs/architecture.md map one-to-one to the actual code directories? Flag a warning if the code contains a module directory not listed in the table.
- Are there multiple ADRs that are both Accepted but conflict with each other (hard to fully automate — can serve as a prompt for manual review)?

### 4. A supplementary index for Agent retrieval

In addition to the human-readable Markdown index, maintain a machine-readable summary file, e.g. `docs/adrs/index.json`, generated by CI from each ADR's front matter:

```json
[
  {
    "id": "adr-008",
    "title": "Payment state machine design",
    "status": "Accepted",
    "implementation": "pending",
    "path": "docs/adrs/adr-008-payment-state-machine-design.md"
  }
]
```

This lets an Agent read constraints directly from structured data without parsing Markdown prose — faster and more accurate retrieval, and easier to plug into more sophisticated automation later.

### 5. AGENTS.md itself can be lightly structured too

For a larger project with many AGENTS.md entries, consider tagging each rule with its source and reference, so a script can detect when a reference goes stale:

```markdown
- rule: Introducing external middleware such as Redis is prohibited
  source: adr-008
```

Or, at minimum, ensure every rule has a fixed-format prefix like `[See ADR-XXX]`, so a simple regex can extract all the reference relationships as a foundation for future automated validation.

---

## Understanding Checkpoints: A Supplementary Mechanism Against Runaway Complexity

The five-layer document structure above solves the problem of "was the decision and the current state recorded" — but being recorded doesn't mean a human genuinely understands the system. As a project's complexity grows, it's easy for humans to degrade into perceiving the project only through requirement descriptions and test results, losing any real predictive grasp of how the system works internally. The documentation system by itself cannot prevent this — even a beautifully written ADR provides no real understanding if a human just nods along without processing it.

For this reason, layer a set of mandatory checkpoints for human understanding on top of the documentation system:

- **Narrative diffs**: after every completed task, require the Agent to output a plain-language summary alongside the code change itself — what changed, why, and which unconfirmed decisions it made on its own.
- **Teach-back**: for architectural changes, after reading the Agent's summary, the human should be able to restate their understanding in their own words, with the Agent correcting any drift — not a human simply nodding "got it" and moving on.
- **Risk-tiered review**: not every change deserves equal attention. Split changes into "routine implementation detail" and "affects module boundaries / data flow," and concentrate human attention on the latter.
- **Periodic re-derivation drills**: periodically pick a random part of the system and have a human explain, without looking at the code, how it currently works. Being unable to explain it is a signal that trust has outrun understanding, and it's time to stop and have the Agent walk through it again.
- **Complexity budget**: set a cap, per session, on the number of un-reviewed architectural decisions. Once exceeded, force a checkpoint — don't rely on a human's own sense of "have things piled up too much."

The relationship between this mechanism and the five-layer document system: the documents are responsible for "recording," the checkpoints are responsible for "making sure the recording is actually absorbed." Neither works alone — documentation without checkpoints becomes a ritual nobody actually reads; checkpoints without documentation mean understanding evaporates the moment the session ends, with nothing captured or reusable.

---

## Migrating an Existing Repository: A Documentation Cleanup Playbook

The sections above assume the documentation system is being built from scratch. A more common situation: a project has organically accumulated a pile of ADRs and design documents — perhaps through some "interview-style auto-documentation" process — with no strict status management, resulting in multiple documents covering the same thing, some of it already out of sync with the code. This section provides a concrete procedure for bringing an existing repository into the five-layer structure defined here (RFC / ADR / system architecture overview / module README / AGENTS.md).

Core rule: **the input to this cleanup is the code, not the old documents. Old documents serve only as clues and historical reference, never as a source of truth.** At any step, if an old document contradicts the code, the code wins.

### How to use this playbook

- Work through it module by module, in batches. Don't try to clean up the entire repository in one pass. A module is done only once it has completed all six stages below — then move to the next module.
- Every stage comes with a prompt template that can be pasted directly to an Agent.
- Use the "Module Cleanup Progress Tracker" template below throughout, to avoid losing track of progress or repeating work.

### Overall flow

```
Stage 0  Build an index         — inventory existing documents, no judgment yet
     │
Stage 1  Re-derive current state — Agent reads only the code, writes "what it actually is now"
     │
Stage 2  Three-way classification — compare that state against old docs: still accurate / partially stale / entirely wrong
     │
Stage 3  Handle hard cases       — anything that can't be judged is honestly flagged "needs human confirmation," never guessed
     │
Stage 4  Migrate into structure  — classification results are formally written into the ADRs / architecture overview / module READMEs defined by this spec
     │
Stage 5  Put up guardrails       — fill in status fields, wire up this spec's validation mechanisms, prevent it from drifting out of control again
```

### Stage 0: build an index, inventory only, no judgment

**Goal**: get a clear picture of how much documentation exists and what it covers, before anything gets missed. This step makes no judgment about whether any document is right or wrong.

Prompt for the Agent:

> Scan the docs/adrs/ directory and every design document in the project (including module READMEs and scattered .md files), and produce a list: filename, rough topic, and which code directory or module it appears to correspond to. Don't judge whether the content is stale — just build the index. Group the output by module.

Output: a raw document inventory, used as the task list for the subsequent per-module cleanup.

### Stage 1: re-derive current state (done separately for each module)

**Goal**: with zero influence from the old documents, build a fresh description of "what this module actually is right now" straight from the code — this becomes the anchor for later comparison. This is the single most important step in the whole process.

Prompt for the Agent:

> Based only on the actual code under `{module directory}`, describe this module's current responsibilities, external interface, key data structures, and core flow. Do not reference any existing ADR or design document — I want the facts as the code itself presents them. Where the code's intent is unclear, explicitly flag it as "intent unclear, needs confirmation" — don't invent a plausible-sounding explanation for it.

Note: explicitly instruct the Agent not to look at the old documents, so it isn't unconsciously biased by their wording. When intent is unclear, it's better to flag it as uncertain than to let the Agent fill in a reasonable-sounding rationale — a fabricated rationale will get treated as fact and keep propagating through the later steps.

Output: a "code current-state description," serving as the factual baseline for that module's cleanup.

### Stage 2: three-way classification

**Goal**: take the Stage 1 current-state description and compare it, one by one, against every old document relevant to that module, sorting each into one of three buckets.

Prompt for the Agent:

> Here is the current-state description for `{module}` (Stage 1 output attached). Now compare it, one document at a time, against the following old documents: `{document list}`. For each one, classify it as one of the following, with reasoning:
>
> 1. Still accurate — the description matches the code's current state
> 2. Partially stale — the general direction is right, but specific interfaces/parameters/flows no longer match the code; list the specific mismatches
> 3. Entirely wrong — the described content no longer exists in the code, or has changed beyond recognition
>
> If multiple documents describe the same thing, identify which one best matches the current code state; mark the rest as superseded.

Key logic for handling "multiple documents, same topic": whichever one matches the current code state is the canonical one; the rest get marked Superseded and linked to point at it. This is not a subjective "which one is better written" judgment.

Output: a classification and rationale for each old document, plus the determination of which one is canonical in any "collision" cases.

### Stage 3: handle hard cases

**Goal**: be honest about cases that can't be judged, rather than force-fitting them into a category just to make the cleanup look complete.

A document should go into a "needs human confirmation" list — instead of being crammed into one of Stage 2's three buckets — if any of the following apply:

- It's completely unclear which code or module it corresponds to.
- The described feature/module can't be found in the code at all, and it's unclear whether it was ever actually implemented.
- The Agent's Stage 2 reasoning was weak, or its judgment was inconsistent across multiple passes.

Prompt for the Agent:

> For any document from Stage 2 where the evidence was insufficient, or where you're not confident in the classification, list it separately. Explain specifically where you got stuck and what information a human needs to confirm — don't force an uncertain classification.

Output: a "needs human confirmation" list with specific points of doubt attached. A human needs to go through this list item by item and make the final call — it cannot be left to the Agent to decide on its own.

### Stage 4: migrate into structure

**Goal**: formally write the classification results into the corresponding locations in this specification's five-layer structure.

**Still-accurate / partially-stale (after revision) architectural decisions** — migrate into ADRs, filling in the status metadata and the Context / Decision / Non-goals / Consequences four-section structure per Stage 2's rules:

```markdown
Status: Accepted
Implementation: Implemented (specific file path)
```

Determine the implementation status directly from the Stage 1 code current-state description.

**Content involving module breakdown or inter-module dependencies** — consolidate into `docs/architecture.md`, following Stage 3's template.

**Single-module implementation detail** — overwrite the body of the relevant module's README.md directly, trimmed per Stage 4's "content criteria" down to invariants, implicit contracts, quantitative rationale, and Non-goals — don't translate the code line by line. Leave historical versions to Git; don't preserve old content inside the document.

**Entirely-wrong / superseded documents** — don't delete them; move them into an archive directory, e.g. `docs/adrs/_archive/`, and mark the file header:

```markdown
Status: Superseded
Superseded by: adr-0XX-new-document.md
Reason for archiving: no longer matches the current implementation; confirmed as a legacy approach during cleanup
```

Prompt for the Agent (per module):

> Based on the Stage 2 and Stage 3 classifications, formally migrate the documents related to `{module}`:
>
> - Architectural decisions classified "still accurate" or "partially stale (revised per the code)" should be written up as standard-format ADRs (with the Context/Decision/Non-goals/Consequences four sections), with status and implementation fields filled in.
> - Content involving module breakdown/dependencies should be merged into docs/architecture.md.
> - Single-module implementation detail should overwrite `{module}/README.md` directly, retaining only what can't be derived from the code (invariants, implicit contracts, quantitative rationale, Non-goals) — don't translate the code line by line.
> - "Entirely wrong" documents should be moved into docs/adrs/_archive/, marked Superseded, with a stated reason for archiving.
>
> When done, list every file change this migration touched.

### Stage 5: put up guardrails to prevent it from happening again

Finishing the cleanup only returns things to a clean state — without guardrails, the same problems will re-accumulate within a few months. Wire in the mechanisms this specification already defines as part of finishing the cleanup:

- Every ADR has its "status" and "implementation" fields filled in (already done as part of Stage 4).
- Add `docs/adrs/README.md` and `docs/rfcs/README.md` index tables, or auto-generate them via script per "Structural Recommendations for Automated Management."
- Add reference-validation principles to AGENTS.md, so an Agent never cites an ADR that's already been Superseded.
- Wire the earlier "Rollout Checklist" into daily review or CI — the first group becomes scripted rules directly, the second group becomes a periodic manual review checklist.

Once this is done, the results of this cleanup won't slide back to where they started within a few months.

### Module cleanup progress tracker template

|Module|Stage 0 index|Stage 1 current-state|Stage 2 classification|Stage 3 hard cases confirmed|Stage 4 migration complete|Stage 5 guardrails in place|
|---|---|---|---|---|---|---|
|payment|Done|Done|Done|Done|Done|Done|
|auth|Done|Done|In progress|—|—|—|
|notification|Done|—|—|—|—|—|

### Common pitfalls

- **Don't skip Stage 1 and jump straight to Stage 2**: comparing old documents against each other directly can still land on the wrong "canonical" version — just a more plausible-sounding wrong version, not one actually checked against the code.
- **Don't force a classification in Stage 3 just to keep the pace up**: leaving a hard case unresolved is better than misclassifying it — a misclassification disguises bad information as "confirmed," making it much harder to catch later.
- **Archiving is not deleting**: an entirely-wrong document can still be a useful reference for a similar future decision. Deleting it outright loses the historical trace of "why we thought that at the time."
- **Clean up one module at a time**: trying to clean the entire repository in one pass usually leads to fatigue partway through and a drop in standards — later modules end up cleaned to a noticeably lower quality.

### Appendix: ready-to-copy prompt set

```
[Stage 0] Scan the docs/adrs/ directory and every design document in the
project (including module READMEs and scattered .md files), and produce a
list: filename, rough topic, and which code directory or module it appears
to correspond to. Don't judge whether the content is stale — just build the
index. Group the output by module.

[Stage 1] Based only on the actual code under {module directory}, describe
this module's current responsibilities, external interface, key data
structures, and core flow. Do not reference any existing ADR or design
document — I want the facts as the code itself presents them. Where the
code's intent is unclear, explicitly flag it as "intent unclear, needs
confirmation" — don't invent a plausible-sounding explanation for it.

[Stage 2] Here is the current-state description for {module} (Stage 1
output attached). Now compare it, one document at a time, against the
following old documents: {document list}. For each one, classify it as one
of the following, with reasoning:
1. Still accurate — the description matches the code's current state
2. Partially stale — the general direction is right, but specific
   interfaces/parameters/flows no longer match the code; list the specific
   mismatches
3. Entirely wrong — the described content no longer exists in the code, or
   has changed beyond recognition
If multiple documents describe the same thing, identify which one best
matches the current code state; mark the rest as superseded.

[Stage 3] For any document from Stage 2 where the evidence was
insufficient, or where you're not confident in the classification, list it
separately. Explain specifically where you got stuck and what information
a human needs to confirm — don't force an uncertain classification.

[Stage 4] Based on the Stage 2 and Stage 3 classifications, formally
migrate the documents related to {module}:
- Architectural decisions classified "still accurate" or "partially stale
  (revised per the code)" should be written up as standard-format ADRs
  (with the Context/Decision/Non-goals/Consequences four sections), with
  status and implementation fields filled in.
- Content involving module breakdown/dependencies should be merged into
  docs/architecture.md.
- Single-module implementation detail should overwrite {module}/README.md
  directly, retaining only what can't be derived from the code (invariants,
  implicit contracts, quantitative rationale, Non-goals) — don't translate
  the code line by line.
- "Entirely wrong" documents should be moved into docs/adrs/_archive/,
  marked Superseded, with a stated reason for archiving.
When done, list every file change this migration touched.
```

---

## Next Steps

Options from here:

1. Use this file directly as the foundation for the project's documentation library.
2. Provide ready-to-paste blank RFC / ADR / architecture overview template files (already including front matter, with the ADR template carrying the Context/Decision/Non-goals/Consequences four-section structure).
3. Provide a simple validation script example that checks the structural rules above.
4. Flesh out the Understanding Checkpoints mechanism into a concrete operational playbook (e.g., how it's implemented in a Claude Code / Cowork setting).
