# Plan

## Current Goal

Restructure repository for Rust TUI app build.

## Scope

- Three-layer architecture: `domain/`, `app/`, `ui/`
- Communication flow: `Intent -> Command -> Event -> State`
- Starter stack: ratatui, tokio, serde, thiserror, proptest

## Completed

- [x] Repository structure created with placeholder modules
- [x] Cargo.toml configured with starter dependencies

## Next Steps

- Define first domain model and reducer branch
- Set up TUI event loop skeleton
- Add first integration test
