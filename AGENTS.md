# tui-slider Development Rules

## Project Overview
- **Name**: tui-slider
- **Type**: Rust library for ratatui
- **Version**: 0.3.0
- **License**: MIT
- **Purpose**: A highly customizable slider widget for terminal UI applications
- **MSRV**: Rust 1.74.0

## Key Dependencies
- `ratatui` (0.30): TUI framework
- `crossterm` (0.29): Terminal backend
- `unicode-width` (0.2): Unicode character width handling

## Project Structure
```
src/
├── lib.rs              # Main library export
├── slider.rs           # Core slider widget
├── state.rs            # SliderState management
├── orientation.rs      # Orientation types (Horizontal/Vertical)
├── symbols.rs          # Symbol definitions and presets
├── style.rs            # Visual styles
├── border.rs           # Border styling
├── position.rs         # Position enums

examples/               # 10+ interactive examples with different styles
justfile                # Task runner with comprehensive commands
scripts/                # Setup and release automation
```

## Code Style Rules

### Rust Standards
1. **Formatting**: Use `cargo fmt` (enforced)
   - Run before commits: `just fmt`
   - Check in CI: `just fmt-check`

2. **Linting**: Use `cargo clippy` with all warnings as errors
   - Command: `just clippy`
   - Must pass: `cargo clippy -- -D warnings`

3. **Idiomatic Rust**
   - Follow Rust API Guidelines
   - Use meaningful variable names
   - Prefer pattern matching over if-else chains
   - Leverage type system for safety

4. **Documentation**
   - All public items must have doc comments (///)
   - Include examples in doc comments for public API
   - Use //! for module-level documentation
   - Run `cargo doc --no-deps --open` to preview

5. **Testing**
   - Write unit tests for new functionality
   - Test edge cases (min/max bounds, zero values, negative ranges)
   - Include integration tests for complex features
   - Example test pattern:
     ```rust
     #[cfg(test)]
     mod tests {
         use super::*;
         
         #[test]
         fn test_feature_description() {
             // Arrange
             let state = SliderState::new(50.0, 0.0, 100.0);
             // Act
             // Assert
         }
     }
     ```

## Development Workflow

### Setup
```bash
./scripts/setup-just.sh      # Interactive setup
cargo install just           # Manual install
```

### Common Tasks
```bash
just build                    # Build library
just test                     # Run all tests
just check-all                # fmt + clippy + test (pre-commit)
just run                      # Run horizontal example
just examples                 # Run all examples
just fmt                      # Auto-format code
just clippy                   # Run linter
just doc                      # Generate and open docs
```

### Code Quality Gate
**Before committing**, run:
```bash
just check-all
```

This ensures:
- Code is properly formatted
- No clippy warnings
- All tests pass
- Project builds successfully

## Commit Guidelines

### Conventional Commits
Use conventional commit format for all commits:
```
<type>(<scope>): <subject>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting/style changes (non-functional)
- `refactor`: Code refactoring (no feature change)
- `perf`: Performance improvements
- `test`: Adding/updating tests
- `chore`: Build, dependencies, tooling
- `ci`: CI/CD configuration

**Scope** (optional):
- `slider`: Slider widget
- `state`: State management
- `style`: Styling/colors
- `border`: Border styling
- `symbols`: Symbol definitions
- `examples`: Example applications
- `docs`: Documentation
- `release`: Version/release related

**Examples**:
```
feat(slider): add gradient fill support
fix(state): handle edge case when value exceeds max
docs: update API overview in README
test(state): add bounds validation tests
refactor(symbols): extract symbol presets
perf(render): optimize horizontal slider rendering
```

**Good commit message**:
- Subject < 50 characters
- Imperative mood ("add" not "adds" or "added")
- No period at end
- Body (if needed): explain why, not what
- Link issues: `fixes #123`, `relates to #456`

### Example Commit
```
feat(slider): add gradient color support

Add support for gradient colors on slider bars.
Users can now define color transitions for filled
and empty sections separately.

- Add GradientColor type
- Add gradient_color() method
- Update examples with gradient demo

fixes #42
```

## Pull Request Guidelines

### Before Creating PR
1. Create feature branch: `git checkout -b feature/my-feature`
2. Run quality checks: `just check-all`
3. Update tests if needed
4. Update docs if needed

### PR Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Refactoring
- [ ] Performance improvement

## Testing
How to test the changes?

## Checklist
- [ ] Code follows style guidelines (ran `just fmt`)
- [ ] Linter passes (ran `just clippy`)
- [ ] Tests pass (ran `just test`)
- [ ] Documentation updated
- [ ] New tests added (if applicable)
- [ ] CHANGELOG.md updated (for features/fixes)
- [ ] No breaking changes (or clearly documented)

## Related Issues
Closes #123
```

### PR Best Practices
- Keep PRs focused (one feature per PR)
- Keep commits logically separated
- Provide clear PR descriptions
- Link related issues
- Ensure all CI checks pass

## Version Management

### Version Bumping
Use the bump command (automatically runs checks):
```bash
just bump 0.4.0
```

This will:
1. Run all checks (fmt, clippy, test)
2. Update version in Cargo.toml
3. Create git tag
4. Commit changes

### Release Process
After version bump:
```bash
git push origin main              # Push to GitHub
git push origin v0.4.0            # Push tag
git push gitea main               # Push to Gitea (if needed)
git push gitea v0.4.0
```

Or use combined command:
```bash
just release 0.4.0                # GitHub only
just release-all 0.4.0            # GitHub + Gitea
```

## Changelog Management

### CHANGELOG Format
Follow [Keep a Changelog](https://keepachangelog.com/):
- Categories: Added, Changed, Deprecated, Removed, Fixed, Security
- Add entries under `[Unreleased]`
- Clear, user-focused descriptions

### Example Entry
```markdown
## [Unreleased]

### Added
- Gradient color support for slider bars
- New `gradient_color()` method on Slider

### Fixed
- Handle edge case when value exceeds max bounds
- Incorrect vertical alignment in some terminals

### Changed
- Improved performance of horizontal slider rendering
```

### Changelog Commands
```bash
just changelog-unreleased         # Show unreleased changes
just changelog                    # Regenerate full changelog
just changelog-preview            # Preview without writing
just changelog-version 0.4.0      # For specific version
```

## Testing Guidelines

### Running Tests
```bash
just test                         # Run all tests
cargo test test_name              # Run specific test
cargo test -- --nocapture         # Show output
cargo test --doc                  # Run doc tests
```

### Test Coverage
```bash
cargo tarpaulin --out Html --output-dir coverage
```

### What to Test
1. **Unit Tests** - Individual functions/methods
2. **Edge Cases** - Boundaries, empty inputs, extreme values
3. **State Changes** - Value updates, bounds validation
4. **Rendering** - Widget output correctness
5. **Integration** - Widget with ratatui applications

## Documentation Guidelines

### Doc Comments
```rust
/// Brief one-line description
///
/// Longer explanation if needed, with more details
/// about the behavior and use cases.
///
/// # Arguments
///
/// * `param1` - Description
/// * `param2` - Description
///
/// # Returns
///
/// Description of return value
///
/// # Example
///
/// ```
/// use tui_slider::Slider;
/// let slider = Slider::new(50.0, 0.0, 100.0);
/// ```
pub fn my_function(param1: T, param2: U) -> V {
    // Implementation
}
```

### Examples
- Place in `examples/` directory
- Include clear comments
- Add keyboard controls help text
- Register in Cargo.toml as `[[example]]`
- Update README with description

## Feature Development

### Adding a New Feature

1. **Create feature branch**
   ```bash
   git checkout -b feature/feature-name
   ```

2. **Implement feature**
   - Write code following style guidelines
   - Add tests
   - Update doc comments
   - Run checks: `just check-all`

3. **Update documentation**
   - Add/update doc comments
   - Create/update example if needed
   - Update README sections
   - Add entry to CHANGELOG

4. **Commit and push**
   ```bash
   git add .
   git commit -m "feat(scope): description"
   git push origin feature/feature-name
   ```

5. **Create pull request**
   - Fill PR template
   - Link related issues
   - Wait for CI and review

### New Slider Style Example
1. Define style in `src/style.rs`
2. Create preset method: `pub fn my_style() -> Self`
3. Add tests for rendering
4. Create example in `examples/`
5. Document in README
6. Commit: `feat(style): add my_style preset`

## Continuous Integration

### CI Checks (GitHub Actions)
- **ci.yml**: Build, test, clippy, fmt
- **release.yml**: Automated release on version tags

All checks must pass before merging:
- ✅ `cargo build`
- ✅ `cargo fmt --check`
- ✅ `cargo clippy -- -D warnings`
- ✅ `cargo test`
- ✅ Documentation builds

## Performance Considerations

1. **Rendering** - Optimize for large terminal sizes
2. **State Updates** - Minimize redraws
3. **Memory** - Keep state structures lean
4. **Unicode** - Use unicode-width for accurate positioning

## Common Tasks Reference

### Daily Development
```bash
just build                    # Quick build check
just test                     # Run tests
just run                      # Test with example
```

### Before Committing
```bash
just check-all                # Format, lint, test (required)
```

### Before Creating PR
```bash
just check-all                # Run all checks
cargo test -- --nocapture     # Test with output
just doc                      # Verify docs build
```

### Release Prep
```bash
# Update version
just bump 0.4.0

# Generate changelog
just changelog-unreleased

# Push release
just release-all 0.4.0
```

## File Modification Rules

### When Modifying Core Files

**`src/slider.rs`** (Core widget)
- Update doc comments
- Add tests for changes
- Update examples if API changes
- Update README if behavior changes

**`src/state.rs`** (State management)
- Add tests for validation
- Document state transitions
- Update examples if needed

**`examples/`** (Examples)
- Keep simple and focused
- Include comments
- Test before committing

**`CHANGELOG.md`**
- Add entry for all user-facing changes
- Use Changelog format
- Never remove previous entries

**`Cargo.toml`**
- Update version before release only
- Use semantic versioning
- Document dependency changes

## Git Remote Management

### Available Remotes
```bash
just remotes              # Show all remotes
```

### Push Operations
```bash
just push                 # Push to GitHub (origin)
just push-gitea           # Push to Gitea
just push-all             # Push to both
just push-tags            # Push tags to GitHub
just push-tags-all        # Push tags to both
```

## Quick Troubleshooting

### Tests Failing
```bash
cargo test -- --nocapture --test-threads=1
```

### Clippy Issues
```bash
cargo clippy --fix        # Auto-fix where possible
cargo clippy -- -D warnings  # Check all warnings
```

### Format Issues
```bash
cargo fmt                 # Auto-format everything
cargo fmt --check         # Check without modifying
```

### Build Issues
```bash
cargo clean
cargo build               # Rebuild from scratch
```

## Resources

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Ratatui Documentation](https://docs.rs/ratatui)
- [This Project README](../README.md)
- [Contributing Guide](../CONTRIBUTING.md)

## Contact & Support

- **Repository**: https://github.com/sorinirimies/tui-slider
- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Documentation**: https://docs.rs/tui-slider
- **Crates.io**: https://crates.io/crates/tui-slider
