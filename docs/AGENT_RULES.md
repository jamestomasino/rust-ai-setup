# Agent Rules

## Scope

- Work on one bounded task at a time and only in its allowed files.
- Preserve the domain, app, and UI boundaries in `contracts.md`.
- Do not redesign the system or change public contracts unless the task names
  that work and the decision is recorded first.

## Task Process

1. Confirm the active task meets the Definition of Ready.
2. Restate its outcome, non-goals, allowed files, and acceptance command.
3. Inspect the worktree and preserve unrelated changes.
4. Set the `state.md` checkpoint and create the task scratchpad.
5. Write or update a test or contract, then make the smallest change that passes.
6. Run acceptance and a proportional quality gate; inspect the final diff.
7. Update `state.md`, clear the scratchpad, and commit exactly one task.
8. In a continuing run, reread the cursor and next task section before starting
   another task. Stop at a documented blocker or context limit.

## Definition of Ready

An implementation task requires:

- one observable outcome and explicit non-goals;
- exact allowed files, at most five primary paths excluding tests and `state.md`;
- dependencies and applicable contract or ADR references;
- an exact acceptance command and expected behavior;
- required fixture, schema, and migration work named explicitly, or stated as
  not applicable;
- no unresolved decision that changes the solution;
- scope small enough to implement, verify, document, and commit in the remaining
  context budget.

If any item is missing, bring the task to Ready in a planning-only commit before
editing implementation code. Use `docs/TASK_TEMPLATE.md` for new tasks.

## Oversized Task Protocol

Split a task before implementation if it exceeds the file limit, has multiple
independent outcomes, changes multiple independent public contracts, lacks one
concise acceptance check, or depends on an unresolved decision. Replace it with
ordered, independently testable subtasks in the active track. Give each subtask
its own outcome, non-goals, allowed files, dependencies, and acceptance check.
Update `state.md` to the first ready subtask and commit the planning change.

New discoveries do not silently expand the task. Record them in a named later
task, backlog item, contract, or ADR as appropriate.

## Coding and Testing

- Keep modules cohesive and split source files before roughly 400 lines.
- Prefer explicit types, validated constructors, and newtypes over loose strings.
- Prefer total functions and explicit data flow. Keep domain logic pure and
  side effects behind app ports and adapters; UI renders and maps input.
- Use unit tests for reducers, parsers, and policies; integration tests for
  ports and adapters; snapshot tests for TUI rendering where useful; and
  property tests for invariants, transitions, and stateful workflows.
- Never remove or weaken tests merely to make a change pass.

## Context and Safety

- Read only the active task and relevant document sections; follow
  `docs/LLM_LOOP.md` after compaction.
- Keep `state.md` short and `.agent-scratch.md` task-scoped. Neither replaces
  accepted contracts, code, tests, or Git history.
- Record material architecture decisions in an ADR or contract before dependent
  implementation.
- Ask before destructive actions, broad refactors, unnamed dependencies, or
  public-contract changes. Do not repeat permission requests for actions already
  authorized by the active task or user.
- Do not expose or commit credentials.

## Definition of Done

- The stated behavior and acceptance command pass.
- Tests accompany implementation and only allowed files changed.
- Required contract, schema, migration, and fixture work is present.
- `state.md` advances, its checkpoint is cleared, and the scratchpad is removed.
- One descriptive commit contains exactly the completed task.

Report changed files, tests, acceptance result, open issues, and next task.
