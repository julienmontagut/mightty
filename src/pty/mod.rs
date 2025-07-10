//! Pseudo-terminal (PTY) management
//!
//! This module provides PTY process management including process spawning,
//! I/O handling, and lifecycle management.

pub mod manager;

pub use manager::Pty;
