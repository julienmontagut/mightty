# Mightty 🚀

[![CI](https://github.com/julienmontagut/mightty/workflows/CI/badge.svg)](https://github.com/julienmontagut/mightty/actions)
[![Release](https://github.com/julienmontagut/mightty/workflows/Release/badge.svg)](https://github.com/julienmontagut/mightty/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

A modern, fast, and lightweight terminal emulator built with Rust and egui.

## ✨ Features

- **🚀 High Performance**: Built with Rust for maximum performance and memory safety
- **🎨 Modern UI**: Beautiful, responsive interface powered by egui
- **⚡ Cross-Platform**: Works on macOS, Linux, and Windows
- **🔧 Configurable**: Extensive configuration options via TOML files
- **🎯 VTE Compatible**: Full VTE (Virtual Terminal Emulator) support
- **🧵 Async I/O**: Non-blocking I/O operations for smooth user experience

## 🏗️ Architecture

Mightty is built with a modular architecture:

```
├── app/         # GUI application layer
├── config/      # Configuration management  
├── parser/      # VTE escape sequence parsing
├── pty/         # Pseudo-terminal management
├── terminal/    # Core terminal emulation
└── ui/          # User interface utilities
```

## 🚀 Installation

### From Source

```bash
git clone https://github.com/julienmontagut/mightty.git
cd mightty
cargo build --release
```

### From Releases

Download the latest binary from the [releases page](https://github.com/julienmontagut/mightty/releases).

## 🎯 Usage

### Basic Usage

```bash
# Run with default settings
mightty

# Specify configuration file
mightty --config /path/to/config.toml

# Set window title
mightty --window-title "My Terminal"
```

### Configuration

Mightty supports configuration through:
- TOML configuration files
- Environment variables (prefixed with `MIGHTTY_`)
- Command-line arguments

Configuration file locations:
- **Linux**: `~/.config/mightty/config.toml`
- **macOS**: `~/Library/Application Support/mightty/config.toml`
- **Windows**: `%APPDATA%\mightty\config.toml`

Example configuration:

```toml
# config.toml
window_title = "Mightty Terminal"
width = 1200.0
height = 800.0
enable_vsync = true
```

## 🛠️ Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security vulnerabilities
cargo audit
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo tarpaulin --html
```

## 📦 Dependencies

- **egui**: Modern immediate mode GUI framework
- **eframe**: Native egui integration
- **portable-pty**: Cross-platform PTY implementation
- **vte**: Virtual terminal emulator parser
- **tokio**: Async runtime
- **anyhow**: Error handling
- **clap**: Command-line argument parsing
- **figment**: Configuration management
- **serde**: Serialization framework

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run linting (`cargo clippy`)
6. Format code (`cargo fmt`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## 📋 Roadmap

- [ ] **Tabs and Panes**: Multi-tab and split-pane support
- [ ] **Themes**: Customizable color schemes
- [ ] **Plugins**: Plugin system for extensibility
- [ ] **Performance**: GPU acceleration for rendering
- [ ] **Accessibility**: Screen reader and keyboard navigation support
- [ ] **Shell Integration**: Enhanced shell integration features

## 🐛 Bug Reports

If you find a bug, please create an issue with:
- Your operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce the issue
- Expected behavior
- Actual behavior

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🏆 Acknowledgments

- [Alacritty](https://github.com/alacritty/alacritty) - Inspiration for terminal emulator design
- [egui](https://github.com/emilk/egui) - Excellent immediate mode GUI framework
- [VTE](https://github.com/GNOME/vte) - VTE specification and reference

## 📊 Stats

![Alt](https://repobeats.axiom.co/api/embed/your-repo-id.svg "Repobeats analytics image")

---

<div align="center">
  <strong>Made with ❤️ and Rust</strong>
</div>