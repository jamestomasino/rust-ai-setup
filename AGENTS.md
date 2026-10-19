# AGENTS.md

> Read this file before doing any work in the repository.

## Quick Start

1. Read `AGENTS.md` and `state.md`.
2. Read `docs/LLM_LOOP.md` and `docs/AGENT_RULES.md`.
3. If a track is active, read its goal, frozen decisions, and current task
   section. Do not dump the whole track.
4. Locate only the `contracts.md` and `plan.md` sections needed by that task.
   If no track is active, use their short placeholders to plan the first one.
5. Inspect `git status --short` and `git log -1 --oneline` before editing.

## Architecture

- Three layers: **domain** → **app** → **ui**.
- Flow: `Intent → Command → Event → State`.
- `ui/` must not mutate domain state directly; `domain/` has no side effects.
- Starter stack: ratatui, tokio, serde, thiserror, proptest.

## Repository Map

| Path | Purpose |
|---|---|
| `plan.md` | Goal, scope, track-level backlog, release criteria |
| `contracts.md` | Public boundaries, types, and invariants |
| `state.md` | Small active-track/task cursor and recovery checkpoint |
| `.agent-scratch.md` | Ignored working decisions for the active task only |
| `docs/NNN-*_TRACK.md` | Ordered, independently testable tasks for a phase |
| `docs/AGENT_RULES.md` | Task readiness and execution rules |
| `docs/TASK_TEMPLATE.md` | Required task shape |
| `docs/LLM_LOOP.md` | Iterative and compaction recovery protocol |
| `src/domain/` | State, invariants, reducers |
| `src/app/` | Orchestration, ports, adapters |
| `src/ui/` | Ratatui rendering and input mapping |
| `tests/` | Integration and acceptance tests |
| `fixtures/` | Test fixtures |

## Document Roles

- `plan.md` is the north star, not a detailed task log.
- `contracts.md` owns accepted public behavior and invariants.
- Each track owns its ordered task definitions and acceptance criteria.
- `state.md` identifies exactly one active task or planning step.
- `.agent-scratch.md` holds tentative, task-scoped reasoning across compaction;
  it never overrides the other files and is cleared at task boundaries.

Tracks use `docs/NNN-NAME_TRACK.md` with zero-padded three-digit indices.
Subtracks may use dot notation, such as `001.1-NAME_TRACK.md`. Keep each track
under 300 lines; split a long track instead of loading it whole.

## Working Session

At startup or after compaction, load only the Quick Start working set. Read
sections of `contracts.md` and `plan.md` by heading when needed. Do not load
future tracks or `docs/STATE_ARCHIVE.md` during normal startup. Before each new
task, reread `state.md` and that task's section rather than relying on chat
memory.

If the checkpoint is active, resume only that task. Read `.agent-scratch.md`
only when its task ID matches the checkpoint and reconcile it with Git, tests,
and accepted documents. If there is no active track, complete the planning step
in `state.md` before implementation.

## Track Rules

- Implement tasks in order and only from the active track.
- A task must meet the Definition of Ready in `docs/AGENT_RULES.md`.
- Each task changes at most five primary files, excluding tests and `state.md`.
  Split oversized or ambiguous tasks in a planning-only commit first.
- Tests precede or accompany implementation. Keep each slice independently
  testable and leave the repository coherent.
- Unnamed dependencies, public-contract changes, destructive work, and broad
  refactors require explicit approval unless already authorized by the active
  task or user.

## Task Completion

Run the task's acceptance check and a proportional quality gate. Verify only
allowed files changed. Advance `state.md`, clear its checkpoint and the ignored
scratchpad, and commit exactly one completed task. In a continuing run, reread
the cursor and next task section, then continue if context permits.

If work stops mid-task, update the checkpoint and scratchpad without advancing
the cursor. Follow `docs/LLM_LOOP.md` after compaction.
