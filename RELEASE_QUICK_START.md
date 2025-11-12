# Release Quick Start Guide

Quick reference for releasing new versions of tui-slider.

## TL;DR - One Command Release

```bash
# Complete automated release (checks + bump + push + publish)
just release-full 0.2.0
```

## Step-by-Step Release

### 1. Check Everything Works

```bash
just check-all
```

This runs:
- Code formatting check
- Clippy lints
- All tests (unit + doc tests)

### 2. Preview Changes

```bash
just changelog-preview-unreleased
```

See what will be in the changelog.

### 3. Prepare and Push Release

```bash
just release 0.1.1
```

This will:
- ✅ Run all checks
- 📝 Update version in `Cargo.toml` and `Cargo.lock`
- 📋 Generate `CHANGELOG.md`
- 💾 Create git commit
- 🏷️ Create git tag `v0.1.1`
- 🚀 Push commits and tags to origin

### 4. Publish to crates.io

```bash
# Dry run first (optional)
just publish-dry

# Actually publish
just publish
```

## Quick Commands

```bash
# Current version
just version

# Project info
just info

# Release help
just help-release

# All available commands
just --list
```

## Common Tasks

### Preview unreleased changes
```bash
just changelog-preview-unreleased
```

### Run CI checks locally
```bash
just ci
```

### Run all examples
```bash
just examples
```

### Format code
```bash
just fmt
```

### Fix common issues
```bash
# Format code
just fmt

# Run checks
just check-all

# Build release
just build-release
```

## Troubleshooting

### Tests failing
```bash
cargo test --all-features -- --nocapture
```

### Clippy warnings
```bash
cargo clippy --fix --allow-dirty
```

### Need to undo version bump
```bash
# If already pushed, you'll need to force push (be careful!)
# First, undo locally
git reset --hard HEAD~1

# Delete the tag locally
git tag -d v0.1.1

# Delete the tag from remote
git push origin :refs/tags/v0.1.1

# Force push main (only if necessary and safe)
git push origin main --force
```

## Version Numbers (Semantic Versioning)

- **MAJOR** (1.0.0): Breaking API changes
- **MINOR** (0.2.0): New features, backwards compatible
- **PATCH** (0.1.1): Bug fixes, backwards compatible

Examples:
```bash
just release 0.1.1  # Bug fix
just release 0.2.0  # New feature
just release 1.0.0  # Breaking changes
```

## Note

The `just release` command now **automatically pushes** to GitHub. If you want to review before pushing, you can manually run the steps:

```bash
# Manual workflow (without auto-push)
just release-check           # Run checks only
# Then manually bump version and push when ready