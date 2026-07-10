# Contracts

## Event Flow

```
Intent -> Command -> Event -> State
```

## Invariants

- `ui/` must not mutate domain state directly
- State changes flow through typed intents, commands, events, and reducers
- `domain/` contains no side effects
- Illegal states must be unrepresentable
- Important actions use append-only event logs

## Boundaries

| Module    | Owns                     | Depends On       |
|-----------|--------------------------|------------------|
| domain/   | State, invariants        | Nothing external |
| app/      | Orchestration, adapters  | domain/          |
| ui/       | Rendering, input mapping | domain/ (read)   |

- Traits are used only at external boundaries
- Provider, tool, and I/O integrations sit behind ports
- Adapters are thin and replaceable

## Naming Conventions

Use these stable names for core types:

- `Intent`, `Command`, `Event`, `Reducer`
- `ViewModel`, `Port`, `Adapter`

## Quality Gate

- `cargo clippy --all-targets --all-features -- -D warnings` must pass
- Stylistic-only lints (manual_map, redundant_closure, etc.) allowed via `[lints.clippy]` config
