# State

This is the minimal durable cursor for the implementation loop. Completed work
belongs in Git and the relevant track file, not in a growing task log here.

## Current Status

- Active track: none; scaffold ready for project planning.
- Position: no implementation task started.

## Next Task

Create the first bounded, ready track task after filling in the project goal,
scope, contracts, and track-level backlog. This is a planning-only step.

## In-Progress Checkpoint

None.

When a task starts, record its exact ID, `in_progress` status, base commit,
changed files, tests run, and remaining steps. Keep this short; task-local
reasoning belongs in the ignored `.agent-scratch.md`. Clear the checkpoint only
after the task's acceptance check passes and the cursor advances.

## Open Decisions and Blockers

None.

## Recent Transition

None.

## State Maintenance Rules

- Keep this file under 100 lines.
- Store only the active track/path, next task, recovery checkpoint, unresolved
  decisions or blockers, and at most one recent transition.
- Do not accumulate completed tasks, test inventories, or review history here.
- If historical state needs preservation, move it to `docs/STATE_ARCHIVE.md`;
  do not load that archive during normal startup or compaction recovery.
