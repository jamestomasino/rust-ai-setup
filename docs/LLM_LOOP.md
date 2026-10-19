# Compaction-Safe Local LLM Loop

## Purpose

Run one bounded task, one acceptance gate, and one commit at a time. Chat
history is temporary. The track, `state.md`, contracts, Git, tests, diff, and
task scratchpad provide recovery after compaction or a new session.

## Short Starting Prompt

```text
Follow AGENTS.md and docs/LLM_LOOP.md. Use state.md to select exactly one task.
Read only its track section and relevant contract or plan sections. Complete,
verify, update state, and commit that task. Then reread the cursor and continue
independently while context and the documented stop conditions permit.
```

## Start and Recovery

At startup and after every compaction:

1. Confirm the repository path. Read `AGENTS.md`, `state.md`, this file, and
   `docs/AGENT_RULES.md`.
2. If a track is active, read its goal, frozen decisions, and current task
   section. Locate relevant `contracts.md` or `plan.md` sections by heading;
   do not dump whole documents or future tracks.
3. Inspect `git status --short`, `git log -1 --oneline`, and any active
   checkpoint. If no track is active, perform only the planning step in
   `state.md`.
4. If a checkpoint is active, inspect the diff, then read `.agent-scratch.md`
   only when its task ID matches. Reconcile tentative notes with current code,
   accepted documents, and tests before editing. Resume only that task.
5. If no checkpoint is active, remove a stale scratchpad and select only
   `Next Task`. Confirm its status is `ready` and it meets the Definition of
   Ready. If marked `proposed`, complete its specification in a planning commit.

The first action after compaction must be recovery, never an implementation
edit. Do not select tasks or decisions from remembered chat context.

## Task Iteration

Before implementation, restate the single outcome, non-goals, allowed files,
dependencies, and exact acceptance command. Record `in_progress` and the base
commit in `state.md`; create `.agent-scratch.md` with the same task ID. Preserve
unrelated worktree changes.

During implementation, keep tests beside the behavior they verify. Update the
scratchpad after meaningful investigations or decisions, even before code or
TODOs exist. If interrupted or nearing compaction, update the short checkpoint
with changed files, tests passing or failing, remaining steps, and deferred
discoveries. Do not advance `Next Task` while work is incomplete.

After implementation, run the exact acceptance command and proportional
quality gate. Inspect the diff and verify only allowed files changed. Resolve
failing tests or unexplained changes before continuing. Advance `state.md`,
record at most one recent transition, clear the checkpoint and scratchpad, and
commit exactly one completed task. Reread `state.md` and the next task section
before continuing in the same run.

## Task Scratchpad

`.agent-scratch.md` is Git-ignored and scoped to one task or subtask. Start it
as soon as the checkpoint becomes `in_progress`. Keep a concise synthesis,
normally under 100 lines:

```markdown
# Active task: <exact task or subtask ID>
Base commit: <hash>
Outcome: <one sentence>

## Current understanding
- <finding with file, section, or test reference>

## Working decisions
- <tentative or confirmed choice, reason, and evidence; label tentative>

## Open questions and next actions
- <next concrete step or unresolved question>
```

Do not store transcripts, command dumps, credentials, or an independent task
plan. Scratchpad entries are hypotheses until checked against the track,
contracts, ADRs, code, tests, and Git. Move accepted public or architecture
decisions to durable files and out-of-scope discoveries to named later tasks.

Clear the scratchpad when a task or subtask is committed, canceled, or split.
Create a fresh one for the next ID and base commit. If it is missing after
compaction, recover from the checkpoint, diff, tests, and task sources without
inventing its contents. If its ID differs from the checkpoint, treat it as
stale; never carry its decisions into another task.

## Context Budget

Do not start a task when the remaining context is unlikely to fit its full
implementation and verification. If the runner reports usage, avoid starting
new work after roughly 70% of its context window is consumed. Near the limit,
finish the current task or refresh the checkpoint and scratchpad. A task
boundary does not reset context automatically; use a runner-provided reset at
a committed boundary, or stop with the next task recorded in `state.md`.

Keep tool output and chat summaries targeted. Do not load historical state
archives during normal startup or recovery. Keep `state.md` under 100 lines;
store history in Git and completed tracks.

## Stop Conditions

Stop and report when a required product or architecture decision is unresolved,
authority would expand scope, unrelated changes cannot be preserved, acceptance
fails for reasons outside the task, a required external dependency is
unavailable, or context is too low to complete another task. Compaction itself
is not a blocker: recover from repository state and resume the same task.
