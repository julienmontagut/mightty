//! Terminal emulation
//!
//! This module provides the core terminal emulation functionality,
//! including terminal state management and emulator logic.

pub mod emulator;
pub mod state;

pub use emulator::{Terminal, TerminalConfig};
pub use state::TerminalState;
