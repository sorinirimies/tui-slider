# Scripts Directory

This directory contains automation scripts for the tui-slider project, written in [Nushell](https://www.nushell.sh/).

## Prerequisites

```bash
cargo install nu
```

## Available Scripts

### `version.nu`

Prints the current version from `Cargo.toml`.

```bash
nu scripts/version.nu
```

---

### `bump_version.nu`

Automated version bump script that handles the entire release preparation process.

```bash
# Using just (recommended)
just bump 0.4.0

# Direct call
nu scripts/bump_version.nu 0.4.0

# Skip confirmation prompt (CI / non-interactive)
nu scripts/bump_version.nu --yes 0.4.0
```

#### What It Does

| Step | Action |
|------|--------|
| **Validation** | Checks semver format (`X.Y.Z` or `X.Y.Z-suffix`) |
| **Guards** | Rejects if already at target version or if the tag already exists |
| **1/8** | Updates `version = "..."` in `Cargo.toml` |
| **2/8** | Updates version badges in `README.md` |
| **3/8** | Syncs `Cargo.lock` via `cargo update -p tui-slider` |
| **4/8** | Runs `cargo fmt` |
| **5/8** | Runs `cargo clippy --all-targets --all-features -- -D warnings` |
| **6/8** | Runs `cargo test --all-features --all-targets` |
| **7/8** | Generates `CHANGELOG.md` via git-cliff (gracefully skips if not installed) |
| **8/8** | Creates git commit and annotated tag |

#### After Running

```bash
# Review
git show

# Push to both remotes (triggers release workflows)
just push-release-all

# Or push individually
git push origin main && git push origin v0.4.0
git push gitea main && git push gitea v0.4.0
```

---

### `check_publish.nu`

Pre-publish readiness check — run this before pushing a release tag.

```bash
# Using just (recommended)
just check-publish

# Direct call
nu scripts/check_publish.nu
```

#### Checks Performed

| # | Check | Command |
|---|-------|---------|
| 1 | Formatting | `cargo fmt -- --check` |
| 2 | Linting | `cargo clippy --all-targets --all-features -- -D warnings` |
| 3 | Tests | `cargo test --all-features --all-targets` |
| 4 | Documentation | `cargo doc --no-deps --all-features` |
| 5 | Examples | `cargo build --examples` |
| 6 | Required files | `README.md`, `LICENSE`, `Cargo.toml`, `CHANGELOG.md`, `cliff.toml` |
| 7 | Cargo.lock | Verifies lockfile is present |
| 8 | Publish dry-run | `cargo publish --dry-run --allow-dirty` |

---

### `release_prepare.nu`

Prepares release artifacts — extracted from the CI release workflow so it can be run locally or from any CI system.

```bash
nu scripts/release_prepare.nu v0.4.0
```

#### What It Does

| Step | Action |
|------|--------|
| **1/5** | Updates `Cargo.toml` version |
| **2/5** | Regenerates `CHANGELOG.md` with git-cliff |
| **3/5** | Generates per-release diff (`CLIFF_CHANGES.md`) |
| **4/5** | Builds `RELEASE_NOTES.md` with installation guide and quick-start example |
| **5/5** | Cleans up temporary files |

The generated `RELEASE_NOTES.md` is used by both the GitHub and Gitea release workflows as the release body.

---

### `upgrade_deps.nu`

Nightly-style dependency upgrade with a quality gate.

```bash
# Using just (recommended, runs with --dry-run)
just upgrade-deps

# Direct call (will commit and push)
nu scripts/upgrade_deps.nu

# Dry-run mode (validate, then restore Cargo files)
nu scripts/upgrade_deps.nu --dry-run

# Leave validated changes for a CI workflow to commit
nu scripts/upgrade_deps.nu --no-commit

# Custom git identity
nu scripts/upgrade_deps.nu --bot-name "github-actions[bot]" --bot-email "github-actions[bot]@users.noreply.github.com"
```

#### Phases

1. **Phase 1** — `cargo upgrade --incompatible allow --ignore-rust-version` rewrites direct dependency requirements without allowing MSRV filtering to select an older release.
2. **Phase 2** — `cargo update --verbose` refreshes `Cargo.lock`; any reported downgrade aborts and restores both Cargo files.
3. **Quality gate** — fmt → clippy → tests. Failure restores both Cargo files and never commits a partial update.
4. **Output mode**:

| Mode | Successful update behavior |
|------|----------------------------|
| Default | Commit `Cargo.toml`/`Cargo.lock` and push |
| `--dry-run` | Show changed files, then restore them |
| `--no-commit` | Leave validated Cargo changes for CI |
| Any mode, no changes | Exit successfully without a commit |

---

### `generate_all_tapes.nu`

Discovers and generates all VHS demo GIFs.

```bash
# Using just (recommended)
just vhs-all

# Direct call
nu scripts/generate_all_tapes.nu
```

#### What It Does

1. Checks that `vhs` is installed
2. Discovers all `*.tape` files in `examples/vhs/`
3. Runs `vhs` on each tape with progress tracking (`[1/12]`, `[2/12]`, ...)
4. Reports success/failure summary

#### Requirements

```bash
# macOS
brew install vhs

# Other platforms — see https://github.com/charmbracelet/vhs
```

#### Adding New Tapes

1. Create a new `.tape` file in `examples/vhs/`
2. Run `just vhs-all` — the script discovers it automatically

---

### `setup_gitea.nu`

Sets up Gitea as a second remote for dual-hosting (GitHub + Gitea).

```bash
nu scripts/setup_gitea.nu git@gitea.example.com:user/tui-slider.git
```

#### What It Does

- Adds (or updates) the `gitea` remote
- Tests the connection
- Optionally pushes all branches and tags
- Prints quick-reference commands

---

### `migrate_to_gitea.nu`

Full migration script for dual GitHub + Gitea hosting.

```bash
# Interactive (prompts for URL)
nu scripts/migrate_to_gitea.nu

# Non-interactive
nu scripts/migrate_to_gitea.nu --gitea-url git@gitea.example.com:user/tui-slider.git

# Different project directory
nu scripts/migrate_to_gitea.nu --project-dir /path/to/project --gitea-url <url>
```

#### What It Does

- Adds/updates the Gitea remote
- Tests SSH connectivity
- Optionally pushes all history
- Optionally sets up `.gitea/workflows` from `.github/workflows`
- Prints a full summary of available `just` commands

---

## Integration with CI/CD

The `release_prepare.nu` script is called by both GitHub Actions and Gitea Actions release workflows. The workflows install Nushell via [`hustcer/setup-nu@v3`](https://github.com/hustcer/setup-nu) and then call:

```yaml
- name: Install Nushell
  uses: hustcer/setup-nu@v3
  with:
    version: '*'

- name: Prepare release artifacts
  run: nu scripts/release_prepare.nu ${{ github.ref_name }}
```

This keeps the workflow files thin and the logic testable locally.

## Quick Reference

| Task | Command |
|------|---------|
| Show version | `just version` |
| Run all checks | `just check-publish` |
| Bump version | `just bump 0.4.0` |
| Upgrade deps | `just upgrade-deps` |
| Generate all GIFs | `just vhs-all` |
| Full release | `just release-all 0.4.0` |

## Contributing

When adding new scripts:

1. Write them in Nushell (`.nu`)
2. Add a shebang: `#!/usr/bin/env nu`
3. Add a corresponding `just` recipe in the justfile
4. Document the script in this README
5. Use `ansi` colors and the `do { ... } | complete` pattern for error handling
6. Follow the existing style (section headers, step numbering, emoji markers)