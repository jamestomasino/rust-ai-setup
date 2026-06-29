// Domain layer – state, invariants, reducers, policies
//
// This layer owns all state transitions. No side effects allowed.

pub mod state;
pub mod intent;
pub mod command;
pub mod event;
pub mod reducer;
pub mod policy;
pub mod invariant;
