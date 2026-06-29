# Contracts

## Event Flow

```
Intent -> Command -> Event -> State
```

## Public Types

| Type       | Layer   | Responsibility                              |
|------------|---------|---------------------------------------------|
| Intent     | domain  | Raw user/system intention                   |
| Command    | domain  | Validated intent ready for dispatch         |
| Event      | domain  | Result of command execution                 |
| Reducer    | domain  | Deterministic state transitions             |
| ViewModel  | ui      | Presentable domain state for rendering      |
| Port       | app     | Interface contract for external integration |
| Adapter    | app     | Implementation of a Port                    |

## Invariants

- `ui/` must not mutate domain state directly.
- State changes must flow through typed intents, commands, events, and reducers.
- `domain/` contains no side effects.
- Illegal states must be unrepresentable.

## Module Boundaries

| Module     | Owns                          | Depends On          |
|------------|-------------------------------|---------------------|
| domain/    | State, invariants, reducers   | Nothing external    |
| app/       | Orchestration, adapters       | domain/             |
| ui/        | Rendering, input mapping      | domain/ (read-only) |
