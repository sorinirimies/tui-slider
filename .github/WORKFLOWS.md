# GitHub Workflows Documentation

This document describes the GitHub Actions workflows for tui-slider.

## Overview

The project uses four workflows:
1. **CI** - Continuous Integration for pull requests and pushes
2. **Nightly Dependency Update** - Library-only upgrades with downgrade protection and auto-merge
3. **Auto Release** - Patch-version bump after an automated dependency PR merges
4. **Release** - Automated release build when a version tag is pushed

All workflows are available for GitHub Actions (`.github/workflows/`) and Gitea Actions (`.gitea/workflows/`).

Release logic is extracted into reusable Nushell scripts under `scripts/`, keeping the workflow YAML thin and the logic testable locally.

## CI Workflow

**File:** `.github/workflows/ci.yml` / `.gitea/workflows/ci.yml`

**Triggers:**
- Pull requests to any branch
- Pushes to `main`, `master`, or `develop` branches

**Features:**
- ✅ Concurrency control (cancels outdated PR runs)
- ✅ Smart caching with `Swatinem/rust-cache`
- ✅ Clippy feedback directly on PRs
- ✅ Rust 1.88 MSRV verification
- ✅ Nightly docs build with strict warning checks and `#[doc(cfg)]` support

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

#### 3. `msrv` - Minimum Supported Rust Version
- Runs on: Ubuntu Latest
- Uses: Rust 1.88.0
- Checks: `cargo check --locked --lib`
- Purpose: Keeps `package.rust-version` truthful

#### 4. `doc` - Documentation
- Runs on: Ubuntu Latest
- Uses: Rust nightly toolchain
- Checks: `cargo doc --no-deps --all-features`
- Purpose: Ensures documentation builds without warnings or broken links
- Environment: `RUSTDOCFLAGS: --cfg docsrs -D warnings`

#### 5. `test` - Tests
- Runs on: Ubuntu Latest
- Commands:
  - `cargo generate-lockfile` (if Cargo.lock missing)
  - `cargo test --locked --all-features --all-targets`
- Purpose: Runs all unit and integration tests

### Why These Choices?

- **Nightly for docs only**: Enables `#[doc(cfg)]` features
- **Clippy check action**: Better PR experience with inline comments
- **Swatinem/rust-cache**: Faster builds (saves ~2-3 minutes)
- **Concurrency control**: Saves CI minutes by canceling old runs

## Nightly Dependency Update

**File:** `.github/workflows/nightly-update.yml` / `.gitea/workflows/nightly-update.yml`

Every night, each workflow:

1. Runs `cargo upgrade --incompatible allow --ignore-rust-version` so `cargo-edit` cannot lower an already-newer requirement to satisfy the declared MSRV.
2. Runs `cargo update --verbose` and fails if Cargo reports any downgrade.
3. Allows changes only in `Cargo.toml` and `Cargo.lock`.
4. Runs formatting, Clippy, tests, and a locked all-target build.
5. Opens or updates `chore/nightly-dependency-update` and merges it after the workflow's quality gate passes.
6. Explicitly dispatches `auto-release.yml` with the built-in workflow credential.

GitHub runs at 03:00 UTC. Gitea runs at 03:30 UTC to reduce duplicate work when both repositories are synchronized mirrors.

## Auto Release

**File:** `.github/workflows/auto-release.yml` / `.gitea/workflows/auto-release.yml`

The nightly workflow explicitly dispatches this workflow after confirming the dependency PR merged. It updates release metadata, commits, creates an annotated `vX.Y.Z` tag, pushes it, then explicitly dispatches `release.yml`. Explicit dispatches avoid GitHub's built-in-credential workflow-chain suppression without requiring another PAT.

## Release Workflow

**File:** `.github/workflows/release.yml` / `.gitea/workflows/release.yml`

**Triggers:**
- Push of tags matching `v*` (e.g., `v0.3.0`, `v0.4.0`)
- Explicit dispatch from `auto-release.yml` with an existing tag

**Features:**
- ✅ Nushell-based release preparation (`scripts/release_prepare.nu`)
- ✅ Automatic changelog generation with git-cliff
- ✅ Rich release notes with installation guide and quick-start example
- ✅ Auto-publish to crates.io (when token is set)
- ✅ GitHub/Gitea Release with files attached

### Jobs

#### 1. `test` - Pre-Release Testing
- Runs on: Ubuntu Latest
- Runs all quality checks:
  - Code formatting (`cargo fmt --check`)
  - Clippy linting (`cargo clippy`)
  - All tests (`cargo test --all-features`)
- Purpose: Ensures release quality before publishing

#### 2. `build` - Build and Release
- Runs on: Ubuntu Latest
- Depends on: `test` job passing
- Steps:
  1. **Install git-cliff**: `cargo install git-cliff`
  2. **Install Nushell**: Uses `hustcer/setup-nu@v3`
  3. **Prepare release**: `nu scripts/release_prepare.nu <tag>` — updates `Cargo.toml`, generates `CHANGELOG.md` and `RELEASE_NOTES.md`
  4. **Build release**: `cargo build --release --all-features`
  5. **Create Release**: Uses `softprops/action-gh-release` with `RELEASE_NOTES.md` as the body
  6. **Publish to crates.io**: Auto-publishes if `CRATES_IO_TOKEN` is set

### What `release_prepare.nu` Does

All release-note logic lives in `scripts/release_prepare.nu` rather than inline shell in the workflow YAML. This means you can run it locally to preview release artifacts:

```bash
nu scripts/release_prepare.nu v0.4.0
```

| Step | Action |
|------|--------|
| 1/5 | Updates `version = "..."` in `Cargo.toml` |
| 2/5 | Regenerates `CHANGELOG.md` with git-cliff |
| 3/5 | Generates per-release diff (`CLIFF_CHANGES.md`) |
| 4/5 | Builds `RELEASE_NOTES.md` with changelog, installation guide, and quick-start |
| 5/5 | Cleans up temporary files |

### Release Notes Format

Each release's `RELEASE_NOTES.md` includes:

- **What's New** — git-cliff generated changelog
- **Installation** — `Cargo.toml` dependency snippet and `cargo add tui-slider`
- **Quick Start** — Rust code example with `Slider`, `SliderState`, and `SliderOrientation`

## How to Release

### Automated (Recommended)

```bash
# 1. Run all checks and bump version
just bump 0.4.0

# 2. Review the changes
git show

# 3. Push to both remotes (triggers release workflows)
just push-release-all

# 4. The release workflow handles the rest:
#    - Runs tests
#    - Generates release notes
#    - Creates GitHub/Gitea release
#    - Publishes to crates.io (if CRATES_IO_TOKEN is set)
```

The `just bump` command runs `nu scripts/bump_version.nu` which:
1. Validates the semver format
2. Updates `Cargo.toml`, `README.md` badges, and `Cargo.lock`
3. Runs fmt, clippy, and tests
4. Generates `CHANGELOG.md` with git-cliff
5. Creates a commit and annotated tag

### Pre-flight Check

Before releasing, you can run a full publish-readiness check:

```bash
just check-publish
```

This runs `nu scripts/check_publish.nu` which validates formatting, linting, tests, docs, examples, required files, and a `cargo publish --dry-run`.

## Required Secrets

Nightly repository automation uses each host's built-in workflow credential. No extra GitHub or Gitea PAT is required. The workflows explicitly dispatch their next stage instead of relying on push events generated by that credential.

### CRATES_IO_TOKEN (Optional)

To enable automatic publishing to crates.io:

1. Go to https://crates.io/settings/tokens
2. Create a new token with `publish-update` scope
3. Add it to GitHub:
   - Repository Settings → Secrets and variables → Actions
   - New repository secret
   - Name: `CRATES_IO_TOKEN`
   - Value: Your token

**Note:** If this secret is not set, the workflow will skip crates.io publishing and just create the GitHub/Gitea Release.

## Workflow Files

```
.github/workflows/
├── ci.yml              # Continuous Integration
├── nightly-update.yml  # Library update PR + auto-merge
├── auto-release.yml    # Patch bump after dependency merge
└── release.yml         # Release automation

.gitea/workflows/
├── ci.yml              # Continuous Integration
├── nightly-update.yml  # Library update PR + auto-merge
├── auto-release.yml    # Patch bump after dependency merge
└── release.yml         # Release automation

scripts/
├── bump_version.nu       # Version bump (called by `just bump`)
├── check_publish.nu      # Publish readiness check
├── release_prepare.nu    # Release artifact preparation (called by CI)
├── generate_all_tapes.nu # VHS demo GIF generation
├── upgrade_deps.nu       # Nightly dependency upgrade
├── version.nu            # Print current version
├── setup_gitea.nu        # Set up Gitea remote
├── migrate_to_gitea.nu   # Full Gitea migration
└── README.md             # Script documentation
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

```bash
just fmt
git add -A
git commit -m "style: format code"
```

### CI Failing on Clippy

```bash
just clippy
# Fix issues, then commit
```

### Release Not Publishing to crates.io

Check:
1. Is `CRATES_IO_TOKEN` secret set?
2. Does the token have `publish-update` scope?
3. Check workflow logs for errors

### Release Notes Not Generated

The release workflow uses git-cliff which requires:
1. Conventional commit messages
2. Proper git history (checkout with `fetch-depth: 0`)
3. At least one commit since the last tag

### Testing Release Locally

```bash
# Preview what the release script will produce
nu scripts/release_prepare.nu v0.4.0

# Dry-run publish
just publish-dry
```

## Best Practices

1. **Use conventional commits** for better changelogs:
   ```
   feat: add new feature
   fix: correct bug
   docs: update documentation
   ```

2. **Run checks locally** before pushing:
   ```bash
   just check-all
   ```

3. **Review release notes** on GitHub/Gitea after releasing

4. **Test releases** with `just publish-dry` first

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Nushell](https://www.nushell.sh/)
- [hustcer/setup-nu](https://github.com/hustcer/setup-nu)
- [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache)
- [git-cliff](https://github.com/orhun/git-cliff)
- [softprops/action-gh-release](https://github.com/softprops/action-gh-release)