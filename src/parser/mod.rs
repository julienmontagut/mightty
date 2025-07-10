//! Terminal escape sequence parsing
//!
//! This module handles parsing of VTE (Virtual Terminal Emulator) escape sequences
//! and converts them into terminal events.

pub mod vte;

pub use vte::{TerminalEvent, VteParser};
