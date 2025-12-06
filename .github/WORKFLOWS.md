# GitHub Workflows Documentation

This document describes the GitHub Actions workflows for tui-slider.

## Overview

The project uses two main workflows:
1. **CI** - Continuous Integration for pull requests and pushes
2. **Release** - Automated releases when version tags are pushed

Both workflows are simplified and match the structure from tui-checkbox for consistency.

## CI Workflow

**File:** `.github/workflows/ci.yml`

**Triggers:**
- Pull requests to any branch
- Pushes to `main`, `master`, or `develop` branches

**Features:**
- ✅ Concurrency control (cancels outdated PR runs)
- ✅ Smart caching with `Swatinem/rust-cache`
- ✅ Tests on Ubuntu and Windows
- ✅ Clippy feedback directly on PRs

### Jobs

#### 1. `fmt` - Code Formatting
- Runs on: Ubuntu Latest
- Checks: `cargo fmt --check`
- Purpose: Ensures code follows Rust formatting standards

#### 2. `clippy` - Linting
- Runs on: Ubuntu Latest
- Uses: `clechasseur/rs-clippy-check` action
- Purpose: Provides inline PR comments for clippy warnings
- Permissions: Requires `contents: read` and `checks: write`

#### 3. `doc` - Documentation
- Runs on: Ubuntu Latest
- Uses: Rust nightly toolchain
- Checks: `cargo doc --no-deps --all-features`
- Purpose: Ensures documentation builds without errors
- Environment: `RUSTDOCFLAGS: --cfg docsrs`

#### 4. `test` - Tests
- Runs on: Ubuntu Latest + Windows Latest
- Matrix strategy with fail-fast disabled
- Commands:
  - `cargo generate-lockfile` (if Cargo.lock missing)
  - `cargo test --locked --all-features --all-targets`
- Purpose: Runs all unit and integration tests

### Why These Choices?

- **Ubuntu + Windows only**: Most users are on these platforms
- **Nightly for docs only**: Enables `#[doc(cfg)]` features
- **Clippy check action**: Better PR experience with inline comments
- **Swatinem/rust-cache**: Faster builds (saves ~2-3 minutes)
- **Concurrency control**: Saves CI minutes by canceling old runs

## Release Workflow

**File:** `.github/workflows/release.yml`

**Triggers:**
- Push of tags matching `v*` (e.g., `v0.1.0`, `v0.2.0`)

**Features:**
- ✅ Automatic changelog generation with git-cliff
- ✅ Rich release notes with installation guide
- ✅ Auto-publish to crates.io (when token is set)
- ✅ GitHub Release with files attached

### Jobs

#### 1. `test` - Pre-Release Testing
- Runs on: Ubuntu Latest + Windows Latest
- Runs all quality checks:
  - Code formatting (`cargo fmt --check`)
  - Clippy linting (`cargo clippy`)
  - All tests (`cargo test --all-features`)
- Purpose: Ensures release quality before publishing

#### 2. `build` - Build and Release
- Runs on: Ubuntu Latest
- Depends on: `test` job passing
- Steps:
  1. **Update Version**: Extracts version from tag and updates `Cargo.toml`
  2. **Generate Changelog**: Uses git-cliff to create `CHANGELOG.md`
  3. **Create Release Notes**: Generates rich markdown with:
     - What's new section
     - Changes since last version
     - Installation instructions
     - Quick start example
  4. **Build Release**: `cargo build --release --all-features`
  5. **Create GitHub Release**: Uses `softprops/action-gh-release`
  6. **Publish to crates.io**: Auto-publishes if `CRATES_IO_TOKEN` is set

### Release Notes Format

Each release includes:

```markdown
# tui-slider X.Y.Z

## 🚀 What's New

### 📝 Changes since vX.Y.Z:
- [Generated changelog from git-cliff]

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tui-slider = "X.Y.Z"
```

Or install with cargo:

```bash
cargo add tui-slider
```

## 🚀 Quick Start

```rust
use ratatui::prelude::*;
use tui_slider::{Slider, SliderState, SliderOrientation};

let mut state = SliderState::new(50.0, 0.0, 100.0);
let slider = Slider::from_state(&state)
    .orientation(SliderOrientation::Horizontal)
    .label("Volume")
    .show_value(true);
```
```

## How to Release

### Automated (Recommended)

Use the justfile commands:

```bash
# Full release (bump version, generate changelog, push)
just release 0.2.0

# Then publish to crates.io
just publish
```

The `just release` command:
1. Runs all CI checks locally
2. Updates `Cargo.toml` and `Cargo.lock`
3. Generates `CHANGELOG.md` with git-cliff
4. Creates commit: `chore(release): bump version to X.Y.Z`
5. Creates tag: `vX.Y.Z`
6. Pushes everything to GitHub

This triggers the Release workflow which:
1. Runs tests on Ubuntu and Windows
2. Builds the release
3. Creates GitHub Release with rich notes
4. Publishes to crates.io (if `CRATES_IO_TOKEN` is set)

### Manual

```bash
# 1. Update version in Cargo.toml
sed -i 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml

# 2. Generate changelog
git-cliff --tag v0.2.0 -o CHANGELOG.md

# 3. Commit and tag
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore(release): bump version to 0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"

# 4. Push
git push origin main
git push origin v0.2.0
```

The Release workflow will handle the rest.

## Required Secrets

### CRATES_IO_TOKEN (Optional)

To enable automatic publishing to crates.io:

1. Go to https://crates.io/settings/tokens
2. Create a new token with `publish-update` scope
3. Add it to GitHub:
   - Repository Settings → Secrets and variables → Actions
   - New repository secret
   - Name: `CRATES_IO_TOKEN`
   - Value: Your token

**Note:** If this secret is not set, the workflow will skip crates.io publishing and just create the GitHub Release.

## Workflow Files

```
.github/workflows/
├── ci.yml        # Continuous Integration
└── release.yml   # Release Automation
```

## Caching Strategy

Both workflows use `Swatinem/rust-cache@v2` which caches:
- Cargo registry (`~/.cargo/registry`)
- Cargo index (`~/.cargo/git`)
- Build artifacts (`target/`)

This reduces CI time from ~5 minutes to ~2 minutes after the first run.

## Concurrency

The CI workflow uses concurrency control:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.head_ref || github.run_id }}
  cancel-in-progress: true
```

This means:
- Multiple pushes to the same PR will cancel older runs
- Saves CI minutes
- Provides faster feedback

## Troubleshooting

### CI Failing on Formatting

Run locally:
```bash
just fmt
git add -A
git commit -m "style: format code"
```

### CI Failing on Clippy

Run locally:
```bash
just clippy
# Fix issues manually
```

### Release Not Publishing to crates.io

Check:
1. Is `CRATES_IO_TOKEN` secret set?
2. Does the token have `publish-update` scope?
3. Check workflow logs for errors

### Release Notes Not Generated

The release workflow uses git-cliff which requires:
1. Conventional commit messages
2. Proper git history
3. At least one commit since the last tag

## Best Practices

1. **Use conventional commits** for better changelogs:
   ```
   feat: add new feature
   fix: correct bug
   docs: update documentation
   ```

2. **Run CI checks locally** before pushing:
   ```bash
   just check-all  # or just ci
   ```

3. **Review release notes** on GitHub after releasing

4. **Test releases** with `just publish-dry` first

## Comparison with tui-checkbox

These workflows are based on tui-checkbox's proven approach:

- ✅ Same job structure
- ✅ Same caching strategy
- ✅ Same release note format
- ✅ Same clippy-check integration
- ✅ Same concurrency control

This ensures consistency across projects and makes maintenance easier.

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [rust-cache Action](https://github.com/Swatinem/rust-cache)
- [git-cliff](https://github.com/orhun/git-cliff)
- [softprops/action-gh-release](https://github.com/softprops/action-gh-release)