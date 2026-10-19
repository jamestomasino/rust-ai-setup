# Task Template

Use this shape for each task or subtask in an implementation track. A task is
not ready until every field is concrete.

```markdown
## Task NNN.N — Outcome-oriented title
Status: `ready`

**Outcome:** One observable result.
**Dependencies/contracts:** Prior task and relevant contract or ADR sections.
**Non-goals:** Explicit exclusions and named later work.
**Allowed files:** Exact primary paths (at most five), tests, and `state.md`.
**Work:** Test or contract first, implementation, and any fixture, schema, or
migration work (or say not applicable).
**Acceptance:** `exact command`; expected behavior and relevant boundary case.
```

Status values: `proposed` (incomplete), `ready` (passes Definition of Ready),
`in_progress` (checkpoint active), `blocked` (evidence in `state.md`), and
`done` (accepted, cursor advanced, committed).

Split by independently observable behavior, not arbitrary code layers. Each
subtask must leave the repository coherent and carry its own test. Name schema
and migration compatibility expectations when they apply.
