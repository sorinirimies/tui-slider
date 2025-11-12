# Release Workflow

This document describes the release workflow for `tui-slider`.

## Prerequisites

Install required tools:

```bash
just install-tools
```

This installs:
- `just` - Command runner
- `git-cliff` - Changelog generator

## Release Steps

### Option 1: Manual Step-by-Step

1. **Run all checks**:
   ```bash
   just check-all
   ```

2. **Preview unreleased changes**:
   ```bash
   just changelog-preview-unreleased
   ```

3. **Prepare and push release** (bumps version, updates changelog, creates tag, and pushes):
   ```bash
   just release 0.2.0
   ```

4. **Publish to crates.io**:
   ```bash
   just publish
   ```

### Option 2: Full Automated Release

Run everything in one command:

```bash
just release-full 0.2.0
```

This will:
- Run all checks (fmt, clippy, tests)
- Build release version
- Bump version in `Cargo.toml` and `Cargo.lock`
- Update `CHANGELOG.md`
- Create git commit and tag
- Push commits and tags to remote
- Publish to crates.io

## What the Release Process Does

### `just release <version>`

1. **Pre-flight checks**:
   - Runs `cargo fmt --check`
   - Runs `cargo clippy` with strict warnings
   - Runs all tests
   - Builds release binary

2. **Version bump**:
   - Updates version in `Cargo.toml`
   - Updates `Cargo.lock`

3. **Changelog generation**:
   - Uses `git-cliff` to generate `CHANGELOG.md`
   - Groups changes by type (Features, Bug Fixes, etc.)
   - Follows conventional commits

4. **Git operations**:
   - Creates commit: `chore(release): bump version to X.Y.Z`
   - Creates annotated tag: `vX.Y.Z`
   - Pushes main branch to origin
   - Pushes the version tag to origin

### `just push-release`

- Pushes main branch to origin
- Pushes all tags to origin
- (Note: `just release` already pushes automatically)

### `just publish`

- Publishes the crate to crates.io

## Useful Commands

### Development

```bash
# Run all checks before committing
just check-all

# Run specific example
just example-horizontal

# Run all examples
just examples

# Watch and run tests
just watch-test
```

### Changelog Management

```bash
# Preview unreleased changes
just changelog-preview-unreleased

# Preview full changelog
just changelog-preview

# Generate changelog without releasing
just changelog

# View current changelog
just view-changelog
```

### Version Management

```bash
# Show current version
just version

# Show project info
just info
```

### Testing Releases

```bash
# Dry-run publish (doesn't actually publish)
just publish-dry
```

## Troubleshooting

### git-cliff not found

Install it:
```bash
cargo install git-cliff
```

Or use the helper:
```bash
just install-tools
```

### Version bump failed

The `bump` command updates `Cargo.toml` using `sed`. Make sure you're on Linux/macOS.

For manual version bump:
1. Edit `Cargo.toml` - change `version = "X.Y.Z"`
2. Run `cargo build` to update `Cargo.lock`
3. Run `just changelog-version X.Y.Z`
4. Commit and tag manually

### Release checks failed

Fix the issues reported by:
- `cargo fmt` - Run `just fmt` to auto-fix
- `cargo clippy` - Fix warnings manually
- `cargo test` - Fix failing tests

## Conventional Commits

The changelog generator uses conventional commits. Use these prefixes:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `test:` - Test changes
- `chore:` - Maintenance tasks
- `ci:` - CI/CD changes

Examples:
```bash
git commit -m "feat: add show_thumb() method"
git commit -m "fix: correct slider rendering bug"
git commit -m "docs: update README with examples"
```

## Release Checklist

Before releasing:

- [ ] All tests pass (`just test`)
- [ ] Code is formatted (`just fmt`)
- [ ] No clippy warnings (`just clippy`)
- [ ] Examples work (`just examples`)
- [ ] README is up to date
- [ ] CHANGELOG preview looks good (`just changelog-preview-unreleased`)

After releasing:

- [ ] Verify tag is pushed to GitHub (`git ls-remote --tags origin`)
- [ ] Check GitHub releases page
- [ ] Verify crate on crates.io (after running `just publish`)
- [ ] Test installation: `cargo install tui-slider --version X.Y.Z`

## GitHub Actions

The repository uses GitHub Actions for CI/CD:

- **CI Workflow**: Runs on every push/PR
  - Tests on Linux, macOS, Windows
  - Tests with stable and nightly Rust
  - Runs fmt, clippy, tests, docs, examples

- **Release Workflow**: Triggers on version tags
  - Publishes to crates.io automatically
  - Creates GitHub release

## Version Numbers

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.0.1): Bug fixes, backwards compatible

Current version: `0.1.0` (initial release)