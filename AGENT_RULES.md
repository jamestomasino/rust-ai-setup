# Agent Rules

## Scope

- Work on one bounded task at a time.
- Change only the files named in the task.
- Do not redesign the system unless the task requires it.
- Do not mix domain, UI, and adapter changes in one step unless the protocol changes.

## Required Inputs

Each task must define:

- goal
- module
- allowed files
- inputs and outputs
- constraints and invariants
- acceptance test
- non-goals

## Context Files

Read these files before work starts:

Repository structure:

- `src/domain/`: state, invariants, reducers, policies
- `src/app/`: orchestration, ports, adapters
- `src/ui/`: Ratatui rendering and input handling
- `tests/`: integration and acceptance tests
- `fixtures/`: test fixtures
- `adr/`: architecture decision records

- `plan.md`
- `state.md`
- `contracts.md`
- relevant `adr/*.md`
- relevant module files

Update these files when needed:

- `state.md`
- `plan.md`
- `adr/*.md`

## Architecture Rules

- `domain/` owns state and invariants.
- `app/` owns orchestration and adapters.
- `ui/` owns rendering and input mapping.
- `ui/` must not mutate domain state directly.
- State changes must flow through typed intents, commands, events, and reducers.

## Coding Rules

- Keep modules small.
- Prefer one public type per file.
- Avoid god modules.
- Use explicit types.
- Prefer enums and structs over stringly-typed values.
- Prefer validated constructors and newtypes for constrained values.
- Use traits only at external boundaries.
- Keep side effects out of `domain/`.

## Task Process

1. Restate the task.
2. Name the files to change.
3. Name the acceptance test.
4. Make the smallest change that can pass the test.
5. Stop after the task is complete.

## Testing Rules

- Write or update tests before implementation.
- Add unit tests for reducers, parsers, and policies.
- Add integration tests for ports and adapters.
- Add property tests for invariants and transitions where useful.
- Do not remove tests to make code pass.

## Context Control

- Keep prompts short.
- Use repo files for durable context.
- Do not rely on long chat history.
- Summarize progress into `state.md` after each task.
- Start a new thread for unrelated work.
- Re-anchor from context files after compaction.

## Safety Rules

- Ask before destructive actions.
- Ask before broad refactors.
- Ask before adding dependencies.
- Ask before changing public contracts.
- Ask before networked or shell actions if not already authorized.

## Output Rules

For each completed task, report:

- files changed
- tests added or updated
- acceptance result
- open issues
- next suggested task
