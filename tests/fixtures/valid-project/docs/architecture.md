# System Architecture Overview

## Module breakdown
| Module | Responsibility | Detailed design | Linked ADR |
|---|---|---|---|
| payment | Payment flow and state machine | payment/README.md | adr-001 |
| auth | Authentication and session management | auth/README.md | — |

## Inter-module dependencies
(payment depends on auth for identity)

## Boundary principles
(No cross-module access to internal data structures; calls go through public interfaces.)
