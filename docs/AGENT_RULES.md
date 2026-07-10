# Agent Rules

## Scope

- Work on one bounded task at a time. Change only the files named in the task.
- Do not redesign the system unless the task requires it.
- Do not mix domain, UI, and adapter changes unless a protocol change requires both.

## Task Process

1. Restate the task. 2. Name the files to change. 3. Name the acceptance test.
4. Make the smallest change that can pass the test. 5. Stop.

## Coding

- Keep modules small. Prefer one public type per file. No god modules.
- Aim for source files under 400 lines. If a file grows beyond that, split it.
- Use explicit types — enums and structs over strings.
- Prefer validated constructors and newtypes for constrained values.
- Prefer total functions, explicit data flow, pure logic in `domain/`.
- Side effects in `app/` adapters only. `ui/` is rendering and input handling.

## Testing

- Write or update tests before implementation.
- Unit tests for reducers, parsers, policies.
- Integration tests for ports and adapters.
- Snapshot tests for TUI rendering where useful.
- Property tests with `proptest` for invariants and transitions.
- Stateful property tests for workflows.
- Never remove tests to make code pass.

## Context Control

- Use repo files for durable context. Do not rely on long chat history.
- Summarize progress into `state.md` after each task.
- When implementation diverges from docs, update the docs immediately.
- Record material architecture decisions in `contracts.md` or `plan.md` before moving on.

## Safety

- Ask before destructive actions, broad refactors, adding dependencies, or changing public contracts.
- Ask before networked or shell actions if not already authorized.

## Output

For each completed task, report: files changed, tests added/updated, acceptance result, open issues, next suggested task.
