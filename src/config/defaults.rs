//! Platform-specific default values determined at compile time.
//!
//! This module replaces runtime platform detection with compile-time
//! platform-specific defaults to simplify the codebase.

/// Default URL opener command for the current platform
#[cfg(target_os = "macos")]
pub const DEFAULT_URL_OPENER: &str = "open";

#[cfg(not(target_os = "macos"))]
pub const DEFAULT_URL_OPENER: &str = "xdg-open";

/// Default font family for the current platform
#[cfg(target_os = "macos")]
pub const DEFAULT_FONT_FAMILY: &str = "Menlo";

#[cfg(not(target_os = "macos"))]
pub const DEFAULT_FONT_FAMILY: &str = "monospace";

/// Default font style for all platforms
pub const DEFAULT_FONT_STYLE: &str = "Regular";

/// Default font size for all platforms
pub const DEFAULT_FONT_SIZE: f32 = 12.0;

/// Default shell for the current platform
#[cfg(target_os = "macos")]
pub const DEFAULT_SHELL: Option<&str> = None; // Will use $SHELL or login shell

#[cfg(not(target_os = "macos"))]
pub const DEFAULT_SHELL: Option<&str> = None; // Will use $SHELL or login shell

/// Default working directory detection method
#[cfg(target_os = "macos")]
pub const USES_PROC_FS: bool = false;

#[cfg(not(target_os = "macos"))]
pub const USES_PROC_FS: bool = true;

/// Platform-specific path for working directory detection
#[cfg(target_os = "macos")]
pub fn proc_cwd_path(_pid: i32) -> String {
    // macOS uses a different method via proc_pidpath
    String::new()
}

#[cfg(not(target_os = "macos"))]
pub fn proc_cwd_path(pid: i32) -> String {
    format!("/proc/{}/cwd", pid)
}

/// Default opacity value
pub const DEFAULT_OPACITY: f32 = 1.0;

/// Default cursor blinking
pub const DEFAULT_CURSOR_BLINKING: bool = false;

/// Default scrollback lines
pub const DEFAULT_SCROLLBACK_LINES: u32 = 10000;
