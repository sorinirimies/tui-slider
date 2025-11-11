# Contributing to tui-slider

Thank you for your interest in contributing to tui-slider! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## How to Contribute

### Reporting Bugs

Before creating a bug report:
- Check the issue tracker to avoid duplicates
- Collect relevant information (OS, terminal, Rust version)
- Create a minimal reproducible example if possible

When filing a bug report, include:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Code samples or screenshots

### Suggesting Features

For feature requests:
- Check existing issues and discussions
- Clearly describe the use case
- Provide examples of how it would work
- Consider backwards compatibility

### Pull Requests

1. **Fork and Clone**
   ```bash
   git clone https://github.com/sorinirimies/tui-slider.git
   cd tui-slider
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/my-new-feature
   ```

3. **Make Changes**
   - Write clear, idiomatic Rust code
   - Follow existing code style
   - Add tests for new functionality
   - Update documentation as needed

4. **Run Tests**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. **Commit Changes**
   - Use conventional commit messages
   - Examples: `feat: add gradient color support`, `fix: handle edge case in animation`

6. **Push and Create PR**
   ```bash
   git push origin feature/my-new-feature
   ```
   Then create a PR on GitHub

## Development Setup

### Prerequisites

- Rust 1.74.0 or later
- A terminal that supports Unicode (for full visual testing)

### Building

```bash
# Build the library
cargo build

# Build with all features
cargo build --all-features

# Build examples
cargo build --examples
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Running Examples

```bash
# Run specific example
cargo run --example horizontal

# Or use just
just example-horizontal
```

### Using Just

This project includes a `justfile` for common tasks:

```bash
# List all commands
just

# Run checks (format, clippy, test)
just check

# Format code
just fmt

# Run clippy
just clippy

# Build documentation
just doc
```

## Code Style

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` to catch common mistakes
- Write idiomatic Rust code

### Documentation

- Add doc comments for all public items
- Include examples in doc comments
- Use `///` for item documentation
- Use `//!` for module documentation
- Run `cargo doc --open` to preview

Example:
```rust
/// Creates a new slider with the given value and bounds
///
/// # Arguments
///
/// * `value` - Initial value
/// * `min` - Minimum value
/// * `max` - Maximum value
///
/// # Example
///
/// ```
/// use tui_slider::Slider;
/// let slider = Slider::new(50.0, 0.0, 100.0);
/// ```
pub fn new(value: f64, min: f64, max: f64) -> Self {
    // ...
}
```

### Testing

- Write unit tests for new functionality
- Add integration tests where appropriate
- Test edge cases and error conditions
- Aim for good test coverage

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_bounds() {
        let slider = Slider::new(150.0, 0.0, 100.0);
        assert_eq!(slider.value(), 100.0);
    }
}
```

## Project Structure

```
tui-slider/
├── src/
│   ├── lib.rs           # Main library entry point
│   ├── slider.rs        # Main slider widget
│   ├── state.rs         # State management
│   ├── orientation.rs   # Orientation types
│   ├── symbols.rs       # Symbol sets
│   ├── styles.rs        # Visual styles
│   ├── colors.rs        # Color gradients
│   └── animation.rs     # Animation system
├── examples/            # Example applications
├── tests/               # Integration tests
└── benches/             # Benchmarks (future)
```

## Adding New Features

### New Slider Style

1. Add the style definition in `src/styles.rs`
2. Create a preset method (e.g., `pub fn my_style()`)
3. Add tests for the new style
4. Document with examples
5. Consider adding an example in `examples/`

### New Symbol Set

1. Add symbols in `src/symbols.rs`
2. Create preset methods for handle and bar
3. Add to the `SliderSymbols` presets
4. Test the symbols render correctly
5. Update documentation

### New Animation Easing

1. Add the easing variant to `AnimationEasing` enum in `src/animation.rs`
2. Implement the easing function in the `apply()` method
3. Add tests for the easing function
4. Document the easing behavior

## Examples

When adding examples:
- Place in `examples/` directory
- Include clear comments explaining functionality
- Add keyboard controls help text
- Register in `Cargo.toml` under `[[example]]`
- Update README with description and run command

## Documentation

### README Updates

When adding significant features:
- Update feature list
- Add examples showing usage
- Update API sections
- Add to appropriate sections

### CHANGELOG

Follow [Keep a Changelog](https://keepachangelog.com/):
- Add entries under `[Unreleased]`
- Use categories: Added, Changed, Deprecated, Removed, Fixed, Security
- Write clear, user-focused descriptions

## Release Process

Maintainers only:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v0.x.x -m "Release v0.x.x"`
4. Push tag: `git push origin v0.x.x`
5. CI will build and publish to crates.io

## Questions?

- Open an issue for questions
- Check existing documentation
- Review examples for usage patterns

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to tui-slider! 🎚️