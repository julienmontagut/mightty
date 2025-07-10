//! Mightty - A modern terminal emulator
//!
//! This crate provides a terminal emulator built with Rust and egui,
//! featuring PTY support and VTE parsing.
//!
//! # Architecture
//!
//! The crate is organized into several modules:
//!
//! - [`app`]: Application layer with GUI components
//! - [`config`]: Configuration management
//! - [`parser`]: VTE escape sequence parsing
//! - [`pty`]: Pseudo-terminal process management
//! - [`terminal`]: Core terminal emulation logic
//! - [`ui`]: User interface utilities

pub mod app;
pub mod config;
pub mod parser;
pub mod pty;
pub mod terminal;
pub mod ui;

pub use app::gui::App;
pub use config::Config;
