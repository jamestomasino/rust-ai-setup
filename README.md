# Rust AI Agent TUI Template

A starting point for a Rust terminal app built with ratatui and tokio. It includes
domain, app, and UI module boundaries, plus a task workflow for working with a
local LLM. The binary is currently a scaffold; `cargo run` prints a placeholder
message.

## Get started

1. Set the project's goal, scope, and backlog in [`plan.md`](plan.md).
2. Record its public behavior and invariants in [`contracts.md`](contracts.md).
3. Create the first bounded task using [`docs/TASK_TEMPLATE.md`](docs/TASK_TEMPLATE.md)
   and point [`state.md`](state.md) to it. The agent workflow is defined in
   [`AGENTS.md`](AGENTS.md) and [`docs/LLM_LOOP.md`](docs/LLM_LOOP.md).
4. Run `cargo test` to check the starter project.

## Sample prompt for the local LLM loop

> Read AGENTS.md, state.md, docs/LLM_LOOP.md, and docs/AGENT_RULES.md. Follow the task loop. Read only the active track’s goal, frozen decisions, and current task section; retrieve relevant plan or contract sections as needed. Complete, verify, update state, and commit one task, then reread the cursor and continue independently. Stop only at a documented stop condition or when there is too little context for another task.
