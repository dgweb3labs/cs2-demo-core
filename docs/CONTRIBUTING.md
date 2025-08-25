# Contributing to CS2 Demo Core

Thank you for your interest in contributing to CS2 Demo Core! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- A CS2 demo file for testing (optional)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/cs2-demo-core.git
   cd cs2-demo-core
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/dgweb3labs/cs2-demo-core.git
   ```

## Development Setup

### Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional tools
cargo install cargo-fmt
cargo install cargo-clippy
cargo install cargo-test
```

### Verify Setup

```bash
# Check that everything compiles
cargo check

# Run tests
cargo test

# Run examples
cargo run --example simple_usage

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

## Coding Standards

### Rust Style Guide

We follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html) and use `rustfmt` for formatting.

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Linting

We use `clippy` for additional linting:

```bash
# Run clippy
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

### Naming Conventions

- **Functions**: `snake_case`
- **Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Types**: `PascalCase`
- **Modules**: `snake_case`

### Documentation

All public APIs must be documented:

```rust
/// Parse a demo file and extract all events
///
/// This method reads a demo file from the filesystem and parses it to extract
/// all game events including kills, headshots, clutches, and rounds.
///
/// # Arguments
///
/// * `path` - Path to the demo file (.dem)
///
/// # Returns
///
/// Returns a `Result<DemoEvents>` containing all parsed events and statistics,
/// or an error if parsing fails.
///
/// # Examples
///
/// ```rust
/// use cs2_demo_core::CS2DemoCore;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let demo_core = CS2DemoCore::new();
///     let events = demo_core.parse_file("match.dem").await?;
///     println!("Parsed {} kills", events.kills.len());
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// This method can return various errors:
///
/// - `DemoError::FileNotFound` - Demo file doesn't exist
/// - `DemoError::InvalidFormat` - File is not a valid CS2 demo
/// - `DemoError::Corrupted` - Demo file is corrupted
pub async fn parse_file(&self, path: &str) -> Result<DemoEvents> {
    // Implementation...
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_parser_creation

# Run integration tests
cargo test --test integration_tests

# Run benchmarks
cargo bench
```

### Writing Tests

All new functionality must include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = CS2Parser::new();
        assert!(parser.is_some());
    }

    #[tokio::test]
    async fn test_parse_demo_file() {
        let demo_core = CS2DemoCore::new();
        let result = demo_core.parse_file("test.dem").await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_handling() {
        let result = parse_invalid_data();
        assert!(matches!(result, Err(DemoError::InvalidFormat { .. })));
    }
}
```

### Test Data

- Use small, focused test files
- Include both valid and invalid demo files
- Test edge cases and error conditions
- Mock external dependencies when appropriate

### Integration Tests

Create integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use cs2_demo_core::CS2DemoCore;

#[tokio::test]
async fn test_full_demo_parsing() {
    let demo_core = CS2DemoCore::new();
    let events = demo_core.parse_file("tests/data/sample.dem").await.unwrap();
    
    assert!(!events.kills.is_empty());
    assert!(!events.rounds.is_empty());
    assert!(!events.players.is_empty());
}
```

## Documentation

### Code Documentation

- Document all public APIs
- Include examples in documentation
- Document error conditions
- Keep documentation up to date

### README Updates

When adding new features, update the README:

- Add new examples
- Update feature list
- Update API documentation
- Add performance notes if relevant

### Changelog

Update `CHANGELOG.md` for all changes:

```markdown
## [Unreleased]

### Added
- New feature for parsing demo metadata
- Support for custom parsing options

### Changed
- Improved error messages for invalid demos
- Updated minimum Rust version to 1.70

### Fixed
- Fixed memory leak in large demo parsing
- Corrected headshot detection logic
```

## Pull Request Process

### Before Submitting

1. **Fork and clone** the repository
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following the coding standards
4. **Add tests** for new functionality
5. **Update documentation** as needed
6. **Run all checks**:
   ```bash
   cargo check
   cargo test
   cargo fmt --check
   cargo clippy
   ```

### Commit Messages

Use conventional commit messages:

```
feat: add support for custom parsing options
fix: resolve memory leak in large demo parsing
docs: update README with new examples
test: add integration tests for error handling
refactor: improve error handling structure
```

### Pull Request Guidelines

1. **Title**: Clear, descriptive title
2. **Description**: Detailed description of changes
3. **Related Issues**: Link to related issues
4. **Testing**: Describe how you tested the changes
5. **Breaking Changes**: Note any breaking changes

Example PR description:

```markdown
## Description

Adds support for custom parsing options to allow users to control which events are extracted from demo files.

## Changes

- Added `ParseOptions` struct with configuration options
- Updated `CS2DemoCore::parse_file_with_options()` method
- Added validation for parsing options
- Updated documentation with examples

## Testing

- Added unit tests for `ParseOptions` validation
- Added integration tests for custom parsing
- Tested with various demo files

## Breaking Changes

None - this is a backward-compatible addition.

Closes #123
```

### Review Process

1. **Automated Checks**: All CI checks must pass
2. **Code Review**: At least one maintainer must approve
3. **Testing**: Changes must include appropriate tests
4. **Documentation**: Documentation must be updated

## Issue Reporting

### Bug Reports

When reporting bugs, include:

- **Description**: Clear description of the problem
- **Steps to Reproduce**: Detailed steps to reproduce
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: OS, Rust version, demo file info
- **Minimal Example**: Minimal code to reproduce

Example bug report:

```markdown
## Bug Description

The parser fails to extract headshots from demo files with certain weapon types.

## Steps to Reproduce

1. Download demo file: [link]
2. Run: `cargo run --example simple_usage`
3. Observe: No headshots are detected

## Expected Behavior

Should detect and extract headshot events from the demo.

## Actual Behavior

Returns empty headshots vector.

## Environment

- OS: Ubuntu 22.04
- Rust: 1.70.0
- Demo: match730_003771140455228506223_0773145705_202.dem

## Minimal Example

```rust
use cs2_demo_core::CS2DemoCore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    let events = demo_core.parse_file("bug_demo.dem").await?;
    println!("Headshots: {}", events.headshots.len()); // Prints 0, should be > 0
    Ok(())
}
```
```

### Feature Requests

When requesting features, include:

- **Description**: Clear description of the feature
- **Use Case**: Why this feature is needed
- **Proposed API**: How you envision the API
- **Alternatives**: Any alternatives you considered

## Getting Help

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Use GitHub Issues for bugs and feature requests
- **Documentation**: Check the README and API docs first

## Recognition

Contributors will be recognized in:

- The README file
- Release notes
- The project's contributors list

Thank you for contributing to CS2 Demo Core! 🎯
