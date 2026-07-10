# AGENTS.md

> Read this file before doing any work in the repository.

## Quick Start

1. Read this file — architecture and operating rules
2. Read `contracts.md` — event flow and invariants
3. Read `plan.md` — scope and status
4. Read `state.md` — current status

## Architecture

- Three layers: **domain** → **app** → **ui**
- Flow: `Intent → Command → Event → State`
- `ui/` must not mutate domain state directly; `domain/` has no side effects
- Starter stack: ratatui, tokio, serde, thiserror, proptest

## Repository Map

| Path                  | Purpose                              |
|-----------------------|--------------------------------------|
| `docs/`               | Operating rules (AGENT_RULES.md)     |
| `contracts.md`        | Event flow, public types, invariants |
| `state.md`            | Current status, next step            |
| `plan.md`             | Goal, scope, status                  |
| `src/domain/`         | State, invariants, reducers          |
| `src/app/`            | Orchestration, ports, adapters       |
| `src/ui/`             | Ratatui rendering, input handling    |
| `tests/`              | Integration and acceptance tests     |
| `fixtures/`           | Test fixtures                        |

## Implementation Tracks

When working on a feature or phase, define a plan file in `docs/` and add it to the track chain below. Each plan file contains scoped tasks with acceptance criteria.

The current implementation track is defined in `state.md` under "Current Track".

### Working Session (Minimal Context)

To implement a task, load only these files:

1. `AGENTS.md` — architecture and operating rules
2. `contracts.md` — event flow and invariants
3. `plan.md` — scope and status
4. `state.md` — current status and next task
5. The active plan file (listed in `state.md` under "Current Track")

Do not load every plan file. Load only the active one.

Before doing work on the current track, read:

1. The active plan file (see `state.md`)
2. `contracts.md`
3. `state.md`

Rules:

- Implement tasks in order.
- Do not skip tasks.
- Do not implement features outside the active plan.
- Each task should change at most 5 files. If more are needed, split the task.
- Plan files should stay under 300 lines. If a plan grows beyond that, break it into
  multiple plan files in the track.

### Post-Task Completion

After implementing a task, update `state.md` to:

1. Mark the completed task under "Completed Tasks" with a brief summary
2. Advance "Next Task" to the following task in the active plan
3. If the active plan is fully complete, advance to the next plan in the track
4. Commit the changes with a descriptive message

This ensures a fresh session can resume from the correct task.
