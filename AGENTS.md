# AGENTS.md

> Read this file before doing any work in the repository.

## Quick Start

1. Read `docs/CONTEXT.md` — architecture and goals
2. Read `docs/AGENT_RULES.md` — operating rules
3. Read `contracts.md` — event flow and public types
4. Read `state.md` — current status
5. Read `plan.md` — scope and backlog

## Architecture

- Three layers: **domain** → **app** → **ui**
- Flow: `Intent → Command → Event → State`
- `ui/` must not mutate domain state directly; `domain/` has no side effects
- Starter stack: ratatui, tokio, serde, thiserror, proptest

## Repository Map

| Path                  | Purpose                              |
|-----------------------|--------------------------------------|
| `docs/`               | Context and agent rules              |
| `contracts.md`        | Event flow, public types, invariants |
| `state.md`            | Current status, next step            |
| `plan.md`             | Goal, scope, backlog                 |
| `adr/`                | Architecture Decision Records        |
| `src/domain/`         | State, invariants, reducers          |
| `src/app/`            | Orchestration, ports, adapters       |
| `src/ui/`             | Ratatui rendering, input handling    |
| `tests/`              | Integration and acceptance tests     |
| `fixtures/`           | Test fixtures                        |
