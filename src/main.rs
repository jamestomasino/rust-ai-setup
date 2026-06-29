//! Rust AI Agent – TUI Application
//!
//! Three-layer architecture:
//! - `domain/`: state, invariants, reducers, policies
//! - `app/`: orchestration, ports, adapters
//! - `ui/`: Ratatui rendering and input handling
//!
//! Communication flows as `Intent -> Command -> Event -> State`.

mod app;
mod domain;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust AI Agent TUI");
    Ok(())
}
