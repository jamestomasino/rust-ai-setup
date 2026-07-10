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

| Path                  | Purpose                                              |
|-----------------------|------------------------------------------------------|
| `plan.md`             | **North star** — goal, scope, backlog, done criteria |
| `docs/`               | **Implementation tracks** — phased task lists        |
| `docs/AGENT_RULES.md` | Operating rules                                      |
| `contracts.md`        | Event flow, public types, invariants                 |
| `state.md`            | Current status, active track, next task              |
| `src/domain/`         | State, invariants, reducers                          |
| `src/app/`            | Orchestration, ports, adapters                       |
| `src/ui/`             | Ratatui rendering, input handling                    |
| `tests/`              | Integration and acceptance tests                     |
| `fixtures/`           | Test fixtures                                        |

## Implementation Tracks

As a project grows, `plan.md` holds the strategic vision while individual
implementation tracks live in `docs/`. This keeps the root plan readable and
lets each phase get its own detailed task list.

Tracks are named `docs/NNN-NAME_TRACK.md` with zero-padded, three-digit indices
(e.g. `docs/001-SETUP_TRACK.md`). When a track splits into subtracks they use
dot notation (e.g. `001.1-...`, `001.2-...`).

### Document Roles

- **`plan.md` (root)** — North star. Goal, scope, backlog, and done criteria
  for the entire project. Read this to understand *what* we're building.
- **`docs/NNN-*_TRACK.md`** — Implementation tracks. Ordered Tasks with acceptance
  criteria for a specific phase. Read the active one to understand *how* to
  build the next piece.
- **`state.md`** — Pointer. Which track is active, what's the next task, what's
  done. Read this to know *where* to work next.

The current implementation track is defined in `state.md` under "Current Track".

### Working Session (Minimal Context)

To implement a Task, load only these files:

1. `AGENTS.md` — architecture and operating rules
2. `contracts.md` — event flow and invariants
3. `plan.md` — scope and status (skim for context)
4. `state.md` — current status and next task
5. The active track file (listed in `state.md` under "Current Track")

Do not load every track file. Load only the active one.

Before doing work on the current track, read:

1. The active track file (see `state.md`)
2. `contracts.md`
3. `state.md`

### Track Rules

- Implement Tasks in order.
- Do not skip Tasks.
- Do not implement features outside the active track.
- Each Task should change at most 5 files. If more are needed, split the Task.
- Each track file should stay under 300 lines. If a track grows beyond that,
  break it into multiple track files in the same phase.
- When starting a new phase, create a track file in `docs/`, add it to the
  track chain in `state.md`, and update `plan.md` backlog status if needed.

### Post-Task Completion

After implementing a Task, update `state.md` to:

1. Mark the completed Task under "Completed Tasks" with a brief summary
2. Advance "Next Task" to the following Task in the active track
3. If the active track is fully complete, advance to the next track in the chain
4. Commit the changes with a descriptive message

This ensures a fresh session can resume from the correct task.
