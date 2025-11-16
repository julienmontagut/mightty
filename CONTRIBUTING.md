# Contributing to Mightty

Thank you for your interest in contributing to Mightty! 🎉

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/mightty.git
   cd mightty
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/julienmontagut/mightty.git
   ```

## Development Setup

### Prerequisites

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **System dependencies** (Linux):
  ```bash
  sudo apt-get install -y libgtk-3-dev libxcb1-dev libxrandr-dev libxss1 libgconf-2-4 libxcomposite1 libxi6 libxdamage1 libxtst6 libnss3 libxrandr2 libasound2 libpangocairo-1.0-0 libatk1.0-0 libcairo-gobject2 libgtk-3-0 libgdk-pixbuf2.0-0
  ```

### Setup

1. **Install development tools**:
   ```bash
   cargo install cargo-watch cargo-audit cargo-tarpaulin
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Run the application**:
   ```bash
   cargo run
   ```

### Development Workflow

1. **Start with a fresh branch**:
   ```bash
   git checkout main
   git pull upstream main
   git checkout -b feature/your-feature-name
   ```

2. **Watch for changes during development**:
   ```bash
   cargo watch -x 'check --all-targets' -x 'test --all-targets'
   ```

3. **Before committing**, run the full test suite:
   ```bash
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   ```

## Making Changes

### Project Structure

```
src/
├── app/         # GUI application layer
├── config/      # Configuration management
├── parser/      # VTE escape sequence parsing
├── pty/         # Pseudo-terminal management
├── terminal/    # Core terminal emulation
└── ui/          # User interface utilities
```

### Areas for Contribution

- **Bug fixes**: Always welcome!
- **Performance improvements**: Profiling and optimization
- **New features**: See our [roadmap](README.md#roadmap)
- **Documentation**: Code comments, API docs, user guides
- **Testing**: Unit tests, integration tests, benchmarks
- **Platform support**: Windows, Linux, macOS improvements

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with coverage
cargo tarpaulin --html

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check
```

### Writing Tests

- **Unit tests**: Place in the same file as the code being tested
- **Integration tests**: Place in `tests/` directory
- **Documentation tests**: Include examples in doc comments

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Test implementation
    }
}
```

## Submitting Changes

### Before Submitting

1. **Update documentation** if needed
2. **Add tests** for new functionality
3. **Run the full test suite**:
   ```bash
   cargo test
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   ```
4. **Update CHANGELOG.md** if applicable

### Pull Request Checklist

- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Commit messages follow [convention](#commit-messages)
- [ ] PR description explains the change
- [ ] Links to related issues

## Code Style

### Rust Style

- Follow the official [Rust style guide](https://doc.rust-lang.org/style-guide/)
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common issues
- Prefer explicit types when it improves readability
- Use meaningful variable and function names

### Documentation

- All public APIs must have documentation
- Use doc comments (`///`) for public items
- Include examples in documentation when helpful
- Keep documentation up to date with code changes

```rust
/// Parses VTE escape sequences into terminal events.
///
/// # Examples
///
/// ```
/// use mightty::parser::VteParser;
/// 
/// let mut parser = VteParser::new();
/// let events = parser.parse(b"\x1b[H");
/// ```
pub struct VteParser {
    // ...
}
```

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feat(parser): add support for OSC sequences

fix(pty): handle process termination correctly

docs(readme): update installation instructions

chore: bump version to 0.2.0 [skip-version]
```

**Note**: Use `[skip-version]` in commit messages to skip automatic version bumping.

## Pull Request Process

1. **Create a descriptive PR title** following conventional commits
2. **Fill out the PR template** completely
3. **Link to related issues** using keywords like "Fixes #123"
4. **Request review** from maintainers
5. **Address feedback** promptly
6. **Squash commits** if requested before merge

### PR Description Template

```markdown
## Summary
Brief description of the changes.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] Tests pass locally
- [ ] Added tests for new functionality
- [ ] Manual testing performed

## Checklist
- [ ] Code follows the project's style guidelines
- [ ] Self-review of code completed
- [ ] Code is commented, particularly in hard-to-understand areas
- [ ] Corresponding changes made to documentation
- [ ] No new warnings introduced
```

## Getting Help

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Documentation**: Check the README and API documentation

## Recognition

Contributors are recognized in:
- GitHub contributors list
- Release notes for significant contributions
- README acknowledgments section

Thank you for contributing to Mightty! 🚀