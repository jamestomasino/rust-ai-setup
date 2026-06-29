# Rust AI Agent Context

## Goals

- Keep context windows small.
- Let the agent implement one bounded task at a time.
- Preserve correctness with types, tests, and explicit boundaries.
- Keep core logic stable while allowing local iteration in the UI and adapters.

## Architecture

- Use Rust for the full stack.
## Repository Structure

- `src/domain/`: state, invariants, reducers, policies
- `src/app/`: orchestration, ports, adapters
- `src/ui/`: Ratatui rendering and input handling
- `tests/`: integration and acceptance tests
- `fixtures/`: test fixtures
- `adr/`: architecture decision records
- `plan.md`: current goal and scope
- `state.md`: current status and next step
- `contracts.md`: event flow and public contracts
- `AGENT_RULES.md`: agent operating rules
- `rust-ai-agent-context.md`: project architecture context

- Split code into three layers:
  - `domain/`: state, invariants, plans, reducers, policies.
  - `app/`: orchestration, commands, ports, adapters.
  - `ui/`: Ratatui rendering, input mapping, focus, panes.
- The `ui/` layer must not mutate state directly.
- The `domain/` layer owns all state transitions.
- Communication path:
  - `Intent -> Command -> Event -> State`

## State Model

- Use typed enums for `Intent`, `Command`, and `Event`.
- Use a central reducer for state transitions.
- Prefer deterministic transitions.
- Prefer append-only event logs for important actions.
- Make illegal states unrepresentable.
- Use newtypes and validated constructors for constrained values.

## Boundaries

- Use traits only at external boundaries.
- Keep provider, tool, and I/O integrations behind ports.
- Keep adapters thin and replaceable.
- Avoid stringly-typed interfaces when enums or structs fit.
- Keep protocol and event contracts explicit.

## File and Module Rules

- Keep modules small.
- Prefer one public type per file.
- Avoid god modules.
- Use stable names:
  - `Intent`
  - `Command`
  - `Event`
  - `Reducer`
  - `ViewModel`
  - `Port`
  - `Adapter`
- Separate `domain/` and `ui/` changes unless a protocol change requires both.

## Testing

- Write tests before implementation.
- Add unit tests for reducers, parsers, and policies.
- Add integration tests for ports and adapters.
- Add snapshot tests for TUI rendering where useful.
- Add property tests with `proptest` for invariants, round-trips, and transitions.
- Use stateful property tests for workflows.

## Agent Work Rules

- Each task must target one module and one acceptance test.
- Each change must be local and bounded.
- Do not ask the agent to redesign the full system in one step.
- Prefer tasks like:
  - add one reducer branch
  - add one adapter
  - add one parser
  - add one test
  - add one widget
- Keep prompts specific about inputs, outputs, and invariants.

## Implementation Preferences

- Prefer total functions where practical.
- Prefer explicit data flow over implicit shared state.
- Prefer pure logic in `domain/`.
- Keep side effects in `app/` adapters.
- Keep `ui/` focused on rendering and input handling.
- Use fixture-backed tests for external integrations.

## Starter Stack

- `ratatui`
- `tokio`
- `serde`
- `thiserror`
- `proptest`

## Definition of Done

- The change is covered by tests.
- The change preserves domain invariants.
- The change does not expand scope beyond the assigned module.
- The public types and event flow remain explicit.
- The code compiles and the tests pass.
